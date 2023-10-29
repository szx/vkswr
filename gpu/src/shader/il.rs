use crate::shader::spirv;
use crate::shader::spirv::{ObjectId, Spirv};
use crate::{
    Fragment, FragmentShaderOutput, Position, Vertex, VertexInputState, VertexShaderOutput,
};
use hashbrown::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Il {
    pub(crate) instructions: Vec<Instruction>,
}

impl Il {
    pub fn new(name: &str, code: Vec<u32>) -> Option<Self> {
        let spirv = Spirv::new(name, code)?;
        let instructions = Self::parse_spirv(spirv)?;
        Some(Self { instructions })
    }

    pub fn execute_vertex_shader(
        &self,
        vertex_input_state: &VertexInputState,
        vertices: Vec<Vertex>,
    ) -> Vec<VertexShaderOutput> {
        // TODO: Create shader input/output interfaces, check if match between stages.
        let mut outputs: Vec<VertexShaderOutput> = vec![];

        for vertex in vertices {
            let mut state = State::new(vertex.into(), Default::default());

            loop {
                let instruction = &self.instructions[state.pc];
                let end = self.execute_instruction(instruction, &mut state);
                if end {
                    break;
                }
            }

            outputs.push(state.vertex_shader_output());
        }
        outputs
    }

    pub fn execute_fragment_shader(&self, fragments: Vec<Fragment>) -> Vec<FragmentShaderOutput> {
        let mut outputs: Vec<FragmentShaderOutput> = vec![];

        for fragment in fragments {
            let mut state = State::new(Default::default(), fragment.into());

            loop {
                let instruction = &self.instructions[state.pc];
                let end = self.execute_instruction(instruction, &mut state);
                if end {
                    break;
                }
            }

            outputs.push(state.fragment_shader_output());
        }
        outputs
    }
    fn execute_instruction(&self, instruction: &Instruction, state: &mut State) -> bool {
        match dbg!(instruction) {
            Instruction::Label { id } => {
                state.labels.try_insert(*id, state.pc).unwrap();
            }
            Instruction::VariableDecl { id, decl } => {
                state.add_new_variable(*id, decl);
            }
            Instruction::StoreImm32 { dst, imm } => {
                state.store_imm32(dst, &vec![*imm]);
            }
            Instruction::StoreImm32Array { dst, imm } => {
                state.store_imm32(dst, imm);
            }
            Instruction::LoadVariableOffset { id, base, offsets } => {
                state.load_variable_offset(id, base, offsets.clone());
            }
            Instruction::LoadVariableImmOffset { id, base, offset } => {
                state.load_variable_offset_imm(id, base, *offset);
            }
            Instruction::StoreVariable { dst_pointer, src } => {
                state.store_pointer(*dst_pointer, *src);
            }
            Instruction::StoreVariableArray { dst, values } => {
                state.store_array(dst, values);
            }
            Instruction::LoadVariable { id, src_pointer } => {
                state.load_variable(id, src_pointer);
            }
            Instruction::MathMulVectorScalar { id, vector, scalar } => {
                state.binary_op(id, vector, scalar, BinaryOpKind::MulVectorScalar);
            }
            Instruction::MathAddI32I32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::AddI32I32);
            }
            Instruction::MathMulI32I32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::MulI32I32);
            }
            Instruction::MathSubF32F32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::SubF32F32);
            }
            Instruction::MathDivF32F32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::DivF32F3);
            }
            Instruction::MathDivI32I32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::DivI32I32);
            }
            Instruction::MathDivU32U32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::DivU32U32);
            }
            Instruction::MathModI32I32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::ModI32I32);
            }
            Instruction::MathModU32U32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::ModU32U32);
            }
            Instruction::MathBitAnd { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::BitAnd);
            }
            Instruction::MathBitShiftRight { id, base, shift } => {
                state.binary_op(id, base, shift, BinaryOpKind::BitShiftRight);
            }
            Instruction::MathEqualI32I32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::EqualI32I32);
            }
            Instruction::MathLessThanU32U32 { id, op1, op2 } => {
                state.binary_op(id, op1, op2, BinaryOpKind::LessThanU32U32);
            }
            Instruction::MathConvertI32F32 { id, op } => {
                state.convert(id, op, ConvertKind::I32F32);
            }
            Instruction::MathConvertF32U32 { id, op } => {
                state.convert(id, op, ConvertKind::F32U32);
            }
            Instruction::MathConvertU32F32 { id, op } => {
                state.convert(id, op, ConvertKind::U32F32);
            }
            Instruction::Return => {
                return true;
            }
            Instruction::Select {
                id,
                cond,
                obj1,
                obj2,
            } => {
                todo!()
            }
            Instruction::SelectionMerge { .. } => {
                todo!()
            }
            Instruction::LoopMerge { .. } => {
                todo!()
            }
            Instruction::Branch { .. } => {
                todo!()
            }
            Instruction::BranchConditional { .. } => {
                todo!()
            }
            Instruction::Kill => {
                todo!()
            }
        };
        state.pc += 1;
        false
    }
}

