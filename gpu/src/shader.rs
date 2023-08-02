use crate::Vertex;
use log::debug;
use rspirv::binary::Disassemble;

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
    pub fn new(name: String, code: Vec<u32>) -> Option<Self> {
        let mut loader = rspirv::dr::Loader::new();
        assert_eq!(rspirv::spirv::MAGIC_NUMBER, code[0]);
        rspirv::binary::parse_words(&code, &mut loader)
            .map_err(|e| debug!("spriv error: {:#?}\nname: {:?}\ncode: {:?}", e, name, code))
            .ok()?;
        let module = loader.module();
        debug!("spirv shader:\n{}", module.disassemble());
        Some(Self { name, module })
    }

    pub fn execute_vertex_shader(&self, vertices: Vec<Vertex>) -> Vec<Vertex> {
        // TODO: Execute vertex shader.
        vertices
    }
}
