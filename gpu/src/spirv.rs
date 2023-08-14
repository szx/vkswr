use crate::shader::VariableBacking as ShaderVariableBacking;
use crate::shader::VariableKind as ShaderVariableKind;
use crate::shader::VariableStorage as ShaderVariableStorage;

use hashbrown::HashMap;
use log::{debug, error};
use rspirv::binary::Disassemble;
use rspirv::dr::{
    Block as Block_, Instruction as Instruction_, Module as Module_, Operand as Operand_, Operand,
};
use rspirv::spirv as spirv_;

#[derive(Debug, Clone)]
pub struct Spirv {
    pub entry_point: EntryPoint,
    pub objects: HashMap<ObjectId, Object>,
    pub functions: HashMap<ObjectId, Function>,
}

impl Spirv {
    pub fn new(name: &str, code: Vec<u32>) -> Option<Self> {
        let mut loader = rspirv::dr::Loader::new();
        assert_eq!(rspirv::spirv::MAGIC_NUMBER, code[0]);
        rspirv::binary::parse_words(&code, &mut loader)
            .map_err(|e| debug!("spriv error: {:#?}\nname: {:?}\ncode: {:?}", e, name, code))
            .ok()?;
        let module = loader.module();
        debug!("spirv shader:\n{}", module.disassemble());

        let entry_point = EntryPoint::parse(&module)?;
        Version::parse(&module)?;
        Capability::parse(&module)?;
        MemoryModel::parse(&module)?;

        let objects = Object::parse(&module)?;
        let functions = Function::parse(&module)?;

        Some(Self {
            entry_point,
            objects,
            functions,
        })
    }

    fn get_type(&self, id: &ObjectId) -> &Type {
        match self.objects.get(id) {
            Some(Object::Type(inner)) => inner,
            _ => unreachable!(),
        }
    }

    fn get_constant(&self, id: &ObjectId) -> &Constant {
        match self.objects.get(id) {
            Some(Object::Constant(inner)) => inner,
            _ => unreachable!(),
        }
    }

    fn get_variable(&self, id: &ObjectId) -> &Variable {
        match self.objects.get(id) {
            Some(Object::Variable(inner)) => inner,
            _ => unreachable!(),
        }
    }

    fn get_shader_var_decl(
        &self,
        type_id: &ObjectId,
        storage: ShaderVariableStorage,
        backing: ShaderVariableBacking,
    ) -> crate::shader::VariableDecl {
        let (kind, component_count, storage, backing) = match self.get_type(type_id) {
            Type::Float { width } => {
                assert_eq!(*width, 32);
                (ShaderVariableKind::F32, 1, storage, backing)
            }
            Type::Int { width, signedness } => {
                assert_eq!(*width, 32);
                if !signedness {
                    (ShaderVariableKind::U32, 1, storage, backing)
                } else {
                    (ShaderVariableKind::I32, 1, storage, backing)
                }
            }
            Type::Void => (ShaderVariableKind::Void, 0, storage, backing),
            Type::Function(_) => {
                unimplemented!()
            }
            Type::Vector {
                component_type,
                component_count,
            } => {
                let component_type = self.get_shader_var_decl(component_type, storage, backing);
                if component_type.component_count == 1 {
                    (
                        component_type.kind,
                        *component_count,
                        component_type.storage,
                        component_type.backing,
                    )
                } else {
                    unimplemented!("Matrix?")
                }
            }
            Type::Struct {
                member_types,
                decorations,
            } => {
                let members = member_types
                    .iter()
                    .map(|MemberType { type_, decorations }| {
                        self.get_shader_var_decl(type_, storage, (*decorations).into())
                    })
                    .collect();
                assert_eq!(decorations.block, true);
                (
                    ShaderVariableKind::Struct,
                    member_types.len() as u32,
                    storage,
                    ShaderVariableBacking::Struct { members },
                )
            }
            Type::Pointer {
                storage_class,
                type_,
            } => {
                let type_ = self.get_shader_var_decl(type_, (*storage_class).into(), backing);
                (
                    ShaderVariableKind::Pointer,
                    1,
                    storage,
                    ShaderVariableBacking::Pointer {
                        kind: Box::new(type_),
                    },
                )
            }
        };

        crate::shader::VariableDecl {
            kind,
            component_count,
            storage,
            backing,
        }
    }

