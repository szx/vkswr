use crate::Vertex;
use log::debug;
use rspirv::binary::Disassemble;
use rspirv::spirv::ExecutionModel;

#[derive(Debug, Clone, Default)]
pub struct ShaderState {
    pub vertex_shader: Option<Shader>,
    pub fragment_shader: Option<Shader>,
}

#[derive(Debug, Clone, Default)]
pub struct Shader {
    name: String,
    module: rspirv::dr::Module,
}

impl Shader {
    pub fn new(name: &str, code: Vec<u32>) -> Option<Self> {
        let mut loader = rspirv::dr::Loader::new();
        assert_eq!(rspirv::spirv::MAGIC_NUMBER, code[0]);
        rspirv::binary::parse_words(&code, &mut loader)
            .map_err(|e| debug!("spriv error: {:#?}\nname: {:?}\ncode: {:?}", e, name, code))
            .ok()?;
        let module = loader.module();
        debug!("spirv shader:\n{}", module.disassemble());
        Some(Self {
            name: name.into(),
            module,
        })
    }

    pub fn execute_vertex_shader(&self, vertices: Vec<Vertex>) -> Vec<Vertex> {
        assert_eq!(self.module.entry_points.len(), 1);
        let entry_point = self
            .module
            .entry_points
            .first()
            .unwrap_or_else(|| unreachable!());
        assert_eq!(
            entry_point.operands[0].unwrap_execution_model(),
            ExecutionModel::Vertex
        );

        // TODO: "Execute vertex shader";
        vertices
    }
}

#[test]
fn execute_vertex_shader_points() {
    // Vertex shader taken from dEQP-VK.memory.pipeline_barrier.host_write_vertex_buffer.1024_vertex_buffer_stride_2
    let code = vec![
        119734787, 65536, 524299, 32, 0, 131089, 1, 393227, 1, 1280527431, 1685353262, 808793134,
        0, 196622, 0, 1, 458767, 0, 4, 1852399981, 0, 10, 20, 327752, 8, 0, 11, 0, 327752, 8, 1,
        11, 1, 196679, 8, 2, 262215, 20, 30, 0, 131091, 2, 196641, 3, 2, 196630, 6, 32, 262167, 7,
        6, 4, 262174, 8, 7, 6, 262176, 9, 3, 8, 262203, 9, 10, 3, 262165, 11, 32, 1, 262187, 11,
        12, 1, 262187, 6, 13, 1065353216, 262176, 14, 3, 6, 262187, 11, 16, 0, 262187, 6, 17,
        1073725047, 262167, 18, 6, 2, 262176, 19, 1, 18, 262203, 19, 20, 1, 262187, 6, 23,
        1065336439, 327724, 18, 24, 23, 23, 262187, 6, 26, 0, 262176, 30, 3, 7, 327734, 2, 4, 0, 3,
        131320, 5, 327745, 14, 15, 10, 12, 196670, 15, 13, 262205, 18, 21, 20, 327822, 18, 22, 21,
        17, 327811, 18, 25, 22, 24, 327761, 6, 27, 25, 0, 327761, 6, 28, 25, 1, 458832, 7, 29, 27,
        28, 26, 13, 327745, 30, 31, 10, 16, 196670, 31, 29, 65789, 65592,
    ];
    let shader = Shader::new("main", code).expect("shader should compile");

    // R8G8
    let inputs = vec![
        Vertex::from_raw(144, 245, 0, 0),
        Vertex::from_raw(79, 68, 0, 0),
        Vertex::from_raw(136, 146, 0, 0),
    ];
    dbg!(inputs[0].get_as_uint8(0));
    dbg!(inputs[0].get_as_unorm8(0));
    dbg!(inputs[0].get_as_uint8(1));
    dbg!(inputs[0].get_as_unorm8(1));

    let outputs = vec![
        Vertex::from_raw(144, 245, 0, 0),
        Vertex::from_raw(79, 68, 0, 0),
        Vertex::from_raw(136, 146, 0, 0),
    ];

    assert_eq!(true, false);
}
