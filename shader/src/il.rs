use crate::error::Error;
use crate::spirv::{self, Spirv};

/// A program in the interpreter's intermediate representation.
#[derive(Debug, Clone)]
pub struct Il {
    pub(crate) instructions: Vec<Instruction>,
}

impl Il {
    pub fn new(name: &str, code: Vec<u32>) -> Result<Self, Error> {
        let spirv = Spirv::new(name, code)?;
        let instructions = Self::parse_spirv(name, &spirv)?;
        Ok(Self { instructions })
    }

    fn parse_spirv(name: &str, spirv: &Spirv) -> Result<Vec<Instruction>, Error> {
        let mut lowering = Lowering {
            name,
            spirv,
            instructions: Vec::new(),
        };
        lowering.lower_globals()?;
        lowering.lower_entry_point()?;
        Ok(lowering.instructions)
    }
}

/// A variable in the intermediate representation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Variable(u32);

impl Variable {
    const fn from_spirv(id: &spirv::ObjectId) -> Self {
        Self(id.0)
    }
}

/// Binary operations supported by the interpreter.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    MulVectorScalar,
    AddI32I32,
    MulI32I32,
    SubF32F32,
    DivF32F32,
    DivI32I32,
    DivU32U32,
    ModI32I32,
    ModU32U32,
    BitAnd,
    BitShiftRight,
    EqualI32I32,
    LessThanU32U32,
}

/// Type conversions supported by the interpreter.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOp {
    ConvertI32ToF32,
    ConvertF32ToU32,
    ConvertU32ToF32,
}

/// One instruction of the intermediate representation.
#[derive(Debug, Clone)]
pub enum Instruction {
    Label {
        id: u32,
    },
    VariableDecl {
        id: Variable,
        decl: VariableDecl,
    },
    StoreImm32 {
        dst: Variable,
        imm: u32,
    },
    StoreImm32Array {
        dst: Variable,
        imm: Vec<u32>,
    },
    LoadVariableOffset {
        id: Variable,
        base: Variable,
        offsets: Vec<Variable>,
    },
    LoadVariableImmOffset {
        id: Variable,
        base: Variable,
        offset: u32,
    },
    StoreVariable {
        dst_pointer: Variable,
        src: Variable,
    },
    StoreVariableArray {
        dst: Variable,
        values: Vec<Variable>,
    },
    LoadVariable {
        id: Variable,
        src_pointer: Variable,
    },
    MathBinary {
        id: Variable,
        op1: Variable,
        op2: Variable,
        op: BinaryOp,
    },
    MathUnary {
        id: Variable,
        operand: Variable,
        op: UnaryOp,
    },
    Return,
    Select {
        id: Variable,
        cond: Variable,
        obj1: Variable,
        obj2: Variable,
    },
    SelectionMerge {
        merge_block_label: u32,
    },
    LoopMerge {
        merge_block_label: u32,
        continue_target_label: u32,
    },
    Branch {
        target_label: u32,
    },
    BranchConditional {
        condition: Variable,
        true_label: u32,
        false_label: u32,
    },
    Kill,
}

/// Lowers parsed SPIR-V instructions to the intermediate representation.
struct Lowering<'a> {
    name: &'a str,
    spirv: &'a Spirv,
    instructions: Vec<Instruction>,
}

