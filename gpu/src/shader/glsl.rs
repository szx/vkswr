use crate::shader::interpreter::Interpreter;
use crate::{Color, Fragment, Position, Vertex, VertexInputState, MAX_CLIP_DISTANCES};

#[derive(Debug, Clone, Default)]
pub struct ShaderState {
    pub vertex_shader: Option<Shader>,
    pub fragment_shader: Option<Shader>,
}

#[derive(Debug, Clone)]
pub struct Shader {
    pub(crate) interpreter: Interpreter,
}

impl Shader {
    pub fn new(name: &str, code: Vec<u32>) -> anyhow::Result<Self> {
        Ok(Self {
            interpreter: Interpreter::new(name, code)?,
        })
    }
}

impl Shader {
    pub fn execute_vertex_shader(
        &self,
        vertex_input_state: &VertexInputState,
        vertices: Vec<Vertex>,
    ) -> Vec<VertexShaderOutput> {
        self.interpreter
            .execute_vertex_shader(vertex_input_state, vertices)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VertexShaderOutput {
    // gl_Position
    pub position: Position,
    // gl_PointSize
    pub point_size: f32,
    // gl_VertexIndex
    pub vertex_index: u32,
    // gl_ClipDistances
    pub clip_distances: [f32; MAX_CLIP_DISTANCES as usize],
    // TODO: Determine shader output interface using OpEntryPoints and use it to initialize ShaderOutput
    //       https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces
}

impl Default for VertexShaderOutput {
    fn default() -> Self {
        Self {
            position: Position::from_sfloat32_raw(0.0, 0.0, 0.0, 0.0),
            point_size: 1.0,
            vertex_index: 0,
            clip_distances: [0.0f32, 0.0f32, 0.0f32, 0.0f32],
        }
    }
}

impl From<Vertex> for VertexShaderOutput {
    fn from(vertex: Vertex) -> Self {
        Self {
            position: vertex.position,
            point_size: vertex.point_size,
            vertex_index: vertex.index,
            clip_distances: vertex.clip_distances,
        }
    }
}

impl Shader {
    pub fn execute_fragment_shader(&self, fragments: Vec<Fragment>) -> Vec<FragmentShaderOutput> {
        self.interpreter.execute_fragment_shader(fragments)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct FragmentShaderOutput {
    pub position: Position,
    pub color: Color,
}

impl From<Fragment> for FragmentShaderOutput {
    fn from(fragment: Fragment) -> Self {
        Self {
            position: fragment.position,
            color: fragment.color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Format, VertexAttribute, VertexBinding, VertexBindingNumber, VertexInputRate,
        MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_BINDINGS,
    };
    use std::process::Command;

    fn compile_glsl(stage: &str, glsl_code: &str) -> Vec<u32> {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let glsl_path = temp_dir.join(format!("shader.{stage}"));
        let spv_path = temp_dir.join(format!("{stage}.spv"));
        std::fs::write(&*glsl_path, glsl_code).unwrap();

        let mut out = Command::new("glslangValidator");
        let out = out.args([
            "-V",
            &glsl_path.to_string_lossy(),
            "-o",
            &spv_path.to_string_lossy(),
        ]);
        let out = out.current_dir(&temp_dir);
        let out = out.output().unwrap();
        let stdout = String::from_utf8_lossy(&out.stdout);
        println!("stdout:\n{}", stdout);
        assert!(
            out.status.success(),
            "Didn't pass: stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );

        let spv = std::fs::read(spv_path).unwrap();
        spv.chunks_exact(4)
            .map(|x| u32::from_ne_bytes(x.try_into().unwrap()))
            .collect::<Vec<_>>()
    }

    #[test]
    fn vertex_shader_set_output() {
        let spv = compile_glsl(
            "vert",
            r#"
            #version 450
            void main() {
                gl_Position = vec4(1,2,3,4);
                gl_ClipDistance[0] = 0.5f;
                gl_ClipDistance[2] = 0.9f;
            }
            "#,
        );
        let shader = Shader::new("main", spv).unwrap();
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
        let inputs = vec![Vertex {
            position: Position::from_raw(10, 20, 30, 40).to_unorm8(),
            point_size: 1.0f32,
            index: 1,
            clip_distances: [0.8f32, 0.4f32, 0.2f32, 0.1f32],
        }];
        let expected = inputs
            .iter()
            .map(|&x| VertexShaderOutput {
                position: Position::from_sfloat32_raw(1.0, 2.0, 3.0, 4.0),
                clip_distances: [0.5f32, 0.4f32, 0.9f32, 0.1f32],
                ..x.into()
            })
            .collect::<Vec<_>>();
        let outputs = shader
            .interpreter
            .execute_vertex_shader(&vertex_input_state, inputs);
        assert_eq!(outputs, expected);
    }

    #[test]
    fn vertex_shader_set_position() {
        let spv = compile_glsl(
            "vert",
            r#"
            #version 450
            void main() {
                gl_Position = vec4(1,2,3,4);
            }
            "#,
        );
        let shader = Shader::new("main", spv).unwrap();
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
        let inputs = vec![Vertex {
            position: Position::from_raw(10, 20, 30, 40).to_unorm8(),
            point_size: 1.0f32,
            index: 1,
            clip_distances: [0.8f32, 0.4f32, 0.2f32, 0.1f32],
        }];
        let expected = inputs
            .iter()
            .map(|&x| VertexShaderOutput {
                position: Position::from_sfloat32_raw(1.0, 2.0, 3.0, 4.0),
                ..x.into()
            })
            .collect::<Vec<_>>();
        let outputs = shader
            .interpreter
            .execute_vertex_shader(&vertex_input_state, inputs);
        assert_eq!(outputs, expected);
    }

    #[test]
    fn vertex_shader_empty() {
        let spv = compile_glsl(
            "vert",
            r#"
            #version 450
            void main() {
            }
            "#,
        );
        let shader = Shader::new("main", spv).unwrap();
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
        let inputs = vec![Vertex {
            position: Position::from_raw(10, 20, 30, 40).to_unorm8(),
            point_size: 1.0f32,
            index: 1,
            clip_distances: [0.0f32, 0.0f32, 0.0f32, 0.0f32],
        }];
        let expected = inputs.iter().map(|&x| x.into()).collect::<Vec<_>>();
        let outputs = shader
            .interpreter
            .execute_vertex_shader(&vertex_input_state, inputs);
        assert_eq!(outputs, expected);
    }

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
            Vertex {
                position: Position::from_raw(101, 228, 0, 0).to_unorm8(),
                point_size: 1.0f32,
                index: 0,
                clip_distances: Default::default(),
            },
            Vertex {
                position: Position::from_raw(161, 201, 0, 0).to_unorm8(),
                point_size: 1.0f32,
                index: 0,
                clip_distances: Default::default(),
            },
            Vertex {
                position: Position::from_raw(243, 120, 0, 0).to_unorm8(),
                point_size: 1.0f32,
                index: 0,
                clip_distances: Default::default(),
            },
        ];

        // gl_Position vec4, gl_PointSize float
        let references = vec![
            (
                Position::from_raw(3193216600, 1061787170, 0, 1065353216),
                1.0f32,
            ),
            (
                Position::from_raw(1048994840, 1058237901, 0, 1065353216),
                1.0f32,
            ),
            (
                Position::from_raw(1063758986, 3178279746, 0, 1065353216),
                1.0f32,
            ),
        ];

        let outputs = shader.execute_vertex_shader(&vertex_input_state, inputs);

        let eps = 0.00001f32; // TODO: Use ULP (units in the last place) as defined in Vulkan spec?
        for (output, (position, point_size)) in outputs.iter().zip(references) {
            // TODO: Refactor Vector4: operation overloading.
            assert!(
                (output.position.get_as_sfloat32(0) - position.get_as_sfloat32(0)).abs() < eps,
                "{} != {}",
                output.position.get_as_sfloat32(0),
                position.get_as_sfloat32(0)
            );
            assert!((output.position.get_as_sfloat32(1) - position.get_as_sfloat32(1)).abs() < eps);
            assert!((output.position.get_as_sfloat32(2) - position.get_as_sfloat32(2)).abs() < eps);
            assert!((output.position.get_as_sfloat32(3) - position.get_as_sfloat32(3)).abs() < eps);
            assert_eq!(output.point_size, point_size);
        }
    }
}
