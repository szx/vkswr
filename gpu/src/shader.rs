use crate::spirv::Spirv;
use crate::{
    spirv, Format, Position, Vertex, VertexAttribute, VertexBinding, VertexBindingNumber,
    VertexInputRate, VertexInputState, MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_BINDINGS,
};
use hashbrown::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct ShaderState {
    pub vertex_shader: Option<Shader>,
    pub fragment_shader: Option<Shader>,
}

#[derive(Debug, Clone)]
pub struct Shader {
    instructions: Vec<Instruction>,
}

impl Shader {
    pub fn new(name: &str, code: Vec<u32>) -> Option<Self> {
        let spirv = Spirv::new(name, code)?;
        let instructions = spirv.shader_instructions()?;
        Some(Self { instructions })
    }

    pub fn empty() -> Self {
        Self {
            instructions: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub kind: VariableKind,
    pub component_count: u32,
    pub storage: VariableStorage,
    pub backing: VariableBacking,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct VariableId(pub u32);

#[derive(Debug, Copy, Clone)]
pub enum VariableKind {
    F32,
    U32,
    I32,
    Void,
    Struct,
    Pointer,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VariableStorage {
    Constant,
    Variable,
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub enum VariableBacking {
    Memory, // TODO: Fold in VariableStorage?
    Location { number: u32 },
    Position,
    PointSize,
    Struct { members: Arc<[VariableDecl]> },
    Pointer { kind: Box<VariableDecl> },
}

#[derive(Debug)]
struct Variable {
    pub allocation: VariableAllocation,
}

#[derive(Debug, Clone)]
enum VariableAllocation {
    Location { number: u32 },
    BuiltinPosition,
    BuiltinPointSize,
    Imm32(Vec<u32>),
    Struct(Vec<VariableAllocation>),
    Pointer(Box<VariableAllocation>),
}

impl From<&VariableDecl> for VariableAllocation {
    fn from(decl: &VariableDecl) -> Self {
        match &decl.backing {
            VariableBacking::Memory => {
                let len = decl.component_count as usize;
                match decl.kind {
                    VariableKind::F32 => VariableAllocation::Imm32(vec![0; len]),
                    VariableKind::U32 => VariableAllocation::Imm32(vec![0; len]),
                    VariableKind::I32 => VariableAllocation::Imm32(vec![0; len]),
                    VariableKind::Void => unimplemented!(),
                    VariableKind::Struct => unimplemented!(),
                    VariableKind::Pointer => unimplemented!(),
                }
            }
            VariableBacking::Location { number } => {
                VariableAllocation::Location { number: *number }
            }
            VariableBacking::Position => VariableAllocation::BuiltinPosition,
            VariableBacking::PointSize => VariableAllocation::BuiltinPointSize,
            VariableBacking::Struct { members } => {
                VariableAllocation::Struct(members.iter().map(|x| x.into()).collect())
            }
            VariableBacking::Pointer { kind } => {
                VariableAllocation::Pointer(Box::new((kind.as_ref()).into()))
            }
        }
    }
}

impl Variable {
    fn from_decl(decl: &VariableDecl) -> Self {
        Self {
            allocation: decl.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
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
    Return,
}

impl Shader {
    pub fn execute_vertex_shader(
        &self,
        vertex_input_state: &VertexInputState,
        vertices: Vec<Vertex>,
    ) -> Vec<VertexShaderOutput> {
        // TODO: Create shader input/output interfaces, check if match between stages.

        let mut outputs: Vec<VertexShaderOutput> = vec![];

        for vertex in vertices {
            let mut output = VertexShaderOutput::default();

            let mut pc = 0_usize;
            let mut labels = HashMap::new();
            let mut variables = HashMap::new();

            loop {
                let instruction = &self.instructions[pc];
                match instruction {
                    Instruction::Label { id } => {
                        labels.try_insert(*id, pc).unwrap();
                    }
                    Instruction::VariableDecl { id, decl } => {
                        variables
                            .try_insert(*id, Variable::from_decl(decl))
                            .unwrap();
                    }
                    Instruction::StoreImm32 { dst, imm } => {
                        let dst = variables.get_mut(dst).unwrap();
                        match &mut dst.allocation {
                            VariableAllocation::Imm32(dst) => {
                                dst[0] = *imm;
                            }
                            _ => unimplemented!(),
                        }
                    }
                    Instruction::StoreImm32Array { dst, imm } => {
                        let dst = variables.get_mut(dst).unwrap();
                        match &mut dst.allocation {
                            VariableAllocation::Imm32(dst) => {
                                dst.copy_from_slice(imm.as_slice());
                            }
                            _ => unimplemented!(),
                        }
                    }
                    Instruction::LoadVariableOffset { id, base, offset } => {
                        let offset = variables.get(offset).unwrap();
                        let offset = match &offset {
                            Variable {
                                allocation: VariableAllocation::Imm32(offset),
                            } => offset[0],
                            _ => {
                                unimplemented!()
                            }
                        };

                        let base = variables.get(base).unwrap();
                        let VariableAllocation::Pointer(base) = &base.allocation else {
                            unimplemented!()
                        };
                        let VariableAllocation::Struct(members) = base.as_ref() else {
                            unimplemented!()
                        };
                        let member = members[offset as usize].clone();

                        let result = variables.get_mut(id).unwrap();
                        result.allocation = member;
                    }
                    Instruction::LoadVariableImmOffset { id, base, offset } => {
                        let offset = *offset;

                        let base = variables.get(base).unwrap();
                        let VariableAllocation::Imm32(base) = &base.allocation else {
                            unimplemented!()
                        };
                        let extracted = *base.get(offset as usize).unwrap();

                        let result = variables.get_mut(id).unwrap();
                        let VariableAllocation::Imm32(result) = &mut result.allocation else {
                            unimplemented!()
                        };
                        result[0] = extracted;
                    }
                    Instruction::StoreVariable { dst_pointer, src } => {
                        let [src, dst_pointer] =
                            variables.get_many_mut([src, dst_pointer]).unwrap();
                        match &mut dst_pointer.allocation {
                            VariableAllocation::Location { .. } => {
                                unimplemented!()
                            }
                            VariableAllocation::BuiltinPosition => {
                                let VariableAllocation::Imm32(imm) = &src.allocation else {
                                    unimplemented!()
                                };
                                output.position = Position::from_raw(
                                    imm.get(0)
                                        .map_or(output.position.get_as_uint64(0), |x| *x as u64),
                                    imm.get(1)
                                        .map_or(output.position.get_as_uint64(1), |x| *x as u64),
                                    imm.get(2)
                                        .map_or(output.position.get_as_uint64(2), |x| *x as u64),
                                    imm.get(3)
                                        .map_or(output.position.get_as_uint64(3), |x| *x as u64),
                                );
                            }
                            VariableAllocation::BuiltinPointSize => {
                                let VariableAllocation::Imm32(imm) = &src.allocation else {
                                    unimplemented!()
                                };
                                output.point_size = f32::from_bits(imm[0]);
                            }
                            VariableAllocation::Imm32(_) => {
                                unimplemented!()
                            }
                            VariableAllocation::Struct(_) => {
                                unimplemented!()
                            }
                            VariableAllocation::Pointer(_) => {
                                unimplemented!()
                            }
                        }
                    }
                    Instruction::StoreVariableArray { dst, values } => {
                        let values = values
                            .iter()
                            .map(|x| match variables.get(x) {
                                Some(Variable {
                                    allocation: VariableAllocation::Imm32(imm),
                                }) => imm[0],
                                _ => unreachable!(),
                            })
                            .collect::<Vec<_>>();

                        let dst = variables.get_mut(dst).unwrap();
                        let VariableAllocation::Imm32(dst) = &mut dst.allocation else {
                            unreachable!()
                        };

                        dst[..values.len()].copy_from_slice(&values[..]);
                    }
                    Instruction::LoadVariable { id, src_pointer } => {
                        let [result, src_pointer] =
                            variables.get_many_mut([id, src_pointer]).unwrap();
                        let VariableAllocation::Pointer(src) = &src_pointer.allocation else {
                            unreachable!()
                        };
                        let src = src.as_ref();

                        match &mut result.allocation {
                            VariableAllocation::Location { .. } => unimplemented!(),
                            VariableAllocation::BuiltinPosition => unimplemented!(),
                            VariableAllocation::BuiltinPointSize => unimplemented!(),
                            VariableAllocation::Imm32(imm) => match src {
                                VariableAllocation::Location { number } => {
                                    assert_eq!(*number, 0);
                                    let format = vertex_input_state
                                        .attributes
                                        .get(*number as usize)
                                        .unwrap()
                                        .unwrap()
                                        .format;
                                    // TODO: Use format.
                                    for (i, imm) in imm.iter_mut().enumerate() {
                                        *imm = vertex.get_as_unorm8(i).to_bits();
                                    }
                                }
                                VariableAllocation::BuiltinPosition => unimplemented!(),
                                VariableAllocation::BuiltinPointSize => unimplemented!(),
                                VariableAllocation::Imm32(_) => unimplemented!(),
                                VariableAllocation::Struct(_) => unimplemented!(),
                                VariableAllocation::Pointer(_) => unimplemented!(),
                            },
                            VariableAllocation::Struct(_) => unimplemented!(),
                            VariableAllocation::Pointer(_) => unimplemented!(),
                        }
                    }
                    Instruction::MathMulVectorScalar { id, vector, scalar } => {
                        let [result, vector, scalar] =
                            variables.get_many_mut([id, vector, scalar]).unwrap();
                        let VariableAllocation::Imm32(result) = &mut result.allocation else {
                            unreachable!()
                        };
                        let VariableAllocation::Imm32(vector) = &vector.allocation else {
                            unreachable!()
                        };
                        assert_eq!(result.len(), vector.len());
                        let VariableAllocation::Imm32(scalar) = &scalar.allocation else {
                            unreachable!()
                        };
                        let &[scalar] = scalar.as_slice() else {
                            unreachable!()
                        };
                        for i in 0..result.len() {
                            // TODO:  Replace Imm32 with Imm32(F32/I32/U32), use to determine casts.
                            result[i] =
                                (f32::from_bits(vector[i]) * f32::from_bits(scalar)).to_bits();
                        }
                    }
                    Instruction::MathSubF32F32 { id, op1, op2 } => {
                        let [result, op1, op2] = variables.get_many_mut([id, op1, op2]).unwrap();
                        let VariableAllocation::Imm32(result) = &mut result.allocation else {
                            unreachable!()
                        };
                        let VariableAllocation::Imm32(op1) = &mut op1.allocation else {
                            unreachable!()
                        };
                        let VariableAllocation::Imm32(op2) = &mut op2.allocation else {
                            unreachable!()
                        };
                        for i in 0..result.len() {
                            result[i] = (f32::from_bits(op1[i]) - f32::from_bits(op2[i])).to_bits();
                        }
                    }
                    Instruction::Return => {
                        break;
                    }
                }
                pc += 1;
            }

            outputs.push(output);
        }
        outputs
    }
}

/// Associates vertex shader input variables and vertex input attributes.
/// Vertex shader input variables listed by OpEntryPoint with the Input storage class
// These variables must be identified with a Location decoration and can also be identified with a Component decoration.
#[derive(Debug, Clone)]
pub struct VertexInputInterface {
    entry_point: spirv::EntryPoint,
}

#[derive(Debug, Copy, Clone)]
pub struct VertexShaderOutput {
    // gl_Position
    pub position: Position,
    // gl_PointSize
    pub point_size: f32,
    // TODO: Determine shader output interface using OpEntryPoints and use it to initialize ShaderOutput
    //       https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces
}

impl Default for VertexShaderOutput {
    fn default() -> Self {
        Self {
            position: Position::from_sfloat32_raw(0.0, 0.0, 0.0, 0.0),
            point_size: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_shader_points() {
        // Vertex shader taken from dEQP-VK.memory.pipeline_barrier.host_write_vertex_buffer.1024_vertex_buffer_stride_2
        // Inputs and outputs dumped used RenderDoc.
        let code = vec![
            119734787, 65536, 524299, 32, 0, 131089, 1, 393227, 1, 1280527431, 1685353262,
            808793134, 0, 196622, 0, 1, 458767, 0, 4, 1852399981, 0, 10, 20, 327752, 8, 0, 11, 0,
            327752, 8, 1, 11, 1, 196679, 8, 2, 262215, 20, 30, 0, 131091, 2, 196641, 3, 2, 196630,
            6, 32, 262167, 7, 6, 4, 262174, 8, 7, 6, 262176, 9, 3, 8, 262203, 9, 10, 3, 262165, 11,
            32, 1, 262187, 11, 12, 1, 262187, 6, 13, 1065353216, 262176, 14, 3, 6, 262187, 11, 16,
            0, 262187, 6, 17, 1073725047, 262167, 18, 6, 2, 262176, 19, 1, 18, 262203, 19, 20, 1,
            262187, 6, 23, 1065336439, 327724, 18, 24, 23, 23, 262187, 6, 26, 0, 262176, 30, 3, 7,
            327734, 2, 4, 0, 3, 131320, 5, 327745, 14, 15, 10, 12, 196670, 15, 13, 262205, 18, 21,
            20, 327822, 18, 22, 21, 17, 327811, 18, 25, 22, 24, 327761, 6, 27, 25, 0, 327761, 6,
            28, 25, 1, 458832, 7, 29, 27, 28, 26, 13, 327745, 30, 31, 10, 16, 196670, 31, 29,
            65789, 65592,
        ];

        let shader = Shader::new("main", code).expect("shader should compile");

        let mut vertex_input_state = VertexInputState {
            attributes: [None; MAX_VERTEX_ATTRIBUTES as usize],
            bindings: [None; MAX_VERTEX_BINDINGS as usize],
        };
        vertex_input_state.attributes[0] = Some(VertexAttribute {
            location: 0,
            binding: VertexBindingNumber(0),
            format: Format::R8G8Unorm,
            offset: 0,
        });
        vertex_input_state.bindings[0] = Some(VertexBinding {
            number: VertexBindingNumber(0),
            stride: 0,
            input_rate: VertexInputRate::Vertex,
        });

        // TODO: Test associating vertex input state to vertex input interface.

        // R8G8 vec2
        let inputs = vec![
            Vertex::from_raw(101, 228, 0, 0),
            Vertex::from_raw(161, 201, 0, 0),
            Vertex::from_raw(243, 120, 0, 0),
        ];

        // gl_Position vec4, gl_PointSize float
        let references = vec![
            (
                Vertex::from_raw(3193216600, 1061787170, 0, 1065353216),
                1.0f32,
            ),
            (
                Vertex::from_raw(1048994840, 1058237901, 0, 1065353216),
                1.0f32,
            ),
            (
                Vertex::from_raw(1063758986, 3178279746, 0, 1065353216),
                1.0f32,
            ),
        ];

        let outputs = shader.execute_vertex_shader(&vertex_input_state, inputs);

        let eps = 0.00001f32; // TODO: Use ULP (units in the last place) as defined in Vulkan spec?
        for (output, (position, point_size)) in outputs.iter().zip(references) {
            // TODO: Refactor Vector4: operation overloading.
            assert!((output.position.get_as_sfloat32(0) - position.get_as_sfloat32(0)).abs() < eps);
            assert!((output.position.get_as_sfloat32(1) - position.get_as_sfloat32(1)).abs() < eps);
            assert!((output.position.get_as_sfloat32(2) - position.get_as_sfloat32(2)).abs() < eps);
            assert!((output.position.get_as_sfloat32(3) - position.get_as_sfloat32(3)).abs() < eps);
            assert_eq!(output.point_size, point_size);
        }
    }
}