impl Lowering<'_> {
    /// Emits a declaration for the result of a SPIR-V instruction.
    fn declare(
        &mut self,
        result_id: &spirv::ObjectId,
        result_type: &spirv::ObjectId,
    ) -> Result<Variable, Error> {
        let decl = get_variable_decl(self.spirv, self.name, result_type, VariableBacking::Memory)?;
        let id = Variable::from_spirv(result_id);
        self.instructions
            .push(Instruction::VariableDecl { id, decl });
        Ok(id)
    }

    fn lower_globals(&mut self) -> Result<(), Error> {
        let spirv = self.spirv;
        let name = self.name;
        for (id, object) in &spirv.objects {
            match object {
                spirv::Object::Type(_) => {}
                spirv::Object::Constant(constant) => match constant {
                    spirv::Constant::Scalar { type_, value } => {
                        let decl = get_variable_decl(spirv, name, type_, VariableBacking::Memory)?;
                        let variable = Variable::from_spirv(id);
                        self.instructions
                            .push(Instruction::VariableDecl { id: variable, decl });
                        self.instructions.push(Instruction::StoreImm32 {
                            dst: variable,
                            imm: *value,
                        });
                    }
                    spirv::Constant::Composite {
                        type_,
                        constituents,
                    } => {
                        let decl = get_variable_decl(spirv, name, type_, VariableBacking::Memory)?;
                        let variable = Variable::from_spirv(id);
                        let values = constituents
                            .iter()
                            .map(|constituent| {
                                match get_spirv_constant(spirv, name, constituent)? {
                                    spirv::Constant::Scalar { value, .. } => Ok(*value),
                                    spirv::Constant::Composite { .. } => Err(
                                        Error::unsupported_il(name, "nested composite constant"),
                                    ),
                                }
                            })
                            .collect::<Result<Vec<_>, Error>>()?;
                        self.instructions
                            .push(Instruction::VariableDecl { id: variable, decl });
                        self.instructions.push(Instruction::StoreImm32Array {
                            dst: variable,
                            imm: values,
                        });
                    }
                },
                spirv::Object::Variable(variable) => {
                    let decl = get_variable_decl(
                        spirv,
                        name,
                        &variable.type_,
                        VariableBacking::from_decorations(&variable.decorations),
                    )?;
                    let id = Variable::from_spirv(id);
                    self.instructions
                        .push(Instruction::VariableDecl { id, decl });
                }
            }
        }
        Ok(())
    }

    fn lower_entry_point(&mut self) -> Result<(), Error> {
        let spirv = self.spirv;
        let Some(function) = spirv.functions.get(&spirv.entry_point.function) else {
            return Err(Error::invalid_spirv(
                self.name,
                format!(
                    "entry point function {} is missing",
                    spirv.entry_point.function.0
                ),
            ));
        };
        for instruction in &function.instructions {
            self.lower_instruction(instruction)?;
        }
        Ok(())
    }

    fn lower_instruction(&mut self, instruction: &spirv::Instruction) -> Result<(), Error> {
        match instruction {
            spirv::Instruction::Label { result_id } => {
                self.instructions
                    .push(Instruction::Label { id: result_id.0 });
            }
            spirv::Instruction::AccessChain {
                result_id,
                result_type,
                base,
                indexes,
            } => {
                let id = self.declare(result_id, result_type)?;
                let offsets = indexes.iter().map(Variable::from_spirv).collect();
                self.instructions.push(Instruction::LoadVariableOffset {
                    id,
                    base: Variable::from_spirv(base),
                    offsets,
                });
            }
            spirv::Instruction::Store { pointer, object } => {
                self.instructions.push(Instruction::StoreVariable {
                    dst_pointer: Variable::from_spirv(pointer),
                    src: Variable::from_spirv(object),
                });
            }
            spirv::Instruction::Load {
                result_id,
                result_type,
                pointer,
            } => {
                let id = self.declare(result_id, result_type)?;
                self.instructions.push(Instruction::LoadVariable {
                    id,
                    src_pointer: Variable::from_spirv(pointer),
                });
            }
            spirv::Instruction::BinaryOp {
                result_id,
                result_type,
                operand1,
                operand2,
                op,
            } => {
                let id = self.declare(result_id, result_type)?;
                self.instructions.push(Instruction::MathBinary {
                    id,
                    op1: Variable::from_spirv(operand1),
                    op2: Variable::from_spirv(operand2),
                    op: *op,
                });
            }
            spirv::Instruction::UnaryOp {
                result_id,
                result_type,
                operand,
                op,
            } => {
                let id = self.declare(result_id, result_type)?;
                self.instructions.push(Instruction::MathUnary {
                    id,
                    operand: Variable::from_spirv(operand),
                    op: *op,
                });
            }
            spirv::Instruction::CompositeExtract {
                result_id,
                result_type,
                composite,
                indexes,
            } => {
                let [offset] = indexes.as_slice() else {
                    return Err(Error::unsupported_il(
                        self.name,
                        format!("composite extract with {} indexes", indexes.len()),
                    ));
                };
                let id = self.declare(result_id, result_type)?;
                self.instructions.push(Instruction::LoadVariableImmOffset {
                    id,
                    base: Variable::from_spirv(composite),
                    offset: *offset,
                });
            }
            spirv::Instruction::CompositeConstruct {
                result_id,
                result_type,
                constituents,
            } => {
                let id = self.declare(result_id, result_type)?;
                let values = constituents.iter().map(Variable::from_spirv).collect();
                self.instructions
                    .push(Instruction::StoreVariableArray { dst: id, values });
            }
            spirv::Instruction::Variable {
                result_id,
                result_type,
            } => {
                self.declare(result_id, result_type)?;
            }
            spirv::Instruction::Select {
                result_id,
                result_type,
                condition,
                object1,
                object2,
            } => {
                let id = self.declare(result_id, result_type)?;
                self.instructions.push(Instruction::Select {
                    id,
                    cond: Variable::from_spirv(condition),
                    obj1: Variable::from_spirv(object1),
                    obj2: Variable::from_spirv(object2),
                });
            }
            spirv::Instruction::SelectionMerge { merge_block } => {
                self.instructions.push(Instruction::SelectionMerge {
                    merge_block_label: merge_block.0,
                });
            }
            spirv::Instruction::LoopMerge {
                merge_block,
                continue_target_label,
            } => {
                self.instructions.push(Instruction::LoopMerge {
                    merge_block_label: merge_block.0,
                    continue_target_label: continue_target_label.0,
                });
            }
            spirv::Instruction::Branch { target_label } => {
                self.instructions.push(Instruction::Branch {
                    target_label: target_label.0,
                });
            }
            spirv::Instruction::BranchConditional {
                condition,
                true_label,
                false_label,
            } => {
                self.instructions.push(Instruction::BranchConditional {
                    condition: Variable::from_spirv(condition),
                    true_label: true_label.0,
                    false_label: false_label.0,
                });
            }
            spirv::Instruction::Return => {
                self.instructions.push(Instruction::Return);
            }
            spirv::Instruction::Kill => {
                self.instructions.push(Instruction::Kill);
            }
        }
        Ok(())
    }
}

