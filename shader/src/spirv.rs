use crate::error::Error;
use crate::il::{BinaryOp, UnaryOp};
use hashbrown::HashMap;
use log::debug;
use rspirv::binary::Disassemble;
use rspirv::dr::{Instruction as DrInstruction, Module as DrModule, Operand as DrOperand};
use rspirv::spirv as spv;

/// The parts of a SPIR-V module needed to execute its entry point.
#[derive(Debug, Clone)]
pub struct Spirv {
    pub(crate) entry_point: EntryPoint,
    pub(crate) objects: HashMap<ObjectId, Object>,
    pub(crate) functions: HashMap<ObjectId, Function>,
}

impl Spirv {
    pub(crate) fn new(name: &str, code: Vec<u32>) -> Result<Self, Error> {
        let mut loader = rspirv::dr::Loader::new();
        rspirv::binary::parse_words(&code, &mut loader).map_err(|source| Error::ParseSpirv {
            name: name.to_owned(),
            source,
        })?;
        let module = loader.module();
        debug!("SPIR-V shader {name:?}:\n{}", module.disassemble());

        let entry_point = EntryPoint::parse(name, &module)?;
        validate_capability(name, &module)?;
        validate_memory_model(name, &module)?;

        let objects = Object::parse(name, &module)?;
        let functions = Function::parse(name, &module)?;

        Ok(Self {
            entry_point,
            objects,
            functions,
        })
    }
}

#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub(crate) function: ObjectId,
}

impl EntryPoint {
    /// Parses and validates `OpEntryPoint`.
    fn parse(name: &str, module: &DrModule) -> Result<Self, Error> {
        let Some(entry_point) = module.entry_points.first() else {
            return Err(Error::invalid_spirv(name, "module has no entry point"));
        };
        match entry_point.operands.as_slice() {
            [
                DrOperand::ExecutionModel(
                    spv::ExecutionModel::Vertex | spv::ExecutionModel::Fragment,
                ),
                DrOperand::IdRef(function),
                _,
                ..,
            ] => Ok(Self {
                function: ObjectId(*function),
            }),
            operands => Err(Error::invalid_spirv(
                name,
                format!("unsupported OpEntryPoint operands: {operands:?}"),
            )),
        }
    }
}

/// Validates that the module's first capability is the shader capability.
fn validate_capability(name: &str, module: &DrModule) -> Result<(), Error> {
    let Some(capability) = module.capabilities.first() else {
        return Err(Error::invalid_spirv(name, "module has no capability"));
    };
    match capability.operands.as_slice() {
        [DrOperand::Capability(spv::Capability::Shader)] => Ok(()),
        operands => Err(Error::invalid_spirv(
            name,
            format!("unsupported OpCapability operands: {operands:?}"),
        )),
    }
}

/// Validates the logical addressing model required by the interpreter.
fn validate_memory_model(name: &str, module: &DrModule) -> Result<(), Error> {
    let Some(memory_model) = &module.memory_model else {
        return Err(Error::invalid_spirv(name, "module has no memory model"));
    };
    match memory_model.operands.as_slice() {
        [
            DrOperand::AddressingModel(spv::AddressingModel::Logical),
            DrOperand::MemoryModel(spv::MemoryModel::GLSL450),
        ] => Ok(()),
        operands => Err(Error::invalid_spirv(
            name,
            format!("unsupported OpMemoryModel operands: {operands:?}"),
        )),
    }
}

/// Whether the interpreter can allocate a variable in this storage class.
const fn is_supported_storage_class(storage_class: spv::StorageClass) -> bool {
    matches!(
        storage_class,
        spv::StorageClass::Input
            | spv::StorageClass::Output
            | spv::StorageClass::Function
            | spv::StorageClass::PushConstant
            | spv::StorageClass::Uniform
    )
}

