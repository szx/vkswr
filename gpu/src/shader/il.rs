use crate::shader::spirv;
use crate::shader::spirv::Spirv;
use crate::{
    Fragment, FragmentShaderOutput, Position, Vertex, VertexInputState, VertexShaderOutput,
};
use hashbrown::HashMap;
use std::sync::Arc;

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
            let mut state = State {
                vertex_shader_output: vertex.into(),
                ..Default::default()
            };

            loop {
                let instruction = &self.instructions[state.pc];
                let end = self.execute_instruction(instruction, &mut state);
                if end {
                    break;
                }
                state.pc += 1;
            }

            outputs.push(state.vertex_shader_output);
        }
        outputs
    }

    pub fn execute_fragment_shader(&self, fragments: Vec<Fragment>) -> Vec<FragmentShaderOutput> {
        let mut outputs: Vec<FragmentShaderOutput> = vec![];

        for fragment in fragments {
            let mut state = State {
                fragment_shader_output: fragment.into(),
                ..Default::default()
            };

            loop {
                let instruction = &self.instructions[state.pc];
                let end = self.execute_instruction(instruction, &mut state);
                if end {
                    break;
                }
                state.pc += 1;
            }

            outputs.push(state.fragment_shader_output);
        }
        outputs
    }
    fn execute_instruction(&self, instruction: &Instruction, state: &mut State) -> bool {
        match instruction {
            Instruction::Label { id } => {
                state.labels.try_insert(*id, state.pc).unwrap();
            }
            Instruction::VariableDecl { id, decl } => {
                state.variables.try_insert(*id, decl.into()).unwrap();
            }
            Instruction::StoreImm32 { dst, imm } => {
                let dst = state.variables.get_mut(dst).unwrap();
                match dst {
                    Variable::Imm32(dst) => {
                        dst[0] = *imm;
                    }
                    _ => unimplemented!(),
                }
            }
            Instruction::StoreImm32Array { dst, imm } => {
                let dst = state.variables.get_mut(dst).unwrap();
                match dst {
                    Variable::Imm32(dst) => {
                        dst.copy_from_slice(imm.as_slice());
                    }
                    _ => unimplemented!(),
                }
            }
            Instruction::LoadVariableOffset { id, base, offset } => {
                let offset = state.variables.get(offset).unwrap();
                let offset = match &offset {
                    Variable::Imm32(offset) => offset[0],
                    _ => {
                        unimplemented!()
                    }
                };

                let base = state.variables.get(base).unwrap();
                let Variable::Pointer(base) = &base else {
                    unimplemented!()
                };
                let Variable::Struct(members) = base.as_ref() else {
                    unimplemented!()
                };
                let member = members[offset as usize].clone();

                let result = state.variables.get_mut(id).unwrap();
                *result = member;
            }
            Instruction::LoadVariableImmOffset { id, base, offset } => {
                let offset = *offset;

                let base = state.variables.get(base).unwrap();
                let Variable::Imm32(base) = &base else {
                    unimplemented!()
                };
                let extracted = *base.get(offset as usize).unwrap();

                let result = state.variables.get_mut(id).unwrap();
                let Variable::Imm32(result) = result else {
                    unimplemented!()
                };
                result[0] = extracted;
            }
            Instruction::StoreVariable { dst_pointer, src } => {
                let [src, dst_pointer] = state.variables.get_many_mut([src, dst_pointer]).unwrap();
                match dst_pointer {
                    Variable::Location { .. } => {
                        unimplemented!()
                    }
                    Variable::BuiltinPosition => {
                        let Variable::Imm32(imm) = &src else {
                            unimplemented!()
                        };
                        state.vertex_shader_output.position = Position::from_raw(
                            imm.get(0).map_or(
                                state.vertex_shader_output.position.get_as_uint64(0),
                                |x| *x as u64,
                            ),
                            imm.get(1).map_or(
                                state.vertex_shader_output.position.get_as_uint64(1),
                                |x| *x as u64,
                            ),
                            imm.get(2).map_or(
                                state.vertex_shader_output.position.get_as_uint64(2),
                                |x| *x as u64,
                            ),
                            imm.get(3).map_or(
                                state.vertex_shader_output.position.get_as_uint64(3),
                                |x| *x as u64,
                            ),
                        );
                    }
                    Variable::BuiltinPointSize => {
                        let Variable::Imm32(imm) = &src else {
                            unimplemented!()
                        };
                        state.vertex_shader_output.point_size = f32::from_bits(imm[0]);
                    }
                    Variable::BuiltinVertexIndex => {
                        unreachable!()
                    }
                    Variable::Imm32(_) => {
                        unimplemented!()
                    }
                    Variable::Struct(_) => {
                        unimplemented!()
                    }
                    Variable::Pointer(dst) => {
                        let Variable::Imm32(src) = &src else {
                            unimplemented!()
                        };
                        dbg!(&src);
                        dbg!(&dst);
                        match dst.as_mut() {
                            Variable::Location { number } => {
                                assert_eq!(*number, 0);
                                for (i, dst) in state
                                    .fragment_shader_output
                                    .color
                                    .components
                                    .iter_mut()
                                    .enumerate()
                                {
                                    *dst = src[i] as u64;
                                }
                            }
                            Variable::BuiltinPosition => {
                                unimplemented!()
                            }
                            Variable::BuiltinPointSize => {
                                unimplemented!()
                            }
                            Variable::BuiltinVertexIndex => {
                                unimplemented!()
                            }
                            Variable::Imm32(dst) => {
                                for (i, dst) in dst.iter_mut().enumerate() {
                                    *dst = src[i];
                                }
                            }
                            Variable::Struct(_) => {
                                unimplemented!()
                            }
                            Variable::Pointer(_) => {
                                unimplemented!()
                            }
                        }
                    }
                }
            }
            Instruction::StoreVariableArray { dst, values } => {
                let values = values
                    .iter()
                    .map(|x| match state.variables.get(x) {
                        Some(Variable::Imm32(imm)) => imm[0],
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();

                let mut dst = state.variables.get_mut(dst).unwrap();
                let Variable::Imm32(dst) = &mut dst else {
                    unreachable!()
                };

                dst[..values.len()].copy_from_slice(&values[..]);
            }
            Instruction::LoadVariable { id, src_pointer } => {
                let [result, src_pointer] =
                    state.variables.get_many_mut([id, src_pointer]).unwrap();
                let Variable::Pointer(src) = &src_pointer else {
                    unreachable!()
                };
                let src = src.as_ref();

                match result {
                    Variable::Location { .. } => unimplemented!(),
                    Variable::BuiltinPosition => unimplemented!(),
                    Variable::BuiltinPointSize => unimplemented!(),
                    Variable::BuiltinVertexIndex => unreachable!(),
                    Variable::Imm32(imm) => match src {
                        Variable::Location { number } => {
                            assert_eq!(*number, 0);
                            for (i, imm) in imm.iter_mut().enumerate() {
                                *imm = state.vertex_shader_output.position.get_as_uint32(i);
                            }
                        }
                        Variable::BuiltinPosition => unimplemented!(),
                        Variable::BuiltinPointSize => unimplemented!(),
                        Variable::BuiltinVertexIndex => {
                            for (i, imm) in imm.iter_mut().enumerate() {
                                *imm = state.vertex_shader_output.vertex_index;
                            }
                        }
                        Variable::Imm32(src_imm) => {
                            for (i, imm) in imm.iter_mut().enumerate() {
                                *imm = src_imm[i];
                            }
                        }
                        Variable::Struct(_) => unimplemented!(),
                        Variable::Pointer(_) => unimplemented!(),
                    },
                    Variable::Struct(_) => unimplemented!(),
                    Variable::Pointer(_) => unimplemented!(),
                }
            }
            Instruction::MathMulVectorScalar { id, vector, scalar } => {
                let [result, vector, scalar] =
                    state.variables.get_many_mut([id, vector, scalar]).unwrap();
                let Variable::Imm32(result) = result else {
                    unreachable!()
                };
                let Variable::Imm32(vector) = &vector else {
                    unreachable!()
                };
                assert_eq!(result.len(), vector.len());
                let Variable::Imm32(scalar) = &scalar else {
                    unreachable!()
                };
                let &[scalar] = scalar.as_slice() else {
                    unreachable!()
                };
                for i in 0..result.len() {
                    // TODO:  Replace Imm32 with Imm32(F32/I32/U32), use to determine casts.
                    result[i] = (f32::from_bits(vector[i]) * f32::from_bits(scalar)).to_bits();
                }
            }
            Instruction::MathSubF32F32 { id, op1, op2 } => {
                let [result, op1, op2] = state.variables.get_many_mut([id, op1, op2]).unwrap();
                let Variable::Imm32(result) = result else {
                    unreachable!()
                };
                let Variable::Imm32(op1) = &op1 else {
                    unreachable!()
                };
                let Variable::Imm32(op2) = &op2 else {
                    unreachable!()
                };
                for i in 0..result.len() {
                    result[i] = (f32::from_bits(op1[i]) - f32::from_bits(op2[i])).to_bits();
                }
            }
            Instruction::MathDivF32F32 { id, op1, op2 } => {
                let [result, op1, op2] = state.variables.get_many_mut([id, op1, op2]).unwrap();
                let Variable::Imm32(result) = result else {
                    unreachable!()
                };
                let Variable::Imm32(op1) = &op1 else {
                    unreachable!()
                };
                let Variable::Imm32(op2) = &op2 else {
                    unreachable!()
                };
                for i in 0..result.len() {
                    result[i] = (f32::from_bits(op1[i]) / f32::from_bits(op2[i])).to_bits();
                }
            }
            Instruction::MathDivI32I32 { id, op1, op2 } => {
                let [result, op1, op2] = state.variables.get_many_mut([id, op1, op2]).unwrap();
                let Variable::Imm32(result) = result else {
                    unreachable!()
                };
                let Variable::Imm32(op1) = &op1 else {
                    unreachable!()
                };
                let Variable::Imm32(op2) = &op2 else {
                    unreachable!()
                };
                for i in 0..result.len() {
                    result[i] =
                        u32::from_ne_bytes(((op1[i] as i32) / (op2[i] as i32)).to_ne_bytes());
                }
            }
            Instruction::MathModI32I32 { id, op1, op2 } => {
                let [result, op1, op2] = state.variables.get_many_mut([id, op1, op2]).unwrap();
                let Variable::Imm32(result) = result else {
                    unreachable!()
                };
                let Variable::Imm32(op1) = &op1 else {
                    unreachable!()
                };
                let Variable::Imm32(op2) = &op2 else {
                    unreachable!()
                };
                for i in 0..result.len() {
                    result[i] =
                        u32::from_ne_bytes(((op1[i] as i32) % (op2[i] as i32)).to_ne_bytes());
                }
            }
            Instruction::MathConvertI32F32 { id, op } => {
                let [result, op] = state.variables.get_many_mut([id, op]).unwrap();
                let Variable::Imm32(result) = result else {
                    unreachable!()
                };
                let Variable::Imm32(op) = &op else {
                    unreachable!()
                };
                for i in 0..result.len() {
                    result[i] = (op[i] as f32).to_bits();
                }
            }
            Instruction::Return => {
                return true;
            }
        };
        false
    }
}

#[derive(Debug, Default)]
struct State {
    pc: usize,
    labels: HashMap<u32, usize>,
    variables: HashMap<VariableId, Variable>,
    // TODO: Replace with inserting variable with builtin backing.
    vertex_shader_output: VertexShaderOutput,
    fragment_shader_output: FragmentShaderOutput,
}

#[derive(Debug, Clone)]
enum Variable {
    Location { number: u32 },
    BuiltinPosition,
    BuiltinPointSize,
    BuiltinVertexIndex,
    Imm32(Vec<u32>),
    Struct(Vec<Variable>),
    Pointer(Box<Variable>),
}

impl From<&VariableDecl> for Variable {
    fn from(decl: &VariableDecl) -> Self {
        match &decl.backing {
            VariableBacking::Memory => {
                let len = decl.component_count as usize;
                match decl.kind {
                    VariableKind::F32 => Variable::Imm32(vec![0; len]),
                    VariableKind::U32 => Variable::Imm32(vec![0; len]),
                    VariableKind::I32 => Variable::Imm32(vec![0; len]),
                    VariableKind::Void => unimplemented!(),
                    VariableKind::Struct => unimplemented!(),
                    VariableKind::Pointer => unimplemented!(),
                }
            }
            VariableBacking::Location { number } => Variable::Location { number: *number },
            VariableBacking::Position => Variable::BuiltinPosition,
            VariableBacking::PointSize => Variable::BuiltinPointSize,
            VariableBacking::VertexIndex => Variable::BuiltinVertexIndex,
            VariableBacking::Struct { members } => {
                Variable::Struct(members.iter().map(|x| x.into()).collect())
            }
            VariableBacking::Pointer { kind } => {
                Variable::Pointer(Box::new((kind.as_ref()).into()))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VariableDecl {
    kind: VariableKind,
    component_count: u32,
    storage: VariableStorage,
    backing: VariableBacking,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub(crate) struct VariableId(pub u32);

#[derive(Debug, Copy, Clone)]
enum VariableKind {
    F32,
    U32,
    I32,
    Void,
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
}

#[derive(Debug, Clone)]
enum VariableBacking {
    Memory, // TODO: Fold in VariableStorage?
    Location { number: u32 },
    Position,
    PointSize,
    VertexIndex,
    Struct { members: Arc<[VariableDecl]> },
    Pointer { kind: Box<VariableDecl> },
}

#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    Label {
        id: u32,
    },
    VariableDecl {
        id: VariableId,
        decl: VariableDecl,
    },
    StoreImm32 {
        dst: VariableId,
        imm: u32,
    },
    StoreImm32Array {
        dst: VariableId,
        imm: Vec<u32>,
    },
    LoadVariableOffset {
        id: VariableId,
        base: VariableId,
        offset: VariableId,
    },
    LoadVariableImmOffset {
        id: VariableId,
        base: VariableId,
        offset: u32,
    },
    StoreVariable {
        dst_pointer: VariableId,
        src: VariableId,
    },
    StoreVariableArray {
        dst: VariableId,
        values: Vec<VariableId>,
    },
    LoadVariable {
        id: VariableId,
        src_pointer: VariableId,
    },
    MathMulVectorScalar {
        id: VariableId,
        vector: VariableId,
        scalar: VariableId,
    },
    MathSubF32F32 {
        id: VariableId,
        op1: VariableId,
        op2: VariableId,
    },
    MathDivF32F32 {
        id: VariableId,
        op1: VariableId,
        op2: VariableId,
    },
    MathDivI32I32 {
        id: VariableId,
        op1: VariableId,
        op2: VariableId,
    },
    MathModI32I32 {
        id: VariableId,
        op1: VariableId,
        op2: VariableId,
    },
    MathConvertI32F32 {
        id: VariableId,
        op: VariableId,
    },
    Return,
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
                        let id = VariableId(id.0);
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
                        let id = VariableId(id.0);
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
                    let id = VariableId(id.0);
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
                    let id = VariableId(result_id.0);
                    if let [offset] = indexes.as_slice() {
                        instructions.push(Instruction::VariableDecl { id, decl });
                        instructions.push(Instruction::LoadVariableOffset {
                            id,
                            base: VariableId(base.0),
                            offset: VariableId(offset.0),
                        });
                    } else {
                        unimplemented!()
                    }
                }
                spirv::Instruction::Store { pointer, object } => {
                    instructions.push(Instruction::StoreVariable {
                        dst_pointer: VariableId(pointer.0),
                        src: VariableId(object.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::LoadVariable {
                        id,
                        src_pointer: VariableId(pointer.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathMulVectorScalar {
                        id,
                        vector: VariableId(vector.0),
                        scalar: VariableId(scalar.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathSubF32F32 {
                        id,
                        op1: VariableId(operand1.0),
                        op2: VariableId(operand2.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivF32F32 {
                        id,
                        op1: VariableId(operand1.0),
                        op2: VariableId(operand2.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathDivI32I32 {
                        id,
                        op1: VariableId(operand1.0),
                        op2: VariableId(operand2.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathModI32I32 {
                        id,
                        op1: VariableId(operand1.0),
                        op2: VariableId(operand2.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    instructions.push(Instruction::MathConvertI32F32 {
                        id,
                        op: VariableId(operand.0),
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
                    let id = VariableId(result_id.0);
                    if let [offset] = indexes.as_slice() {
                        instructions.push(Instruction::VariableDecl { id, decl });
                        instructions.push(Instruction::LoadVariableImmOffset {
                            id,
                            base: VariableId(composite.0),
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                    let values = constituents
                        .iter()
                        .map(|id| VariableId(id.0))
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
                    let id = VariableId(result_id.0);
                    instructions.push(Instruction::VariableDecl { id, decl });
                }
                spirv::Instruction::Return => {
                    instructions.push(Instruction::Return);
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
        }
    }

    fn from_spirv_decorations(decorations: &spirv::Decorations) -> VariableBacking {
        if let Some(builtin) = decorations.builtin {
            match builtin {
                spirv::BuiltInDecoration::Position => VariableBacking::Position,
                spirv::BuiltInDecoration::PointSize => VariableBacking::PointSize,
                spirv::BuiltInDecoration::VertexIndex => VariableBacking::VertexIndex,
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
            spirv::Type::Function(_) => {
                unimplemented!()
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
