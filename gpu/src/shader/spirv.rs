use hashbrown::HashMap;
use log::{debug, error};
use rspirv::binary::Disassemble;
use rspirv::dr::{
    Block as Block_, Instruction as Instruction_, Module as Module_, Operand as Operand_, Operand,
};
use rspirv::spirv as spirv_;

#[derive(Debug, Clone)]
pub(crate) struct Spirv {
    pub(crate) entry_point: EntryPoint,
    pub(crate) objects: HashMap<ObjectId, Object>,
    pub(crate) functions: HashMap<ObjectId, Function>,
}

impl Spirv {
    pub(crate) fn new(name: &str, code: Vec<u32>) -> Option<Self> {
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
}

#[derive(Debug, Clone)]
pub(crate) struct EntryPoint {
    pub(crate) entry_point: ObjectId,
    pub(crate) interfaces: Vec<ObjectId>,
}

impl EntryPoint {
    /// Parses OpEntryPoint.
    fn parse(module: &Module_) -> Option<Self> {
        let entry_point = module.entry_points.first()?;
        match &entry_point.operands[..] {
            [Operand_::ExecutionModel(
                spirv_::ExecutionModel::Vertex | spirv_::ExecutionModel::Fragment,
            ), Operand_::IdRef(entry_point), _name, interfaces @ ..] => {
                let interfaces = interfaces
                    .iter()
                    .map(|x| ObjectId(x.unwrap_id_ref()))
                    .collect::<Vec<_>>();
                Some(Self {
                    entry_point: ObjectId(*entry_point),
                    interfaces,
                })
            }
            invalid => {
                debug!("spriv error: invalid OpEntryPoint {:#?}", invalid);
                None
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Version {
    major: u8,
    minor: u8,
}

impl Version {
    /// Returns supported SPIR-V version..
    fn parse(module: &Module_) -> Option<Self> {
        let (major, minor) = module.header.as_ref()?.version();
        Some(Self { major, minor })
    }
}

#[derive(Debug, Clone, Default)]
struct Capability {}

impl Capability {
    /// Parses OpCapability.
    fn parse(module: &Module_) -> Option<Self> {
        let capability = module.capabilities.first()?;
        match &capability.operands[..] {
            [Operand_::Capability(spirv_::Capability::Shader)] => Some(Self {}),
            invalid => {
                debug!("spriv error: invalid OpCapability {:#?}", invalid);
                None
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct MemoryModel {}

impl MemoryModel {
    /// Parses OpMemoryModel.
    fn parse(module: &Module_) -> Option<Self> {
        let memory_model = &module.memory_model.as_ref()?;
        match &memory_model.operands[..] {
            [Operand_::AddressingModel(spirv_::AddressingModel::Logical), Operand_::MemoryModel(spirv_::MemoryModel::GLSL450)] => {
                Some(Self {})
            }
            invalid => {
                debug!("spriv error: invalid OpMemoryModel {:#?}", invalid);
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Object {
    Type(Type),
    Constant(Constant),
    Variable(Variable),
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub(crate) struct ObjectId(pub u32);

impl Object {
    /// Parses global types, constants and variables.
    fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut types = Type::parse(module)?;
        let mut constants = Constant::parse(module)?;
        let mut variables = Variable::parse(module)?;
        Decorations::parse(module, &mut types, &mut constants, &mut variables);

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
pub(crate) enum Type {
    Void,
    Bool,
    Function(Vec<ObjectId>),
    Float {
        width: u32,
    },
    Int {
        width: u32,
        signedness: bool,
    },
    Array {
        element_type: ObjectId,
        length: ObjectId,
        decorations: Decorations,
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
pub(crate) struct MemberType {
    pub(crate) type_: ObjectId,
    pub(crate) decorations: Decorations,
}

impl Type {
    /// Parses types.
    fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
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
                (spirv_::Op::TypeBool, None, &result_id, &[]) => {
                    data.insert(ObjectId(result_id), Type::Bool);
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
                    spirv_::Op::TypeArray,
                    None,
                    &result_id,
                    &[Operand_::IdRef(element_type), Operand_::IdRef(length)],
                ) => {
                    data.insert(
                        ObjectId(result_id),
                        Type::Array {
                            element_type: ObjectId(element_type),
                            length: ObjectId(length),
                            decorations: Default::default(),
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
            Some(Type::Array {
                element_type: _,
                length: _,
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
pub(crate) enum Constant {
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
    fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut data = HashMap::default();

        for inst in &module.types_global_values {
            let (opcode, result_type, Some(result_id), operands) = deconstruct_instruction(inst)
            else {
                return None;
            };
            match (opcode, result_type, result_id, operands) {
                (
                    spirv_::Op::TypeVoid
                    | spirv_::Op::TypeBool
                    | spirv_::Op::TypeFunction
                    | spirv_::Op::TypeFloat
                    | spirv_::Op::TypeInt
                    | spirv_::Op::TypeArray
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
pub(crate) enum StorageClass {
    Input,
    Output,
    Function,
    PushConstant,
    Uniform,
}

impl From<spirv_::StorageClass> for StorageClass {
    fn from(value: spirv_::StorageClass) -> Self {
        match value {
            spirv_::StorageClass::Input => Self::Input,
            spirv_::StorageClass::Output => Self::Output,
            spirv_::StorageClass::Function => Self::Function,
            spirv_::StorageClass::PushConstant => Self::PushConstant,
            spirv_::StorageClass::Uniform => Self::Uniform,
            invalid => {
                unimplemented!("{:#?}", invalid)
            }
        }
    }
}

impl Variable {
    /// Parses variables.
    fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
        let mut data = HashMap::default();

        for inst in &module.types_global_values {
            let (opcode, result_type, Some(result_id), operands) = deconstruct_instruction(inst)
            else {
                return None;
            };
            match (opcode, result_type, result_id, operands) {
                (
                    spirv_::Op::TypeVoid
                    | spirv_::Op::TypeBool
                    | spirv_::Op::TypeFunction
                    | spirv_::Op::TypeFloat
                    | spirv_::Op::TypeInt
                    | spirv_::Op::TypeArray
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

    fn decorate(
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
pub(crate) enum Variable {
    Pointer(Pointer),
}

#[derive(Debug, Clone)]
pub(crate) enum Pointer {
    ToMemoryObject { memory_object: MemoryObject },
}

#[derive(Debug, Clone)]
pub(crate) struct MemoryObject {
    pub(crate) type_: ObjectId,
    pub(crate) storage_class: StorageClass,
    pub(crate) decorations: Decorations,
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct Decorations {
    pub(crate) builtin: Option<BuiltInDecoration>,
    pub(crate) block: bool,
    pub(crate) location: Option<LocationDecoration>,
    pub(crate) relaxed_precision: bool, // TODO: Implement RelaxedPrecision decoration.
    pub(crate) byte_offset: Option<u32>, // TODO: Implement Offset decoration for array member type.
    pub(crate) array_stride: Option<u32>, // TODO: Implement Offset decoration for array type.
    pub(crate) descriptor_set: Option<u32>, // TODO: Implement DescriptorSet decoration for variable type.
    pub(crate) binding_point: Option<u32>,  // TODO: Implement Binding decoration for variable type.
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
            (spirv_::Decoration::RelaxedPrecision, &[]) => self.relaxed_precision = true,
            (spirv_::Decoration::Offset, &[Operand_::LiteralInt32(byte_offset)]) => {
                self.byte_offset = Some(byte_offset)
            }
            (spirv_::Decoration::ArrayStride, &[Operand_::LiteralInt32(array_stride)]) => {
                self.array_stride = Some(array_stride)
            }
            (spirv_::Decoration::DescriptorSet, &[Operand_::LiteralInt32(descriptor_set)]) => {
                self.descriptor_set = Some(descriptor_set)
            }
            (spirv_::Decoration::Binding, &[Operand_::LiteralInt32(binding_point)]) => {
                self.binding_point = Some(binding_point)
            }
            _ => unimplemented!("{:?}, {:?}", value, literals),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum BuiltInDecoration {
    Position,
    PointSize,
    VertexIndex,
    FragCoord,
    ClipDistance,
    CullDistance,
}

impl BuiltInDecoration {
    pub(crate) fn new(operand: &Operand) -> Self {
        match operand {
            Operand_::BuiltIn(spirv_::BuiltIn::Position) => Self::Position,
            Operand_::BuiltIn(spirv_::BuiltIn::PointSize) => Self::PointSize,
            Operand_::BuiltIn(spirv_::BuiltIn::VertexIndex) => Self::VertexIndex,
            Operand_::BuiltIn(spirv_::BuiltIn::FragCoord) => Self::FragCoord,
            Operand_::BuiltIn(spirv_::BuiltIn::ClipDistance) => Self::ClipDistance,
            Operand_::BuiltIn(spirv_::BuiltIn::CullDistance) => Self::CullDistance,
            _ => unimplemented!("{operand:?}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct LocationDecoration {
    pub number: u32,
}

impl Decorations {
    /// Parses and apply decorations.
    fn parse(
        module: &Module_,
        types: &mut HashMap<ObjectId, Type>,
        _constants: &mut HashMap<ObjectId, Constant>,
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
pub(crate) struct Function {
    pub(crate) result_type: ObjectId,
    pub(crate) function_type: ObjectId,
    pub(crate) instructions: Vec<Instruction>,
}

impl Function {
    /// Parses functions.
    fn parse(module: &Module_) -> Option<HashMap<ObjectId, Self>> {
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
pub(crate) enum Instruction {
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
    IAdd {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    IMul {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    FSub {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    FDiv {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    SDiv {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    UDiv {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    SMod {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    UMod {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },

    BitwiseAnd {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    ShiftRightLogical {
        result_id: ObjectId,
        result_type: ObjectId,
        base: ObjectId,
        shift: ObjectId,
    },

    IEqual {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    ULessThan {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
    },
    ConvertSToF {
        result_id: ObjectId,
        result_type: ObjectId,
        operand: ObjectId,
    },
    ConvertFToU {
        result_id: ObjectId,
        result_type: ObjectId,
        operand: ObjectId,
    },
    ConvertUToF {
        result_id: ObjectId,
        result_type: ObjectId,
        operand: ObjectId,
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
    Variable {
        result_id: ObjectId,
        result_type: ObjectId,
        storage_class: StorageClass,
    },
    Select {
        result_id: ObjectId,
        result_type: ObjectId,
        condition: ObjectId,
        object1: ObjectId,
        object2: ObjectId,
    },
    SelectionMerge {
        merge_block: ObjectId,
    },
    LoopMerge {
        merge_block: ObjectId,
        continue_target_label: ObjectId,
    },
    Branch {
        target_label: ObjectId,
    },
    BranchConditional {
        condition: ObjectId,
        true_label: ObjectId,
        false_label: ObjectId,
    },
    Return,
    Kill,
}

impl Instruction {
    /// Parses instruction.
    fn parse(instruction: &Instruction_) -> Option<Self> {
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
                spirv_::Op::IAdd,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::IAdd {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::IMul,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::IMul {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
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
                spirv_::Op::FDiv,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::FDiv {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::SDiv,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::SDiv {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::UDiv,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::UDiv {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::SMod,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::SMod {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::UMod,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::UMod {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::BitwiseAnd,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::BitwiseAnd {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::ShiftRightLogical,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(base), Operand_::IdRef(shift)],
            ) => Some(Self::ShiftRightLogical {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                base: ObjectId(*base),
                shift: ObjectId(*shift),
            }),
            (
                spirv_::Op::IEqual,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::IEqual {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::ULessThan,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand1), Operand_::IdRef(operand2)],
            ) => Some(Self::ULessThan {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
            }),
            (
                spirv_::Op::ConvertSToF,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand)],
            ) => Some(Self::ConvertSToF {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand: ObjectId(*operand),
            }),
            (
                spirv_::Op::ConvertFToU,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand)],
            ) => Some(Self::ConvertFToU {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand: ObjectId(*operand),
            }),
            (
                spirv_::Op::ConvertUToF,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(operand)],
            ) => Some(Self::ConvertUToF {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand: ObjectId(*operand),
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
            (
                spirv_::Op::Variable,
                &Some(result_type),
                &Some(result_id),
                &[Operand_::StorageClass(storage_class)],
            ) => Some(Self::Variable {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                storage_class: storage_class.into(),
            }),
            (
                spirv_::Op::Select,
                &Some(result_type),
                &Some(result_id),
                [Operand_::IdRef(condition), Operand_::IdRef(object1), Operand_::IdRef(object2)],
            ) => Some(Self::Select {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                condition: ObjectId(*condition),
                object1: ObjectId(*object1),
                object2: ObjectId(*object2),
            }),
            (
                spirv_::Op::SelectionMerge,
                None,
                None,
                [Operand_::IdRef(merge_block), Operand_::SelectionControl(_selection_control)],
            ) => Some(Self::SelectionMerge {
                merge_block: ObjectId(*merge_block),
            }),
            (
                spirv_::Op::LoopMerge,
                None,
                None,
                [Operand_::IdRef(merge_block), Operand_::IdRef(continue_target_label), Operand_::LoopControl(_loop_control), ..],
            ) => Some(Self::LoopMerge {
                merge_block: ObjectId(*merge_block),
                continue_target_label: ObjectId(*continue_target_label),
            }),
            (spirv_::Op::Branch, None, None, [Operand_::IdRef(target_label)]) => {
                Some(Self::Branch {
                    target_label: ObjectId(*target_label),
                })
            }
            (
                spirv_::Op::BranchConditional,
                None,
                None,
                [Operand_::IdRef(condition), Operand_::IdRef(true_label), Operand_::IdRef(false_label)],
            ) => Some(Self::BranchConditional {
                condition: ObjectId(*condition),
                true_label: ObjectId(*true_label),
                false_label: ObjectId(*false_label),
            }),
            (spirv_::Op::Return, None, None, &[]) => Some(Self::Return),
            (spirv_::Op::Kill, None, None, &[]) => Some(Self::Kill),
            _ => {
                unimplemented!("{instruction:#?}")
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