#[derive(Debug, Default)]
struct State {
    pc: usize,
    labels: HashMap<u32, usize>,
    memory: Vec<u32>,
}

impl State {
    fn new(
        vertex_shader_output: VertexShaderOutput,
        fragment_shader_output: FragmentShaderOutput,
    ) -> Self {
        Self {
            pc: 0,
            labels: Default::default(),
            memory: vec![0_u32; 10000], // HIRO set memory with vertex_shader_output
        }
    }
    fn vertex_shader_output(&self) -> VertexShaderOutput {
        todo!()
    }

    fn fragment_shader_output(&self) -> FragmentShaderOutput {
        todo!()
    }
}

impl State {
    // HIRO move to interpreter.rs

    fn add_new_variable(&self, variable: Variable, decl: &VariableDecl) {
        todo!()
    }
    pub(crate) fn load_variable(&self, id: &Variable, src_pointer: &Variable) {
        todo!()
    }
    pub(crate) fn load_variable_offset(
        &self,
        id: &Variable,
        base: &Variable,
        offsets: Arc<[Variable]>,
    ) {
        todo!()
    }

    pub(crate) fn load_variable_offset_imm(&self, id: &Variable, base: &Variable, offset: u32) {
        todo!()
    }

    fn store(&mut self, dst: Variable, src: Variable) {
        todo!();
    }
    pub(crate) fn store_array(&self, dst_pointer: &Variable, src: &Vec<Variable>) {
        todo!()
    }
    pub(crate) fn store_pointer(&self, dst_pointer: Variable, str: Variable) {
        todo!()
    }
    pub(crate) fn store_imm32(&self, variable: &Variable, imm: &Vec<u32>) {
        todo!()
    }

    pub(crate) fn binary_op(
        &self,
        id: &Variable,
        op1: &Variable,
        op2: &Variable,
        kind: BinaryOpKind,
    ) {
        todo!()
    }