    pub fn shader_instructions(&self) -> Option<Vec<crate::shader::Instruction>> {
        let mut scalar_variables = vec![];
        let mut composite_variables = vec![];
        let mut pointer_variables = vec![];

        for (id, object) in &self.objects {
            match object {
                Object::Type(_) => continue,
                Object::Constant(constant) => match constant {
                    Constant::Scalar { type_, value } => {
                        let decl = self.get_shader_var_decl(
                            type_,
                            ShaderVariableStorage::Constant,
                            ShaderVariableBacking::Memory,
                        );
                        let id = (*id).into();
                        scalar_variables
                            .push(crate::shader::Instruction::VariableDecl { id, decl });
                        scalar_variables.push(crate::shader::Instruction::StoreImm32 {
                            dst: id,
                            imm: *value,
                        });
                    }
                    Constant::Composite {
                        type_,
                        constituents,
                    } => {
                        let decl = self.get_shader_var_decl(
                            type_,
                            ShaderVariableStorage::Constant,
                            ShaderVariableBacking::Memory,
                        );
                        let id = (*id).into();
                        composite_variables
                            .push(crate::shader::Instruction::VariableDecl { id, decl });
                        let values = constituents
                            .iter()
                            .map(|id| match self.get_constant(id) {
                                Constant::Scalar { type_, value } => *value,
                                Constant::Composite { .. } => {
                                    unreachable!()
                                }
                            })
                            .collect::<Vec<_>>();
                        composite_variables.push(crate::shader::Instruction::StoreImm32Array {
                            dst: id,
                            imm: values,
                        });
                    }
                },
                Object::Variable(Variable::Pointer(Pointer::ToMemoryObject { memory_object })) => {
                    let decl = self.get_shader_var_decl(
                        &memory_object.type_,
                        memory_object.storage_class.into(),
                        memory_object.decorations.into(),
                    );
                    let id = (*id).into();
                    pointer_variables.push(crate::shader::Instruction::VariableDecl { id, decl });
                }
            }
        }

        let mut instructions = vec![];
        instructions.extend(scalar_variables);
        instructions.extend(composite_variables);
        instructions.extend(pointer_variables);

        let main = self.functions.get(&self.entry_point.entry_point)?;
        for instruction in &main.instructions {
            match instruction {
                Instruction::Label { result_id } => {
                    instructions.push(crate::shader::Instruction::Label { id: result_id.0 });
                }
                Instruction::AccessChain {
                    result_id,
                    result_type,
                    base,
                    indexes,
                } => {
                    let decl = self.get_shader_var_decl(
                        result_type,
                        ShaderVariableStorage::Variable,
                        ShaderVariableBacking::Memory,
                    );
                    let id = (*result_id).into();
                    if let [offset] = indexes.as_slice() {
                        instructions.push(crate::shader::Instruction::VariableDecl { id, decl });
                        instructions.push(crate::shader::Instruction::LoadVariableOffset {
                            id,
                            base: (*base).into(),
                            offset: (*offset).into(),
                        });
                    } else {
                        unimplemented!()
                    }
                }
                Instruction::Store { pointer, object } => {
                    instructions.push(crate::shader::Instruction::StoreVariable {
                        dst_pointer: (*pointer).into(),
                        src: (*object).into(),
                    });
                }
                Instruction::Load {
                    result_id,
                    result_type,
                    pointer,
                } => {
                    let decl = self.get_shader_var_decl(
                        result_type,
                        ShaderVariableStorage::Variable,
                        ShaderVariableBacking::Memory,
                    );
                    let id = (*result_id).into();
                    instructions.push(crate::shader::Instruction::VariableDecl { id, decl });
                    instructions.push(crate::shader::Instruction::LoadVariable {
                        id,
                        src_pointer: (*pointer).into(),
                    });
                }
                Instruction::VectorTimesScalar {
                    result_id,
                    result_type,
                    vector,
                    scalar,
                } => {
                    let decl = self.get_shader_var_decl(
                        result_type,
                        ShaderVariableStorage::Variable,
                        ShaderVariableBacking::Memory,
                    );
                    let id = (*result_id).into();
                    instructions.push(crate::shader::Instruction::VariableDecl { id, decl });
                    instructions.push(crate::shader::Instruction::MathMulVectorScalar {
                        id,
                        vector: (*vector).into(),
                        scalar: (*scalar).into(),
                    });
                }
                Instruction::FSub {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = self.get_shader_var_decl(
                        result_type,
                        ShaderVariableStorage::Variable,
                        ShaderVariableBacking::Memory,
                    );
                    let id = (*result_id).into();
                    instructions.push(crate::shader::Instruction::VariableDecl { id, decl });
                    instructions.push(crate::shader::Instruction::MathSubF32F32 {
                        id,
                        op1: (*operand1).into(),
                        op2: (*operand2).into(),
                    });
                }
                Instruction::CompositeExtract {
                    result_id,
                    result_type,
                    composite,
                    indexes,
                } => {
                    let decl = self.get_shader_var_decl(
                        result_type,
                        ShaderVariableStorage::Variable,
                        ShaderVariableBacking::Memory,
                    );
                    let id = (*result_id).into();
                    if let [offset] = indexes.as_slice() {
                        instructions.push(crate::shader::Instruction::VariableDecl { id, decl });
                        instructions.push(crate::shader::Instruction::LoadVariableImmOffset {
                            id,
                            base: (*composite).into(),
                            offset: *offset,
                        });
                    } else {
                        unimplemented!()
                    }
                }
                Instruction::CompositeConstruct {
                    result_id,
                    result_type,
                    constituents,
                } => {
                    let decl = self.get_shader_var_decl(
                        result_type,
                        ShaderVariableStorage::Variable,
                        ShaderVariableBacking::Memory,
                    );
                    let id = (*result_id).into();
                    instructions.push(crate::shader::Instruction::VariableDecl { id, decl });
                    let values = constituents
                        .iter()
                        .map(|id| (*id).into())
                        .collect::<Vec<_>>();
                    instructions
                        .push(crate::shader::Instruction::StoreVariableArray { dst: id, values });
                }
                Instruction::Return => {
                    instructions.push(crate::shader::Instruction::Return);
                }
            }
        }
        Some(instructions)
    }
}