impl VariableBacking {
    const fn from_decorations(decorations: &spirv::Decorations) -> Self {
        if let Some(builtin) = decorations.builtin {
            match builtin {
                spirv::BuiltInDecoration::Position => Self::Position,
                spirv::BuiltInDecoration::PointSize => Self::PointSize,
                spirv::BuiltInDecoration::VertexIndex => Self::VertexIndex,
                spirv::BuiltInDecoration::FragCoord => Self::FragCoord,
                spirv::BuiltInDecoration::ClipDistance => Self::ClipDistance,
                spirv::BuiltInDecoration::CullDistance => Self::CullDistance,
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

/// Resolves a type id to its declaration.
fn get_spirv_type<'a>(
    spirv: &'a Spirv,
    name: &str,
    id: &spirv::ObjectId,
) -> Result<&'a spirv::Type, Error> {
    match spirv.objects.get(id) {
        Some(spirv::Object::Type(type_)) => Ok(type_),
        Some(other) => Err(Error::invalid_spirv(
            name,
            format!("object {id:?} is not a type: {other:?}"),
        )),
        None => Err(Error::invalid_spirv(name, format!("unknown type {id:?}"))),
    }
}

/// Resolves a constant id to its declaration.
fn get_spirv_constant<'a>(
    spirv: &'a Spirv,
    name: &str,
    id: &spirv::ObjectId,
) -> Result<&'a spirv::Constant, Error> {
    match spirv.objects.get(id) {
        Some(spirv::Object::Constant(constant)) => Ok(constant),
        Some(other) => Err(Error::invalid_spirv(
            name,
            format!("object {id:?} is not a constant: {other:?}"),
        )),
        None => Err(Error::invalid_spirv(
            name,
            format!("unknown constant {id:?}"),
        )),
    }
}