#[derive(Debug, Clone)]
pub enum Object {
    Type(Type),
    Constant(Constant),
    Variable(Variable),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ObjectId(pub u32);

impl Object {
    /// Parses global types, constants, and variables.
    fn parse(name: &str, module: &DrModule) -> Result<HashMap<ObjectId, Self>, Error> {
        let mut types: HashMap<ObjectId, Type> = HashMap::default();
        let mut constants: HashMap<ObjectId, Constant> = HashMap::default();
        let mut variables: HashMap<ObjectId, Variable> = HashMap::default();

        for instruction in &module.types_global_values {
            let (opcode, result_type, result_id, operands) = deconstruct_instruction(instruction);
            let Some(result_id) = result_id else {
                return Err(Error::invalid_spirv(
                    name,
                    format!("{opcode:?} has no result id"),
                ));
            };
            let id = ObjectId(*result_id);

            match opcode {
                spv::Op::TypeVoid => {
                    types.insert(id, Type::Void);
                }
                spv::Op::TypeBool => {
                    types.insert(id, Type::Bool);
                }
                spv::Op::TypeFunction => {
                    types.insert(id, Type::Function(id_refs(name, operands)?));
                }
                spv::Op::TypeFloat => match operands {
                    [DrOperand::LiteralBit32(width)] => {
                        types.insert(id, Type::Float { width: *width });
                    }
                    _ => return Err(invalid_operands(name, *opcode, operands)),
                },
                spv::Op::TypeInt => match operands {
                    [
                        DrOperand::LiteralBit32(width),
                        DrOperand::LiteralBit32(signedness),
                    ] => {
                        types.insert(
                            id,
                            Type::Int {
                                width: *width,
                                signedness: *signedness == 1,
                            },
                        );
                    }
                    _ => return Err(invalid_operands(name, *opcode, operands)),
                },
                spv::Op::TypeArray => match operands {
                    [DrOperand::IdRef(element_type), DrOperand::IdRef(length)] => {
                        types.insert(
                            id,
                            Type::Array {
                                element_type: ObjectId(*element_type),
                                length: ObjectId(*length),
                                decorations: Decorations::default(),
                            },
                        );
                    }
                    _ => return Err(invalid_operands(name, *opcode, operands)),
                },
                spv::Op::TypeVector => match operands {
                    [
                        DrOperand::IdRef(component_type),
                        DrOperand::LiteralBit32(component_count),
                    ] => {
                        types.insert(
                            id,
                            Type::Vector {
                                component_type: ObjectId(*component_type),
                                component_count: *component_count,
                            },
                        );
                    }
                    _ => return Err(invalid_operands(name, *opcode, operands)),
                },
                spv::Op::TypeStruct => {
                    let member_types = id_refs(name, operands)?
                        .into_iter()
                        .map(|type_| MemberType {
                            type_,
                            decorations: Decorations::default(),
                        })
                        .collect();
                    types.insert(
                        id,
                        Type::Struct {
                            member_types,
                            decorations: Decorations::default(),
                        },
                    );
                }
                spv::Op::TypePointer => match operands {
                    [
                        DrOperand::StorageClass(storage_class),
                        DrOperand::IdRef(type_),
                    ] => {
                        if !is_supported_storage_class(*storage_class) {
                            return Err(Error::unsupported_spirv(
                                name,
                                format!("storage class {storage_class:?}"),
                            ));
                        }
                        types.insert(
                            id,
                            Type::Pointer {
                                type_: ObjectId(*type_),
                            },
                        );
                    }
                    _ => return Err(invalid_operands(name, *opcode, operands)),
                },
                spv::Op::Variable => {
                    let Some(result_type) = result_type else {
                        return Err(Error::invalid_spirv(name, "OpVariable has no pointer type"));
                    };
                    match operands {
                        [DrOperand::StorageClass(storage_class)] => {
                            if !is_supported_storage_class(*storage_class) {
                                return Err(Error::unsupported_spirv(
                                    name,
                                    format!("storage class {storage_class:?}"),
                                ));
                            }
                            variables.insert(
                                id,
                                Variable {
                                    type_: ObjectId(*result_type),
                                    decorations: Decorations::default(),
                                },
                            );
                        }
                        _ => return Err(invalid_operands(name, *opcode, operands)),
                    }
                }
                spv::Op::Constant => {
                    let Some(result_type) = result_type else {
                        return Err(Error::invalid_spirv(name, "OpConstant has no result type"));
                    };
                    match operands {
                        // rspirv stores 32-bit literals as raw bits, the interpreter's format.
                        [DrOperand::LiteralBit32(value)] => {
                            constants.insert(
                                id,
                                Constant::Scalar {
                                    type_: ObjectId(*result_type),
                                    value: *value,
                                },
                            );
                        }
                        [DrOperand::LiteralBit64(_)] => {
                            return Err(Error::unsupported_spirv(name, "64-bit constants"));
                        }
                        _ => return Err(invalid_operands(name, *opcode, operands)),
                    }
                }
                spv::Op::ConstantComposite => {
                    let Some(result_type) = result_type else {
                        return Err(Error::invalid_spirv(
                            name,
                            "OpConstantComposite has no result type",
                        ));
                    };
                    constants.insert(
                        id,
                        Constant::Composite {
                            type_: ObjectId(*result_type),
                            constituents: id_refs(name, operands)?,
                        },
                    );
                }
                _ => {
                    return Err(Error::unsupported_spirv(
                        name,
                        format!("global instruction: {instruction:?}"),
                    ));
                }
            }
        }

        Decorations::parse(name, module, &mut types, &mut variables)?;

        let mut objects = HashMap::with_capacity(types.len() + constants.len() + variables.len());
        objects.extend(types.into_iter().map(|(id, type_)| (id, Self::Type(type_))));
        objects.extend(
            constants
                .into_iter()
                .map(|(id, constant)| (id, Self::Constant(constant))),
        );
        objects.extend(
            variables
                .into_iter()
                .map(|(id, variable)| (id, Self::Variable(variable))),
        );
        Ok(objects)
    }
}

fn invalid_operands(name: &str, opcode: spv::Op, operands: &[DrOperand]) -> Error {
    Error::invalid_spirv(
        name,
        format!("unexpected operands for {opcode:?}: {operands:?}"),
    )
}

fn id_refs(name: &str, operands: &[DrOperand]) -> Result<Vec<ObjectId>, Error> {
    operands
        .iter()
        .map(|operand| match operand {
            DrOperand::IdRef(id) => Ok(ObjectId(*id)),
            other => Err(Error::invalid_spirv(
                name,
                format!("expected an id reference, found {other:?}"),
            )),
        })
        .collect()
}

fn literal_words(name: &str, operands: &[DrOperand]) -> Result<Vec<u32>, Error> {
    operands
        .iter()
        .map(|operand| match operand {
            DrOperand::LiteralBit32(value) => Ok(*value),
            other => Err(Error::invalid_spirv(
                name,
                format!("expected a 32-bit literal, found {other:?}"),
            )),
        })
        .collect()
}

#[derive(Debug, Clone)]
pub enum Type {
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
        type_: ObjectId,
    },
}

