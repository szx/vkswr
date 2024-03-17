use crate::spirv;
use crate::spirv::Spirv;
use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Il {
    pub(crate) instructions: Vec<Instruction>,
}

impl Il {
    pub fn new(name: &str, code: Vec<u32>) -> anyhow::Result<Self> {
        let spirv = Spirv::new(name, code)?;
        let instructions = Self::parse_spirv(spirv)?;
        Ok(Self { instructions })
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Variable {
    ObjectId(u32),
}

impl Variable {
    pub const fn from_spirv(id: &spirv::ObjectId) -> Self {
        Self::ObjectId(id.0)
    }
}

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
    MathMulVectorScalar {
        id: Variable,
        vector: Variable,
        scalar: Variable,
    },
    MathAddI32I32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathMulI32I32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathSubF32F32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathDivF32F32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathDivI32I32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathDivU32U32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathModI32I32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathModU32U32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },

    MathBitAnd {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathBitShiftRight {
        id: Variable,
        base: Variable,
        shift: Variable,
    },

    MathEqualI32I32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathLessThanU32U32 {
        id: Variable,
        op1: Variable,
        op2: Variable,
    },
    MathConvertI32F32 {
        id: Variable,
        op: Variable,
    },
    MathConvertF32U32 {
        id: Variable,
        op: Variable,
    },
    MathConvertU32F32 {
        id: Variable,
        op: Variable,
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

impl Il {
    fn parse_spirv(spirv: Spirv) -> anyhow::Result<Vec<Instruction>> {
        let mut scalar_variables = vec![];
        let mut composite_variables = vec![];
        let mut pointer_variables = vec![];

        for (id, object) in &spirv.objects {
            match object {
                spirv::Object::Type(_) => continue,
                spirv::Object::Constant(constant) => match constant {
                    spirv::Constant::Scalar { type_, value } => {
                        let decl = Self::get_variable_decl(&spirv, type_, VariableBacking::Memory);
                        let id = Variable::from_spirv(id);
                        scalar_variables.push(Instruction::VariableDecl { id, decl });
                        scalar_variables.push(Instruction::StoreImm32 {
                            dst: id,
                            imm: *value,
                        });
                    }
                    spirv::Constant::Composite {
                        type_,
                        constituents,
                    } => {
                        let decl = Self::get_variable_decl(&spirv, type_, VariableBacking::Memory);
                        let id = Variable::from_spirv(id);
                        composite_variables.push(Instruction::VariableDecl { id, decl });
                        let values = constituents
                            .iter()
                            .map(|id| match Self::get_spirv_constant(&spirv, id) {
                                spirv::Constant::Scalar { type_: _, value } => *value,
                                spirv::Constant::Composite { .. } => {
                                    unreachable!()
                                }
                            })
                            .collect::<Vec<_>>();
                        composite_variables.push(Instruction::StoreImm32Array {
                            dst: id,
                            imm: values,
                        });
                    }
                },
                spirv::Object::Variable(spirv::Variable::Pointer(
                    spirv::Pointer::ToMemoryObject { memory_object },
                )) => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        &memory_object.type_,
                        Self::from_spirv_decorations(&memory_object.decorations),
                    );
                    let id = Variable::from_spirv(id);
                    pointer_variables.push(Instruction::VariableDecl { id, decl });
                }
            }
        }

        let mut instructions = vec![];
        instructions.extend(scalar_variables);
        instructions.extend(composite_variables);
        instructions.extend(pointer_variables);

        let main = spirv
            .functions
            .get(&spirv.entry_point.entry_point)
            .context("failed to get spirv function")?;
        for instruction in &main.instructions {
            match instruction {
                spirv::Instruction::Label { result_id } => {
                    instructions.push(Instruction::Label { id: result_id.0 });
                }
                spirv::Instruction::AccessChain {
                    result_id,
                    result_type,
                    base,
                    indexes,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::LoadVariableOffset {
                        id,
                        base: Variable::from_spirv(base),
                        offsets: indexes.iter().map(Variable::from_spirv).collect(),
                    });
                }
                spirv::Instruction::Store { pointer, object } => {
                    instructions.push(Instruction::StoreVariable {
                        dst_pointer: Variable::from_spirv(pointer),
                        src: Variable::from_spirv(object),
                    });
                }
                spirv::Instruction::Load {
                    result_id,
                    result_type,
                    pointer,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::LoadVariable {
                        id,
                        src_pointer: Variable::from_spirv(pointer),
                    });
                }
                spirv::Instruction::VectorTimesScalar {
                    result_id,
                    result_type,
                    vector,
                    scalar,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathMulVectorScalar {
                        id,
                        vector: Variable::from_spirv(vector),
                        scalar: Variable::from_spirv(scalar),
                    });
                }
                spirv::Instruction::IAdd {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathAddI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::IMul {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathMulI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::FSub {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathSubF32F32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::FDiv {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivF32F32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::SDiv {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::UDiv {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivU32U32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::SMod {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathModI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::UMod {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathModU32U32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::BitwiseAnd {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathBitAnd {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::ShiftRightLogical {
                    result_id,
                    result_type,
                    base,
                    shift,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathBitShiftRight {
                        id,
                        base: Variable::from_spirv(base),
                        shift: Variable::from_spirv(shift),
                    });
                }
                spirv::Instruction::IEqual {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathEqualI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }

                spirv::Instruction::ULessThan {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathLessThanU32U32 {
                        id,
                        op1: Variable::from_spirv(operand1),
                        op2: Variable::from_spirv(operand2),
                    });
                }
                spirv::Instruction::ConvertSToF {
                    result_id,
                    result_type,
                    operand,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathConvertI32F32 {
                        id,
                        op: Variable::from_spirv(operand),
                    });
                }
                spirv::Instruction::ConvertFToU {
                    result_id,
                    result_type,
                    operand,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathConvertF32U32 {
                        id,
                        op: Variable::from_spirv(operand),
                    });
                }
                spirv::Instruction::ConvertUToF {
                    result_id,
                    result_type,
                    operand,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathConvertU32F32 {
                        id,
                        op: Variable::from_spirv(operand),
                    });
                }
                spirv::Instruction::CompositeExtract {
                    result_id,
                    result_type,
                    composite,
                    indexes,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    if let [offset] = indexes.as_slice() {
                        instructions.push(Instruction::VariableDecl { id, decl });
                        instructions.push(Instruction::LoadVariableImmOffset {
                            id,
                            base: Variable::from_spirv(composite),
                            offset: *offset,
                        });
                    } else {
                        unimplemented!()
                    }
                }
                spirv::Instruction::CompositeConstruct {
                    result_id,
                    result_type,
                    constituents,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    let values = constituents
                        .iter()
                        .map(Variable::from_spirv)
                        .collect::<Vec<_>>();
                    instructions.push(Instruction::StoreVariableArray { dst: id, values });
                }
                spirv::Instruction::Variable {
                    result_id,
                    result_type,
                    storage_class: _,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                }
                spirv::Instruction::Select {
                    result_id,
                    result_type,
                    condition,
                    object1,
                    object2,
                } => {
                    let decl =
                        Self::get_variable_decl(&spirv, result_type, VariableBacking::Memory);
                    let id = Variable::from_spirv(result_id);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::Select {
                        id,
                        cond: Variable::from_spirv(condition),
                        obj1: Variable::from_spirv(object1),
                        obj2: Variable::from_spirv(object2),
                    });
                }
                spirv::Instruction::SelectionMerge { merge_block } => {
                    instructions.push(Instruction::SelectionMerge {
                        merge_block_label: merge_block.0,
                    });
                }
                spirv::Instruction::LoopMerge {
                    merge_block,
                    continue_target_label,
                } => {
                    instructions.push(Instruction::LoopMerge {
                        merge_block_label: merge_block.0,
                        continue_target_label: continue_target_label.0,
                    });
                }
                spirv::Instruction::Branch { target_label } => {
                    instructions.push(Instruction::Branch {
                        target_label: target_label.0,
                    });
                }
                spirv::Instruction::BranchConditional {
                    condition,
                    true_label,
                    false_label,
                } => {
                    instructions.push(Instruction::BranchConditional {
                        condition: Variable::from_spirv(condition),
                        true_label: true_label.0,
                        false_label: false_label.0,
                    });
                }
                spirv::Instruction::Return => {
                    instructions.push(Instruction::Return);
                }
                spirv::Instruction::Kill => {
                    instructions.push(Instruction::Kill);
                }
            }
        }
        Ok(instructions)
    }

    fn get_spirv_type<'a>(spirv: &'a Spirv, id: &spirv::ObjectId) -> &'a spirv::Type {
        match spirv.objects.get(id) {
            Some(spirv::Object::Type(inner)) => inner,
            _ => unreachable!(),
        }
    }