fn get_variable_decl(
    spirv: &Spirv,
    name: &str,
    type_id: &spirv::ObjectId,
    backing: VariableBacking,
) -> Result<VariableDecl, Error> {
    let (kind, component_count, backing) = match get_spirv_type(spirv, name, type_id)? {
        spirv::Type::Float { width } => {
            if *width != 32 {
                return Err(Error::unsupported_spirv(
                    name,
                    format!("{width}-bit floating-point type"),
                ));
            }
            (VariableKind::F32, 1, backing)
        }
        spirv::Type::Int { width, signedness } => {
            if *width != 32 {
                return Err(Error::unsupported_spirv(
                    name,
                    format!("{width}-bit integer type"),
                ));
            }
            let kind = if *signedness {
                VariableKind::I32
            } else {
                VariableKind::U32
            };
            (kind, 1, backing)
        }
        spirv::Type::Void => (VariableKind::Void, 0, backing),
        spirv::Type::Bool => (VariableKind::Bool, 1, backing),
        spirv::Type::Function(_) => {
            return Err(Error::unsupported_spirv(name, "function type"));
        }
        spirv::Type::Array {
            element_type,
            length,
            decorations,
        } => {
            let element_kind = get_variable_decl(spirv, name, element_type, backing)?;
            let length = match get_spirv_constant(spirv, name, length)? {
                spirv::Constant::Scalar { value, .. } => *value,
                spirv::Constant::Composite { .. } => {
                    return Err(Error::invalid_spirv(
                        name,
                        "array length is not a scalar constant",
                    ));
                }
            };
            let array_stride = match decorations.array_stride {
                None if length == 1 => {
                    u32::try_from(std::mem::size_of::<u32>()).expect("u32 size fits in u32")
                }
                Some(array_stride) => array_stride,
                None => 0,
            };
            (
                VariableKind::Array,
                length,
                VariableBacking::Array {
                    element_kind: Box::new(element_kind),
                    array_stride,
                },
            )
        }
        spirv::Type::Vector {
            component_type,
            component_count,
        } => {
            let component_type = get_variable_decl(spirv, name, component_type, backing)?;
            if component_type.component_count == 1 {
                (
                    component_type.kind,
                    *component_count,
                    component_type.backing,
                )
            } else {
                return Err(Error::unsupported_spirv(name, "matrix type"));
            }
        }
        spirv::Type::Struct {
            member_types,
            decorations,
        } => {
            let members = member_types
                .iter()
                .map(|member| {
                    get_variable_decl(
                        spirv,
                        name,
                        &member.type_,
                        VariableBacking::from_decorations(&member.decorations),
                    )
                })
                .collect::<Result<Vec<_>, Error>>()?;
            if !decorations.block {
                return Err(Error::unsupported_spirv(
                    name,
                    "struct type without Block decoration",
                ));
            }
            let member_count = u32::try_from(member_types.len())
                .map_err(|_| Error::invalid_spirv(name, "too many struct members"))?;
            (
                VariableKind::Struct,
                member_count,
                VariableBacking::Struct { members },
            )
        }
        spirv::Type::Pointer { type_ } => {
            let kind = get_variable_decl(spirv, name, type_, backing)?;
            (
                VariableKind::Pointer,
                1,
                VariableBacking::Pointer {
                    kind: Box::new(kind),
                },
            )
        }
    };

    Ok(VariableDecl {
        kind,
        component_count,
        backing,
    })
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub(crate) kind: VariableKind,
    pub(crate) component_count: u32,
    pub(crate) backing: VariableBacking,
}

#[derive(Debug, Copy, Clone)]
pub enum VariableKind {
    F32,
    U32,
    I32,
    Void,
    Bool,
    Array,
    Struct,
    Pointer,
}

#[derive(Debug, Clone)]
pub enum VariableBacking {
    Memory,
    Location {
        number: u32,
    },
    Position,
    PointSize,
    VertexIndex,
    FragCoord,
    ClipDistance,
    CullDistance,
    Array {
        element_kind: Box<VariableDecl>,
        array_stride: u32,
    },
    Struct {
        members: Vec<VariableDecl>,
    },
    Pointer {
        kind: Box<VariableDecl>,
    },
}