#[derive(Debug, Clone)]
pub struct MemberType {
    pub(crate) type_: ObjectId,
    pub(crate) decorations: Decorations,
}

impl Type {
    fn decorate_member(
        name: &str,
        types: &mut HashMap<ObjectId, Self>,
        target: &spv::Word,
        member: u32,
        decoration: &spv::Decoration,
        literals: &[DrOperand],
    ) -> Result<(), Error> {
        let Some(Self::Struct { member_types, .. }) = types.get_mut(&ObjectId(*target)) else {
            return Err(Error::invalid_spirv(
                name,
                format!("OpMemberDecorate target {target} is not a struct"),
            ));
        };
        let Some(member_type) = member_types.get_mut(to_usize(name, member)?) else {
            return Err(Error::invalid_spirv(
                name,
                format!("OpMemberDecorate member {member} is out of range"),
            ));
        };
        member_type.decorations.decorate(name, decoration, literals)
    }

    fn decorate(
        name: &str,
        types: &mut HashMap<ObjectId, Self>,
        target: &spv::Word,
        decoration: &spv::Decoration,
        literals: &[DrOperand],
    ) -> Result<bool, Error> {
        let Some(type_) = types.get_mut(&ObjectId(*target)) else {
            return Ok(false);
        };
        let decorations = match type_ {
            Self::Array { decorations, .. } | Self::Struct { decorations, .. } => decorations,
            other => {
                return Err(Error::invalid_spirv(
                    name,
                    format!("type {target} cannot be decorated: {other:?}"),
                ));
            }
        };
        decorations.decorate(name, decoration, literals)?;
        Ok(true)
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

#[derive(Debug, Clone)]
pub struct Variable {
    pub(crate) type_: ObjectId,
    pub(crate) decorations: Decorations,
}

impl Variable {
    fn decorate(
        name: &str,
        variables: &mut HashMap<ObjectId, Self>,
        target: &spv::Word,
        decoration: &spv::Decoration,
        literals: &[DrOperand],
    ) -> Result<bool, Error> {
        let Some(variable) = variables.get_mut(&ObjectId(*target)) else {
            return Ok(false);
        };
        variable.decorations.decorate(name, decoration, literals)?;
        Ok(true)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Decorations {
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
    fn decorate(
        &mut self,
        name: &str,
        decoration: &spv::Decoration,
        literals: &[DrOperand],
    ) -> Result<(), Error> {
        match (decoration, literals) {
            (spv::Decoration::BuiltIn, [operand]) => {
                if self.builtin.is_some() {
                    return Err(Error::invalid_spirv(name, "duplicate BuiltIn decoration"));
                }
                let builtin = BuiltInDecoration::from_operand(operand).ok_or_else(|| {
                    Error::unsupported_spirv(name, format!("built-in {operand:?}"))
                })?;
                self.builtin = Some(builtin);
            }
            (spv::Decoration::Block, _) => self.block = true,
            (spv::Decoration::Location, [DrOperand::LiteralBit32(location)]) => {
                self.location = Some(LocationDecoration { number: *location });
            }
            (spv::Decoration::RelaxedPrecision, &[]) => self.relaxed_precision = true,
            (spv::Decoration::Offset, [DrOperand::LiteralBit32(byte_offset)]) => {
                self.byte_offset = Some(*byte_offset);
            }
            (spv::Decoration::ArrayStride, [DrOperand::LiteralBit32(array_stride)]) => {
                self.array_stride = Some(*array_stride);
            }
            (spv::Decoration::DescriptorSet, [DrOperand::LiteralBit32(descriptor_set)]) => {
                self.descriptor_set = Some(*descriptor_set);
            }
            (spv::Decoration::Binding, [DrOperand::LiteralBit32(binding_point)]) => {
                self.binding_point = Some(*binding_point);
            }
            _ => {
                return Err(Error::unsupported_spirv(
                    name,
                    format!("decoration {decoration:?} with operands {literals:?}"),
                ));
            }
        }
        Ok(())
    }

    /// Parses and applies decorations to types and variables.
    fn parse(
        name: &str,
        module: &DrModule,
        types: &mut HashMap<ObjectId, Type>,
        variables: &mut HashMap<ObjectId, Variable>,
    ) -> Result<(), Error> {
        for instruction in &module.annotations {
            let (opcode, result_type, result_id, operands) = deconstruct_instruction(instruction);
            if result_type.is_some() || result_id.is_some() {
                return Err(Error::invalid_spirv(
                    name,
                    format!("unexpected result id on annotation {opcode:?}"),
                ));
            }
            match (opcode, operands) {
                (
                    spv::Op::MemberDecorate,
                    [
                        DrOperand::IdRef(target),
                        DrOperand::LiteralBit32(member),
                        DrOperand::Decoration(decoration),
                        literals @ ..,
                    ],
                ) => Type::decorate_member(name, types, target, *member, decoration, literals)?,
                (
                    spv::Op::Decorate,
                    [
                        DrOperand::IdRef(target),
                        DrOperand::Decoration(decoration),
                        literals @ ..,
                    ],
                ) => {
                    if !Type::decorate(name, types, target, decoration, literals)?
                        && !Variable::decorate(name, variables, target, decoration, literals)?
                    {
                        return Err(Error::invalid_spirv(
                            name,
                            format!("decoration target {target} is not a type or variable"),
                        ));
                    }
                }
                _ => {
                    return Err(Error::unsupported_spirv(
                        name,
                        format!("annotation instruction: {instruction:?}"),
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BuiltInDecoration {
    Position,
    PointSize,
    VertexIndex,
    FragCoord,
    ClipDistance,
    CullDistance,
}

impl BuiltInDecoration {
    const fn from_operand(operand: &DrOperand) -> Option<Self> {
        match operand {
            DrOperand::BuiltIn(spv::BuiltIn::Position) => Some(Self::Position),
            DrOperand::BuiltIn(spv::BuiltIn::PointSize) => Some(Self::PointSize),
            DrOperand::BuiltIn(spv::BuiltIn::VertexIndex) => Some(Self::VertexIndex),
            DrOperand::BuiltIn(spv::BuiltIn::FragCoord) => Some(Self::FragCoord),
            DrOperand::BuiltIn(spv::BuiltIn::ClipDistance) => Some(Self::ClipDistance),
            DrOperand::BuiltIn(spv::BuiltIn::CullDistance) => Some(Self::CullDistance),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LocationDecoration {
    pub number: u32,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub(crate) instructions: Vec<Instruction>,
}

impl Function {
    /// Parses the functions in the module.
    fn parse(name: &str, module: &DrModule) -> Result<HashMap<ObjectId, Self>, Error> {
        let mut functions = HashMap::default();
        for function in &module.functions {
            let definition = function
                .def
                .as_ref()
                .ok_or_else(|| Error::invalid_spirv(name, "function has no definition"))?;
            let (opcode, result_type, result_id, operands) = deconstruct_instruction(definition);
            let (
                spv::Op::Function,
                &Some(_result_type),
                &Some(result_id),
                [
                    DrOperand::FunctionControl(_function_control),
                    DrOperand::IdRef(_function_type),
                ],
            ) = (opcode, result_type, result_id, operands)
            else {
                return Err(Error::invalid_spirv(
                    name,
                    format!("unsupported function definition: {definition:?}"),
                ));
            };
            if !function.parameters.is_empty() {
                return Err(Error::unsupported_spirv(name, "function parameters"));
            }

            let mut instructions = Vec::new();
            for block in &function.blocks {
                let label = block
                    .label
                    .as_ref()
                    .ok_or_else(|| Error::invalid_spirv(name, "block has no label"))?;
                instructions.push(Instruction::parse(name, label)?);
                for instruction in &block.instructions {
                    instructions.push(Instruction::parse(name, instruction)?);
                }
            }
            functions.insert(ObjectId(result_id), Self { instructions });
        }
        Ok(functions)
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
    BinaryOp {
        result_id: ObjectId,
        result_type: ObjectId,
        operand1: ObjectId,
        operand2: ObjectId,
        op: BinaryOp,
    },
    UnaryOp {
        result_id: ObjectId,
        result_type: ObjectId,
        operand: ObjectId,
        op: UnaryOp,
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
    /// Parses one instruction from the entry point's function.
    fn parse(name: &str, instruction: &DrInstruction) -> Result<Self, Error> {
        let (opcode, result_type, result_id, operands) = deconstruct_instruction(instruction);

        if let Some(op) = binary_op(*opcode) {
            let (
                &Some(result_type),
                &Some(result_id),
                [DrOperand::IdRef(operand1), DrOperand::IdRef(operand2)],
            ) = (result_type, result_id, operands)
            else {
                return Err(invalid_operands(name, *opcode, operands));
            };
            return Ok(Self::BinaryOp {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand1: ObjectId(*operand1),
                operand2: ObjectId(*operand2),
                op,
            });
        }

        if let Some(op) = unary_op(*opcode) {
            let (&Some(result_type), &Some(result_id), [DrOperand::IdRef(operand)]) =
                (result_type, result_id, operands)
            else {
                return Err(invalid_operands(name, *opcode, operands));
            };
            return Ok(Self::UnaryOp {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                operand: ObjectId(*operand),
                op,
            });
        }

        match (opcode, result_type, result_id, operands) {
            (spv::Op::Label, None, &Some(result_id), &[]) => Ok(Self::Label {
                result_id: ObjectId(result_id),
            }),
            (
                spv::Op::AccessChain,
                &Some(result_type),
                &Some(result_id),
                [DrOperand::IdRef(base), indexes @ ..],
            ) => Ok(Self::AccessChain {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                base: ObjectId(*base),
                indexes: id_refs(name, indexes)?,
            }),
            (spv::Op::Store, None, None, [DrOperand::IdRef(pointer), DrOperand::IdRef(object)]) => {
                Ok(Self::Store {
                    pointer: ObjectId(*pointer),
                    object: ObjectId(*object),
                })
            }
            (spv::Op::Load, &Some(result_type), &Some(result_id), [DrOperand::IdRef(pointer)]) => {
                Ok(Self::Load {
                    result_id: ObjectId(result_id),
                    result_type: ObjectId(result_type),
                    pointer: ObjectId(*pointer),
                })
            }
            (
                spv::Op::CompositeExtract,
                &Some(result_type),
                &Some(result_id),
                [DrOperand::IdRef(composite), indexes @ ..],
            ) => Ok(Self::CompositeExtract {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                composite: ObjectId(*composite),
                indexes: literal_words(name, indexes)?,
            }),
            (spv::Op::CompositeConstruct, &Some(result_type), &Some(result_id), constituents) => {
                Ok(Self::CompositeConstruct {
                    result_id: ObjectId(result_id),
                    result_type: ObjectId(result_type),
                    constituents: id_refs(name, constituents)?,
                })
            }
            (
                spv::Op::Variable,
                &Some(result_type),
                &Some(result_id),
                [DrOperand::StorageClass(storage_class)],
            ) => {
                if !is_supported_storage_class(*storage_class) {
                    return Err(Error::unsupported_spirv(
                        name,
                        format!("storage class {storage_class:?}"),
                    ));
                }
                Ok(Self::Variable {
                    result_id: ObjectId(result_id),
                    result_type: ObjectId(result_type),
                })
            }
            (
                spv::Op::Select,
                &Some(result_type),
                &Some(result_id),
                [
                    DrOperand::IdRef(condition),
                    DrOperand::IdRef(object1),
                    DrOperand::IdRef(object2),
                ],
            ) => Ok(Self::Select {
                result_id: ObjectId(result_id),
                result_type: ObjectId(result_type),
                condition: ObjectId(*condition),
                object1: ObjectId(*object1),
                object2: ObjectId(*object2),
            }),
            (
                spv::Op::SelectionMerge,
                None,
                None,
                [
                    DrOperand::IdRef(merge_block),
                    DrOperand::SelectionControl(_),
                ],
            ) => Ok(Self::SelectionMerge {
                merge_block: ObjectId(*merge_block),
            }),
            (
                spv::Op::LoopMerge,
                None,
                None,
                [
                    DrOperand::IdRef(merge_block),
                    DrOperand::IdRef(continue_target_label),
                    DrOperand::LoopControl(_),
                    ..,
                ],
            ) => Ok(Self::LoopMerge {
                merge_block: ObjectId(*merge_block),
                continue_target_label: ObjectId(*continue_target_label),
            }),
            (spv::Op::Branch, None, None, [DrOperand::IdRef(target_label)]) => Ok(Self::Branch {
                target_label: ObjectId(*target_label),
            }),
            (
                spv::Op::BranchConditional,
                None,
                None,
                [
                    DrOperand::IdRef(condition),
                    DrOperand::IdRef(true_label),
                    DrOperand::IdRef(false_label),
                ],
            ) => Ok(Self::BranchConditional {
                condition: ObjectId(*condition),
                true_label: ObjectId(*true_label),
                false_label: ObjectId(*false_label),
            }),
            (spv::Op::Return, None, None, &[]) => Ok(Self::Return),
            (spv::Op::Kill, None, None, &[]) => Ok(Self::Kill),
            _ => Err(Error::unsupported_spirv(
                name,
                format!("instruction: {instruction:?}"),
            )),
        }
    }
}

/// Maps a SPIR-V arithmetic, bitwise, or comparison opcode to an IR operation.
const fn binary_op(opcode: spv::Op) -> Option<BinaryOp> {
    match opcode {
        spv::Op::VectorTimesScalar => Some(BinaryOp::MulVectorScalar),
        spv::Op::IAdd => Some(BinaryOp::AddI32I32),
        spv::Op::IMul => Some(BinaryOp::MulI32I32),
        spv::Op::FSub => Some(BinaryOp::SubF32F32),
        spv::Op::FDiv => Some(BinaryOp::DivF32F32),
        spv::Op::SDiv => Some(BinaryOp::DivI32I32),
        spv::Op::UDiv => Some(BinaryOp::DivU32U32),
        spv::Op::SMod => Some(BinaryOp::ModI32I32),
        spv::Op::UMod => Some(BinaryOp::ModU32U32),
        spv::Op::BitwiseAnd => Some(BinaryOp::BitAnd),
        spv::Op::ShiftRightLogical => Some(BinaryOp::BitShiftRight),
        spv::Op::IEqual => Some(BinaryOp::EqualI32I32),
        spv::Op::ULessThan => Some(BinaryOp::LessThanU32U32),
        _ => None,
    }
}

/// Maps a SPIR-V conversion opcode to an IR operation.
const fn unary_op(opcode: spv::Op) -> Option<UnaryOp> {
    match opcode {
        spv::Op::ConvertSToF => Some(UnaryOp::ConvertI32ToF32),
        spv::Op::ConvertFToU => Some(UnaryOp::ConvertF32ToU32),
        spv::Op::ConvertUToF => Some(UnaryOp::ConvertU32ToF32),
        _ => None,
    }
}

fn to_usize(name: &str, value: u32) -> Result<usize, Error> {
    usize::try_from(value)
        .map_err(|_| Error::invalid_spirv(name, format!("{value} does not fit in usize")))
}

const fn deconstruct_instruction(
    instruction: &DrInstruction,
) -> (
    &spv::Op,
    &Option<spv::Word>,
    &Option<spv::Word>,
    &[DrOperand],
) {
    (
        &instruction.class.opcode,
        &instruction.result_type,
        &instruction.result_id,
        instruction.operands.as_slice(),
    )
}