    pub(crate) fn convert(&self, id: &Variable, op: &Variable, kind: ConvertKind) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
struct Variable {
    address: u32,
    count: u32,
    stride: u32,
}

impl Variable {
    fn from_spirv(spirv_id: u32) -> Self {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
enum BinaryOpKind {
    MulVectorScalar,
    AddI32I32,
    MulI32I32,
    SubF32F32,
    DivF32F3,
    DivI32I32,
    DivU32U32,
    ModI32I32,
    ModU32U32,
    BitAnd,
    BitShiftRight,
    EqualI32I32,
    LessThanU32U32,
}

#[derive(Debug, Copy, Clone)]
enum ConvertKind {
    I32F32,
    F32U32,
    U32F32,
}

#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    Label {
        id: u32,
    },
    VariableDecl {
        id: Variable,
        decl: VariableDecl, // HIRO remove, replace with params (size, stride)
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
        offsets: Arc<[Variable]>,
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
    fn parse_spirv(spirv: Spirv) -> Option<Vec<Instruction>> {
        let mut scalar_variables = vec![];
        let mut composite_variables = vec![];
        let mut pointer_variables = vec![];

        for (id, object) in &spirv.objects {
            match object {
                spirv::Object::Type(_) => continue,
                spirv::Object::Constant(constant) => match constant {
                    spirv::Constant::Scalar { type_, value } => {
                        let decl = Self::get_variable_decl(
                            &spirv,
                            type_,
                            VariableStorage::Constant,
                            VariableBacking::Memory,
                        );
                        let id = Variable::from_spirv(id.0);
                        scalar_variables
                            .push(crate::shader::il::Instruction::VariableDecl { id, decl });
                        scalar_variables.push(crate::shader::il::Instruction::StoreImm32 {
                            dst: id,
                            imm: *value,
                        });
                    }
                    spirv::Constant::Composite {
                        type_,
                        constituents,
                    } => {
                        let decl = Self::get_variable_decl(
                            &spirv,
                            type_,
                            VariableStorage::Constant,
                            VariableBacking::Memory,
                        );
                        let id = Variable::from_spirv(id.0);
                        composite_variables
                            .push(crate::shader::il::Instruction::VariableDecl { id, decl });
                        let values = constituents
                            .iter()
                            .map(|id| match Self::get_spirv_constant(&spirv, id) {
                                spirv::Constant::Scalar { type_, value } => *value,
                                spirv::Constant::Composite { .. } => {
                                    unreachable!()
                                }
                            })
                            .collect::<Vec<_>>();
                        composite_variables.push(crate::shader::il::Instruction::StoreImm32Array {
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
                        Self::from_spirv_storage_class(&memory_object.storage_class),
                        Self::from_spirv_decorations(&memory_object.decorations),
                    );
                    let id = Variable::from_spirv(id.0);
                    pointer_variables.push(Instruction::VariableDecl { id, decl });
                }
            }
        }

        let mut instructions = vec![];
        instructions.extend(scalar_variables);
        instructions.extend(composite_variables);
        instructions.extend(pointer_variables);

        let main = spirv.functions.get(&spirv.entry_point.entry_point)?;
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
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::LoadVariableOffset {
                        id,
                        base: Variable::from_spirv(base.0),
                        offsets: indexes.iter().map(|x| Variable::from_spirv(x.0)).collect(),
                    });
                }
                spirv::Instruction::Store { pointer, object } => {
                    instructions.push(Instruction::StoreVariable {
                        dst_pointer: Variable::from_spirv(pointer.0),
                        src: Variable::from_spirv(object.0),
                    });
                }
                spirv::Instruction::Load {
                    result_id,
                    result_type,
                    pointer,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::LoadVariable {
                        id,
                        src_pointer: Variable::from_spirv(pointer.0),
                    });
                }
                spirv::Instruction::VectorTimesScalar {
                    result_id,
                    result_type,
                    vector,
                    scalar,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathMulVectorScalar {
                        id,
                        vector: Variable::from_spirv(vector.0),
                        scalar: Variable::from_spirv(scalar.0),
                    });
                }
                spirv::Instruction::IAdd {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathAddI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::IMul {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathMulI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::FSub {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathSubF32F32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::FDiv {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivF32F32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::SDiv {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::UDiv {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivU32U32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::SMod {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathModI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::UMod {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathModU32U32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::BitwiseAnd {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathBitAnd {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::ShiftRightLogical {
                    result_id,
                    result_type,
                    base,
                    shift,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathBitShiftRight {
                        id,
                        base: Variable::from_spirv(base.0),
                        shift: Variable::from_spirv(shift.0),
                    });
                }
                spirv::Instruction::IEqual {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathEqualI32I32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }

                spirv::Instruction::ULessThan {
                    result_id,
                    result_type,
                    operand1,
                    operand2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathLessThanU32U32 {
                        id,
                        op1: Variable::from_spirv(operand1.0),
                        op2: Variable::from_spirv(operand2.0),
                    });
                }
                spirv::Instruction::ConvertSToF {
                    result_id,
                    result_type,
                    operand,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathConvertI32F32 {
                        id,
                        op: Variable::from_spirv(operand.0),
                    });
                }
                spirv::Instruction::ConvertFToU {
                    result_id,
                    result_type,
                    operand,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathConvertF32U32 {
                        id,
                        op: Variable::from_spirv(operand.0),
                    });
                }
                spirv::Instruction::ConvertUToF {
                    result_id,
                    result_type,
                    operand,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathConvertU32F32 {
                        id,
                        op: Variable::from_spirv(operand.0),
                    });
                }
                spirv::Instruction::CompositeExtract {
                    result_id,
                    result_type,
                    composite,
                    indexes,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    if let [offset] = indexes.as_slice() {
                        instructions.push(Instruction::VariableDecl { id, decl });
                        instructions.push(Instruction::LoadVariableImmOffset {
                            id,
                            base: Variable::from_spirv(composite.0),
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
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    let values = constituents
                        .iter()
                        .map(|id| Variable::from_spirv(id.0))
                        .collect::<Vec<_>>();
                    instructions.push(Instruction::StoreVariableArray { dst: id, values });
                }
                spirv::Instruction::Variable {
                    result_id,
                    result_type,
                    storage_class,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        &result_type,
                        Self::from_spirv_storage_class(storage_class),
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                }
                spirv::Instruction::Select {
                    result_id,
                    result_type,
                    condition,
                    object1,
                    object2,
                } => {
                    let decl = Self::get_variable_decl(
                        &spirv,
                        result_type,
                        VariableStorage::Variable,
                        VariableBacking::Memory,
                    );
                    let id = Variable::from_spirv(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::Select {
                        id,
                        cond: Variable::from_spirv(condition.0),
                        obj1: Variable::from_spirv(object1.0),
                        obj2: Variable::from_spirv(object2.0),
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
                        condition: Variable::from_spirv(condition.0),
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
        Some(instructions)
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

    fn get_spirv_variable<'a>(spirv: &'a Spirv, id: &spirv::ObjectId) -> &'a spirv::Variable {
        match spirv.objects.get(id) {
            Some(spirv::Object::Variable(inner)) => inner,
            _ => unreachable!(),
        }
    }

    fn from_spirv_storage_class(value: &spirv::StorageClass) -> VariableStorage {
        match value {
            spirv::StorageClass::Input => VariableStorage::Input,
            spirv::StorageClass::Output => VariableStorage::Output,
            spirv::StorageClass::Function => VariableStorage::FunctionMemory,
            spirv::StorageClass::PushConstant => VariableStorage::PushConstant,
            spirv::StorageClass::Uniform => VariableStorage::Uniform,
        }
    }

    fn from_spirv_decorations(decorations: &spirv::Decorations) -> VariableBacking {
        if let Some(builtin) = decorations.builtin {
            match builtin {
                spirv::BuiltInDecoration::Position => VariableBacking::Position,
                spirv::BuiltInDecoration::PointSize => VariableBacking::PointSize,
                spirv::BuiltInDecoration::VertexIndex => VariableBacking::VertexIndex,
                spirv::BuiltInDecoration::FragCoord => VariableBacking::FragCoord,
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
        storage: VariableStorage,
        backing: VariableBacking,
    ) -> VariableDecl {
        let (kind, component_count, storage, backing) = match Self::get_spirv_type(spirv, type_id) {
            spirv::Type::Float { width } => {
                assert_eq!(*width, 32);
                (VariableKind::F32, 1, storage, backing)
            }
            spirv::Type::Int { width, signedness } => {
                assert_eq!(*width, 32);
                if !signedness {
                    (VariableKind::U32, 1, storage, backing)
                } else {
                    (VariableKind::I32, 1, storage, backing)
                }
            }
            spirv::Type::Void => (VariableKind::Void, 0, storage, backing),
            spirv::Type::Bool => (VariableKind::Bool, 1, storage, backing),
            spirv::Type::Function(_) => {
                unimplemented!()
            }
            spirv::Type::Array {
                element_type,
                length,
                decorations,
            } => {
                let element_type = Self::get_variable_decl(spirv, element_type, storage, backing);
                let &spirv::Constant::Scalar { type_: _, value } =
                    Self::get_spirv_constant(spirv, length)
                else {
                    unreachable!()
                };
                let Some(array_stride) = decorations.array_stride else {
                    unreachable!()
                };
                (
                    VariableKind::Struct,
                    value,
                    storage,
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
                let component_type =
                    Self::get_variable_decl(spirv, component_type, storage, backing);
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
                            storage,
                            Self::from_spirv_decorations(decorations),
                        )
                    })
                    .collect();
                assert!(decorations.block);
                (
                    VariableKind::Struct,
                    member_types.len() as u32,
                    storage,
                    VariableBacking::Struct { members },
                )
            }
            spirv::Type::Pointer {
                storage_class,
                type_,
            } => {
                let type_ = Self::get_variable_decl(
                    spirv,
                    type_,
                    Self::from_spirv_storage_class(storage_class),
                    backing,
                );
                (
                    VariableKind::Pointer,
                    1,
                    storage,
                    VariableBacking::Pointer {
                        kind: Box::new(type_),
                    },
                )
            }
        };

        VariableDecl {
            kind,
            component_count,
            storage,
            backing,
        }
    }
}

impl From<&VariableDecl> for Variable {
    fn from(decl: &VariableDecl) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VariableDecl {
    kind: VariableKind,
    component_count: u32,
    storage: VariableStorage,
    backing: VariableBacking,
}

#[derive(Debug, Copy, Clone)]
enum VariableKind {
    F32,
    U32,
    I32,
    Void,
    Bool,
    Array,
    Struct,
    Pointer,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum VariableStorage {
    Constant,
    Variable,
    Input,
    Output,
    FunctionMemory,
    PushConstant,
    Uniform,
}

#[derive(Debug, Clone)]
enum VariableBacking {
    Memory, // TODO: Fold in VariableStorage?
    Location {
        number: u32,
    },
    Position,
    PointSize,
    VertexIndex,
    FragCoord,
    Array {
        element_kind: Box<VariableDecl>,
        array_stride: u32,
    },
    Struct {
        members: Arc<[VariableDecl]>,
    },
    Pointer {
        kind: Box<VariableDecl>,
    },
}