#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub entry_point: ObjectId,
    pub interfaces: Vec<ObjectId>,
}

impl EntryPoint {
    /// Parses OpEntryPoint.
    pub fn parse(module: &Module_) -> Option<Self> {
        let entry_point = module.entry_points.first()?;
        match &entry_point.operands[..] {
            [Operand_::ExecutionModel(spirv_::ExecutionModel::Vertex), Operand_::IdRef(entry_point), _name, interfaces @ ..] =>
            {
                let interfaces = interfaces
                    .iter()
                    .map(|x| ObjectId(x.unwrap_id_ref()))
                    .collect::<Vec<_>>();
                Some(Self {
                    entry_point: ObjectId(*entry_point),
                    interfaces,
                })
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Version {
    major: u8,
    minor: u8,
}

impl Version {
    /// Returns supported SPIR-V version..
    pub fn parse(module: &Module_) -> Option<Self> {
        let (major, minor) = module.header.as_ref()?.version();
        Some(Self { major, minor })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Capability {}

impl Capability {
    /// Parses OpCapability.
    pub fn parse(module: &Module_) -> Option<Self> {
        let capability = module.capabilities.first()?;
        match &capability.operands[..] {
            [Operand_::Capability(spirv_::Capability::Shader)] => Some(Self {}),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MemoryModel {}

impl MemoryModel {
    /// Parses OpMemoryModel.
    pub fn parse(module: &Module_) -> Option<Self> {
        let memory_model = &module.memory_model.as_ref()?;
        match &memory_model.operands[..] {
            [Operand_::AddressingModel(spirv_::AddressingModel::Logical), Operand_::MemoryModel(spirv_::MemoryModel::GLSL450)] => {
                Some(Self {})
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Type(Type),
    Constant(Constant),
    Variable(Variable),
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct ObjectId(pub u32);

impl From<ObjectId> for crate::shader::VariableId {
    fn from(value: ObjectId) -> Self {
        Self(value.0)
    }
}

impl Object {
    /// Parses global types, constants and variables.
    pub fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut types = Type::parse(&module)?;
        let mut constants = Constant::parse(&module)?;
        let mut variables = Variable::parse(&module)?;
        Decorations::parse(&module, &mut types, &mut constants, &mut variables);

        let mut data = HashMap::default();
        data.extend(types.iter().map(|(id, x)| (*id, Self::Type(x.clone()))));
        data.extend(
            constants
                .iter()
                .map(|(id, x)| (*id, Self::Constant(x.clone()))),
        );
        data.extend(
            variables
                .iter()
                .map(|(id, x)| (*id, Self::Variable(x.clone()))),
        );
        Some(data)
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Function(Vec<ObjectId>),
    Float {
        width: u32,
    },
    Int {
        width: u32,
        signedness: bool,
    },
    Vector {
        component_type: ObjectId,
        component_count: u32,
    },
    Struct {
        member_types: Vec<MemberType>,
        decorations: Decorations,
    },
    Pointer {
        storage_class: StorageClass,
        type_: ObjectId,
    },
}

#[derive(Debug, Clone)]
pub struct MemberType {
    pub type_: ObjectId,
    pub decorations: Decorations,
}

impl Type {
    /// Parses types.
    pub fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut data = HashMap::default();

        for inst in &module.types_global_values {
            let (opcode, result_type, Some(result_id), operands) = deconstruct_instruction(inst)
            else {
                return None;
            };
            match (opcode, result_type, result_id, operands) {
                (spirv_::Op::TypeVoid, None, &result_id, &[]) => {
                    data.insert(ObjectId(result_id), Type::Void);
                }
                (spirv_::Op::TypeFunction, None, &result_id, operands) => {
                    data.insert(
                        ObjectId(result_id),
                        Type::Function(
                            operands
                                .iter()
                                .map(|x| ObjectId(x.unwrap_id_ref()))
                                .collect(),
                        ),
                    );
                }
                (spirv_::Op::TypeFloat, None, &result_id, &[Operand_::LiteralInt32(width)]) => {
                    data.insert(ObjectId(result_id), Type::Float { width });
                }
                (
                    spirv_::Op::TypeInt,
                    None,
                    &result_id,
                    &[Operand_::LiteralInt32(width), Operand_::LiteralInt32(signedness)],
                ) => {
                    data.insert(
                        ObjectId(result_id),
                        Type::Int {
                            width,
                            signedness: signedness == 1,
                        },
                    );
                }
                (
                    spirv_::Op::TypeVector,
                    None,
                    &result_id,
                    &[Operand_::IdRef(component_type), Operand_::LiteralInt32(component_count)],
                ) => {
                    data.insert(
                        ObjectId(result_id),
                        Type::Vector {
                            component_type: ObjectId(component_type),
                            component_count,
                        },
                    );
                }
                (spirv_::Op::TypeStruct, None, &result_id, operands) => {
                    data.insert(
                        ObjectId(result_id),
                        Type::Struct {
                            member_types: operands
                                .iter()
                                .map(|x| MemberType {
                                    type_: ObjectId(x.unwrap_id_ref()),
                                    decorations: Default::default(),
                                })
                                .collect(),
                            decorations: Default::default(),
                        },
                    );
                }
                (
                    spirv_::Op::TypePointer,
                    None,
                    &result_id,
                    &[Operand_::StorageClass(storage_class), Operand_::IdRef(type_)],
                ) => {
                    data.insert(
                        ObjectId(result_id),
                        Type::Pointer {
                            storage_class: storage_class.into(),
                            type_: ObjectId(type_),
                        },
                    );
                }
                (spirv_::Op::Variable, _, _, _) => continue,
                (spirv_::Op::Constant, _, _, _) => continue,
                (spirv_::Op::ConstantComposite, _, _, _) => continue,
                _ => {
                    unimplemented!("{:#?}\n{:#?}", inst, data)
                }
            }
        }
        Some(data)
    }

    fn decorate_member(
        data: &mut HashMap<ObjectId, Self>,
        target: &spirv_::Word,
        member: &u32,
        decoration: &spirv_::Decoration,
        literals: &[Operand_],
    ) {
        let member_types = match data.get_mut(&ObjectId(*target)) {
            Some(Type::Struct {
                member_types,
                decorations: _,
            }) => member_types,
            _ => unreachable!(),
        };
        let MemberType {
            type_: _,
            decorations,
        } = member_types.get_mut(*member as usize).unwrap();
        decorations.decorate(decoration, literals);
    }

    pub fn decorate(
        data: &mut HashMap<ObjectId, Self>,
        target: &spirv_::Word,
        decoration: &spirv_::Decoration,
        literals: &[Operand_],
    ) -> bool {
        let type_ = data.get_mut(&ObjectId(*target));
        let decorations = match type_ {
            Some(Type::Struct {
                member_types: _,
                decorations,
            }) => decorations,
            None => return false,
            _ => unreachable!("{:?}, {:?}", target, type_),
        };
        decorations.decorate(decoration, literals);
        true
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Scalar {
        type_: ObjectId,
        value: u32,
    },
    Composite {
        type_: ObjectId,
        constituents: Vec<ObjectId>,
    },
}

impl Constant {
    /// Parses constants.
    pub fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut data = HashMap::default();

        for inst in &module.types_global_values {
            let (opcode, result_type, Some(result_id), operands) = deconstruct_instruction(inst)
            else {
                return None;
            };
            match (opcode, result_type, result_id, operands) {
                (
                    spirv_::Op::TypeVoid
                    | spirv_::Op::TypeFunction
                    | spirv_::Op::TypeFloat
                    | spirv_::Op::TypeInt
                    | spirv_::Op::TypeVector
                    | spirv_::Op::TypeStruct
                    | spirv_::Op::TypePointer
                    | spirv_::Op::Variable,
                    _,
                    _,
                    _,
                ) => continue,
                (
                    spirv_::Op::Constant,
                    &Some(result_type),
                    &result_id,
                    &[Operand_::LiteralInt32(value)],
                ) => {
                    data.insert(
                        ObjectId(result_id),
                        Constant::Scalar {
                            type_: ObjectId(result_type),
                            value,
                        },
                    );
                }
                (
                    spirv_::Op::Constant,
                    &Some(result_type),
                    &result_id,
                    &[Operand_::LiteralFloat32(value)],
                ) => {
                    data.insert(
                        ObjectId(result_id),
                        Constant::Scalar {
                            type_: ObjectId(result_type),
                            value: value.to_bits(),
                        },
                    );
                }
                (spirv_::Op::ConstantComposite, &Some(result_type), &result_id, operands) => {
                    data.insert(
                        ObjectId(result_id),
                        Constant::Composite {
                            type_: ObjectId(result_type),
                            constituents: operands
                                .iter()
                                .map(|x| ObjectId(x.unwrap_id_ref()))
                                .collect(),
                        },
                    );
                }
                _ => {
                    unimplemented!("{:#?}\n{:#?}", inst, data)
                }
            }
        }
        Some(data)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StorageClass {
    Input,
    Output,
}

impl From<spirv_::StorageClass> for StorageClass {
    fn from(value: spirv_::StorageClass) -> Self {
        match value {
            spirv_::StorageClass::Input => Self::Input,
            spirv_::StorageClass::Output => Self::Output,
            _ => unimplemented!(),
        }
    }
}

impl From<StorageClass> for ShaderVariableStorage {
    fn from(value: StorageClass) -> Self {
        match value {
            StorageClass::Input => Self::Input,
            StorageClass::Output => Self::Output,
        }
    }
}

impl Variable {
    /// Parses variables.
    pub fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut data = HashMap::default();

        for inst in &module.types_global_values {
            let (opcode, result_type, Some(result_id), operands) = deconstruct_instruction(inst)
            else {
                return None;
            };
            match (opcode, result_type, result_id, operands) {
                (
                    spirv_::Op::TypeVoid
                    | spirv_::Op::TypeFunction
                    | spirv_::Op::TypeFloat
                    | spirv_::Op::TypeInt
                    | spirv_::Op::TypeVector
                    | spirv_::Op::TypeStruct
                    | spirv_::Op::TypePointer
                    | spirv_::Op::Constant
                    | spirv_::Op::ConstantComposite,
                    _,
                    _,
                    _,
                ) => continue,
                (
                    spirv_::Op::Variable,
                    &Some(pointer_type),
                    &result_id,
                    &[Operand_::StorageClass(storage_class)],
                ) => {
                    data.insert(
                        ObjectId(result_id),
                        Variable::Pointer(Pointer::ToMemoryObject {
                            memory_object: MemoryObject {
                                type_: ObjectId(pointer_type),
                                storage_class: storage_class.into(),
                                decorations: Default::default(),
                            },
                        }),
                    );
                }
                _ => {
                    unimplemented!("{:#?}\n{:#?}", inst, data)
                }
            }
        }
        Some(data)
    }

    pub fn decorate(
        data: &mut HashMap<ObjectId, Self>,
        target: &spirv_::Word,
        decoration: &spirv_::Decoration,
        literals: &[Operand_],
    ) -> bool {
        let type_ = data.get_mut(&ObjectId(*target));
        let decorations = match type_ {
            Some(Variable::Pointer(Pointer::ToMemoryObject {
                memory_object:
                    MemoryObject {
                        type_: _,
                        storage_class: _,
                        decorations,
                    },
            })) => decorations,
            Some(Variable::Pointer(_)) => return false,
            None => return false,
        };
        decorations.decorate(decoration, literals);
        true
    }
}

#[derive(Debug, Clone)]
pub enum Variable {
    Pointer(Pointer),
}

#[derive(Debug, Clone)]
pub enum Pointer {
    ToMemoryObject { memory_object: MemoryObject },
}

#[derive(Debug, Clone)]
pub struct MemoryObject {
    pub type_: ObjectId,
    pub storage_class: StorageClass,
    pub decorations: Decorations,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Decorations {
    pub builtin: Option<BuiltInDecoration>,
    pub block: bool,
    pub location: Option<LocationDecoration>,
}

impl From<Decorations> for ShaderVariableBacking {
    fn from(decorations: Decorations) -> Self {
        if let Some(builtin) = decorations.builtin {
            match builtin {
                BuiltInDecoration::Position => Self::Position,
                BuiltInDecoration::PointSize => Self::PointSize,
            }
        } else if let Some(location) = decorations.location {
            Self::Location {
                number: location.number,
            }
        } else {
            Self::Memory
        }
    }
}

impl Decorations {
    fn decorate(&mut self, value: &spirv_::Decoration, literals: &[Operand_]) {
        match (value, literals) {
            (spirv_::Decoration::BuiltIn, [literal]) => {
                assert!(self.builtin.is_none());
                self.builtin = Some(BuiltInDecoration::new(literal));
            }
            (spirv_::Decoration::Block, _) => self.block = true,
            (spirv_::Decoration::Location, [location]) => {
                self.location = Some(LocationDecoration {
                    number: location.unwrap_literal_int32(),
                })
            }
            _ => unimplemented!("{:?}, {:?}", value, literals),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BuiltInDecoration {
    Position,
    PointSize,
}

impl BuiltInDecoration {
    pub(crate) fn new(operand: &Operand) -> Self {
        match operand {
            Operand_::BuiltIn(spirv_::BuiltIn::Position) => Self::Position,
            Operand_::BuiltIn(spirv_::BuiltIn::PointSize) => Self::PointSize,
            _ => unimplemented!("{operand:?}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LocationDecoration {
    pub number: u32,
}

impl Decorations {
    /// Parses and apply decorations.
    pub fn parse(
        module: &Module_,
        types: &mut HashMap<ObjectId, Type>,
        constants: &mut HashMap<ObjectId, Constant>,
        variables: &mut HashMap<ObjectId, Variable>,
    ) {
        for inst in &module.annotations {
            let (opcode, None, None, operands) = deconstruct_instruction(inst) else {
                unreachable!();
            };
            match (opcode, operands) {
                (
                    spirv_::Op::MemberDecorate,
                    [Operand_::IdRef(target), Operand_::LiteralInt32(member), Operand_::Decoration(decoration), literals @ ..],
                ) => {
                    Type::decorate_member(types, target, member, decoration, literals);
                }
                (
                    spirv_::Op::Decorate,
                    [Operand_::IdRef(target), Operand_::Decoration(decoration), literals @ ..],
                ) => {
                    if Type::decorate(types, target, decoration, literals)
                        || Variable::decorate(variables, target, decoration, literals)
                    {
                        continue;
                    } else {
                        unreachable!("{:#?}", inst)
                    }
                }
                _ => {
                    unimplemented!("{:#?}", inst)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub result_type: ObjectId,
    pub function_type: ObjectId,
    pub instructions: Vec<Instruction>,
}

impl Function {
    /// Parses functions.
    pub fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut data = HashMap::default();
        for function in &module.functions {
            let builder = FunctionBuilder::new();

            let mut builder = if let (
                spirv_::Op::Function,
                &Some(result_type),
                &Some(result_id),
                &[Operand_::FunctionControl(function_control), Operand_::IdRef(function_type)],
            ) = deconstruct_instruction(function.def.as_ref().unwrap())
            {
                builder.def(result_id, result_type, function_control, function_type)
            } else {
                return None;
            };
            assert!(function.parameters.is_empty()); // TODO: Parse SPIR-V function parameters.
            for block in &function.blocks {
                builder = builder.block(block);
            }
            let (result_id, function) = builder.build();
            data.insert(result_id, function);
        }
        Some(data)
    }
}

#[derive(Debug)]
struct FunctionBuilder {
    result_id: spirv_::Word,
    result_type: spirv_::Word,
    function_control: spirv_::FunctionControl,
    function_type: spirv_::Word,
    instructions: Vec<Instruction>,
}

impl FunctionBuilder {
    fn new() -> Self {
        Self {
            result_id: 0,
            result_type: 0,
            function_control: spirv_::FunctionControl::NONE,
            function_type: 0,
            instructions: Default::default(),
        }
    }

    const fn def(
        mut self,
        result_id: spirv_::Word,
        result_type: spirv_::Word,
        function_control: spirv_::FunctionControl,
        function_type: spirv_::Word,
    ) -> Self {
        self.result_id = result_id;
        self.result_type = result_type;
        self.function_control = function_control;
        self.function_type = function_type;
        self
    }

    fn block(mut self, block: &Block_) -> Self {
        self = self.instruction(block.label.as_ref().unwrap());
        for inst in &block.instructions {
            self = self.instruction(inst);
        }
        self
    }

    fn instruction(mut self, instruction: &Instruction_) -> Self {
        self.instructions
            .push(Instruction::parse(instruction).unwrap());
        self
    }

    fn build(self) -> (ObjectId, Function) {
        (
            ObjectId(self.result_id),
            Function {
                result_type: ObjectId(self.result_type),
                function_type: ObjectId(self.function_type),
                instructions: self.instructions,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Label {
        result_id: ObjectId,
    },
    AccessChain {
        result_id: ObjectId,
        result_type: ObjectId,
        base: ObjectId,
        indexes: Vec<ObjectId>,
    },
    Store {
        pointer: ObjectId,
        object: ObjectId,
    },
    Load {
        result_id: ObjectId,
        result_type: ObjectId,
        pointer: ObjectId,
    },
    VectorTimesScalar {
        result_id: ObjectId,
        result_type: ObjectId,
        vector: ObjectId,
        scalar: ObjectId,
    },
    FSub {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    CompositeExtract {
        result_id: ObjectId,
        result_type: ObjectId,
        composite: ObjectId,
        indexes: Vec<u32>,
    },
    CompositeConstruct {
        result_id: ObjectId,
        result_type: ObjectId,
        constituents: Vec<ObjectId>,
    },
    Return,
}

impl Instruction {
    /// Parses instruction.
    pub fn parse(instruction: &Instruction_) -> Option<Self> {
        let (opcode, result_type, result_id, operands) = deconstruct_instruction(instruction);
        match (opcode, result_type, result_id, operands) {
            (spirv_::Op::Label, None, &Some(result_id), &[]) => Some(Self::Label {
                result_id: ObjectId(result_id),
            }),
            (
                spirv_::Op::AccessChain,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(base), indexes @ ..],
            ) => Some(Self::AccessChain {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                base: ObjectId(*base),
                indexes: indexes
                    .iter()
                    .map(|x| ObjectId(x.unwrap_id_ref()))
                    .collect::<Vec<_>>(),
            }),
            (
                spirv_::Op::Store,
                None,
                None,
                [Operand_::IdRef(pointer), Operand_::IdRef(object), memory_operands @ ..],
            ) => {
                assert_eq!(memory_operands.len(), 0);
                Some(Self::Store {
                    pointer: ObjectId(*pointer),
                    object: ObjectId(*object),
                })
            }
            (
                spirv_::Op::Load,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(pointer), memory_operands @ ..],
            ) => {
                assert_eq!(memory_operands.len(), 0);
                Some(Self::Load {
                    result_id: ObjectId(result_id),
                    result_type: ObjectId(result_type),
                    pointer: ObjectId(*pointer),
                })
            }
            (
                spirv_::Op::VectorTimesScalar,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(vector), Operand_::IdRef(scalar)],
            ) => Some(Self::VectorTimesScalar {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                vector: ObjectId(*vector),
                scalar: ObjectId(*scalar),
            }),
            (
                spirv_::Op::FSub,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::FSub {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::CompositeExtract,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(composite), indexes @ ..],
            ) => Some(Self::CompositeExtract {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                composite: ObjectId(*composite),
                indexes: indexes
                    .iter()
                    .map(|x| x.unwrap_literal_int32())
                    .collect::<Vec<_>>(),
            }),
            (
                spirv_::Op::CompositeConstruct,
                &Some(result_type),
                &Some(result_id),
                [constituents @ ..],
            ) => Some(Self::CompositeConstruct {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                constituents: constituents
                    .iter()
                    .map(|x| ObjectId(x.unwrap_id_ref()))
                    .collect::<Vec<_>>(),
            }),
            (spirv_::Op::Return, None, None, &[]) => Some(Self::Return),
            _ => {
                error!("{instruction:#?}");
                None
            }
        }
    }
}

fn deconstruct_instruction(
    instruction: &Instruction_,
) -> (
    &spirv_::Op,
    &Option<spirv_::Word>,
    &Option<spirv_::Word>,
    &[Operand_],
) {
    let Instruction_ {
        class:
            rspirv::grammar::Instruction {
                opname,
                opcode,
                capabilities,
                extensions,
                operands: grammar_operands,
            },
        result_type,
        result_id,
        operands,
    } = instruction;
    (opcode, result_type, result_id, operands.as_slice())
}