    fn get_spirv_constant<'a>(spirv: &'a Spirv, id: &spirv::ObjectId) -> &'a spirv::Constant {
        match spirv.objects.get(id) {
            Some(spirv::Object::Constant(inner)) => inner,
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    fn get_spirv_variable<'a>(spirv: &'a Spirv, id: &spirv::ObjectId) -> &'a spirv::Variable {
        match spirv.objects.get(id) {
            Some(spirv::Object::Variable(inner)) => inner,
            _ => unreachable!(),
        }
    }

    const fn from_spirv_decorations(decorations: &spirv::Decorations) -> VariableBacking {
        if let Some(builtin) = decorations.builtin {
            match builtin {
                spirv::BuiltInDecoration::Position => VariableBacking::Position,
                spirv::BuiltInDecoration::PointSize => VariableBacking::PointSize,
                spirv::BuiltInDecoration::VertexIndex => VariableBacking::VertexIndex,
                spirv::BuiltInDecoration::FragCoord => VariableBacking::FragCoord,
                spirv::BuiltInDecoration::ClipDistance => VariableBacking::ClipDistance,
                spirv::BuiltInDecoration::CullDistance => VariableBacking::CullDistance,
            }
        } else if let Some(location) = decorations.location {
            VariableBacking::Location {
                number: location.number,
            }
        } else {
            VariableBacking::Memory
        }
    }

    fn get_variable_decl(
        spirv: &Spirv,
        type_id: &spirv::ObjectId,
        backing: VariableBacking,
    ) -> VariableDecl {
        let (kind, component_count, backing) = match Self::get_spirv_type(spirv, type_id) {
            spirv::Type::Float { width } => {
                assert_eq!(*width, 32);
                (VariableKind::F32, 1, backing)
            }
            spirv::Type::Int { width, signedness } => {
                assert_eq!(*width, 32);
                if !signedness {
                    (VariableKind::U32, 1, backing)
                } else {
                    (VariableKind::I32, 1, backing)
                }
            }
            spirv::Type::Void => (VariableKind::Void, 0, backing),
            spirv::Type::Bool => (VariableKind::Bool, 1, backing),
            spirv::Type::Function(_) => {
                unimplemented!()
            }
            spirv::Type::Array {
                element_type,
                length,
                decorations,
            } => {
                let element_type = Self::get_variable_decl(spirv, element_type, backing);
                let &spirv::Constant::Scalar {
                    type_: _,
                    value: length,
                } = Self::get_spirv_constant(spirv, length)
                else {
                    unreachable!()
                };
                let array_stride = match decorations.array_stride {
                    None if length == 1 => std::mem::size_of::<u32>() as u32,
                    Some(inner) => inner,
                    None => 0,
                };
                (
                    VariableKind::Array,
                    length,
                    VariableBacking::Array {
                        element_kind: Box::new(element_type),
                        array_stride,
                    },
                )
            }
            spirv::Type::Vector {
                component_type,
                component_count,
            } => {
                let component_type = Self::get_variable_decl(spirv, component_type, backing);
                if component_type.component_count == 1 {
                    (
                        component_type.kind,
                        *component_count,
                        component_type.backing,
                    )
                } else {
                    unimplemented!("Matrix?")
                }
            }
            spirv::Type::Struct {
                member_types,
                decorations,
            } => {
                let members = member_types
                    .iter()
                    .map(|spirv::MemberType { type_, decorations }| {
                        Self::get_variable_decl(
                            spirv,
                            type_,
                            Self::from_spirv_decorations(decorations),
                        )
                    })
                    .collect();
                assert!(decorations.block);
                (
                    VariableKind::Struct,
                    member_types.len() as u32,
                    VariableBacking::Struct { members },
                )
            }
            spirv::Type::Pointer {
                storage_class: _,
                type_,
            } => {
                let type_ = Self::get_variable_decl(spirv, type_, backing);
                (
                    VariableKind::Pointer,
                    1,
                    VariableBacking::Pointer {
                        kind: Box::new(type_),
                    },
                )
            }
        };

        VariableDecl {
            kind,
            component_count,
            backing,
        }
    }
}

impl From<&VariableDecl> for Variable {
    fn from(_decl: &VariableDecl) -> Self {
        todo!()
    }
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
