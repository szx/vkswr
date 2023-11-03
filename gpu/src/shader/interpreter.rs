use crate::shader::il;
use crate::shader::il::Il;
use crate::{
    Format, Fragment, FragmentShaderOutput, Position, Vector4, Vertex, VertexInputState,
    VertexShaderOutput,
};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Interpreter {
    il: Il,
}

impl Interpreter {
    pub fn new(name: &str, code: Vec<u32>) -> Option<Self> {
        let il = Il::new(name, code)?;
        Some(Self { il })
    }
}

impl Interpreter {
    pub(crate) fn execute_vertex_shader(
        &self,
        vertex_input_state: &VertexInputState,
        vertices: Vec<Vertex>,
    ) -> Vec<VertexShaderOutput> {
        // TODO: Create shader input/output interfaces, check if match between stages.
        let vertex_format = vertex_input_state.attributes[0].unwrap().format;

        let mut outputs: Vec<VertexShaderOutput> = vec![];

        for vertex in vertices {
            let mut state = State::new();
            state.set_vertex_shader_input(vertex);

            loop {
                let instruction = &self.il.instructions[state.pc];
                let end = state.interpret_il_instruction(instruction);
                if end {
                    break;
                }
            }

            outputs.push(state.vertex_shader_output());
        }
        outputs
    }

    pub(crate) fn execute_fragment_shader(
        &self,
        fragments: Vec<Fragment>,
    ) -> Vec<FragmentShaderOutput> {
        let mut outputs: Vec<FragmentShaderOutput> = vec![];

        for fragment in fragments {
            let mut state = State::new();
            state.set_fragment_shader_input(fragment);

            loop {
                let instruction = &self.il.instructions[state.pc];
                let end = state.interpret_il_instruction(instruction);
                if end {
                    break;
                }
            }

            outputs.push(state.fragment_shader_output());
        }
        outputs
    }
}

#[derive(Debug)]
struct State {
    pc: usize,
    labels: HashMap<u32, usize>,
    memory: Vec<u8>,
}

impl State {
    fn new() -> Self {
        Self {
            pc: 0,
            labels: Default::default(),
            memory: vec![0_u8; 10000], // HIRO
        }
    }

    fn set_vertex_shader_input(&mut self, vertex: Vertex) {
        self.memory_mut(self.memory_region_of_built_in(BuiltIn::Position))
            .copy_from_slice(bytemuck::cast_slice(
                vertex.position.get_as_f32_array().as_slice(),
            ));
        self.memory_mut(self.memory_region_of_built_in(BuiltIn::PointSize))
            .copy_from_slice(bytemuck::cast_slice(&[f32::to_bits(1.0f32)]));
        self.memory_mut(self.memory_region_of_built_in(BuiltIn::VertexIndex))
            .copy_from_slice(bytemuck::cast_slice(&[vertex.index]));
    }

    fn vertex_shader_output(&self) -> VertexShaderOutput {
        let position = Vector4::from_vertex_buffer_bytes(
            Format::R32G32B32A32Sfloat,
            self.memory(self.memory_region_of_built_in(BuiltIn::Position)),
        );
        let point_size = *bytemuck::from_bytes::<f32>(
            self.memory(self.memory_region_of_built_in(BuiltIn::PointSize)),
        );
        let vertex_index = *bytemuck::from_bytes::<u32>(
            self.memory(self.memory_region_of_built_in(BuiltIn::VertexIndex)),
        );
        VertexShaderOutput {
            position,
            point_size,
            vertex_index,
        }
    }

    fn set_fragment_shader_input(&self, fragment: Fragment) {
        todo!()
    }

    fn fragment_shader_output(&mut self) -> FragmentShaderOutput {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
enum BuiltIn {
    Position,
    PointSize,
    VertexIndex,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MemoryRegion {
    address: usize,
    size: usize,
}

impl State {
    fn memory_region_of_built_in(&self, built_in: BuiltIn) -> MemoryRegion {
        match built_in {
            BuiltIn::Position => MemoryRegion {
                address: 0,
                size: std::mem::size_of::<f32>() * 4,
            },
            BuiltIn::PointSize => {
                let prev = self.memory_region_of_built_in(BuiltIn::Position);
                MemoryRegion {
                    address: prev.address + prev.size,
                    size: std::mem::size_of::<f32>(),
                }
            }
            BuiltIn::VertexIndex => {
                let prev = self.memory_region_of_built_in(BuiltIn::PointSize);
                MemoryRegion {
                    address: prev.address + prev.size,
                    size: std::mem::size_of::<u32>(),
                }
            }
        }
    }

    fn memory(&self, memory_region: MemoryRegion) -> &[u8] {
        &self.memory[memory_region.address..memory_region.address + memory_region.size]
    }

    fn memory_mut(&mut self, memory_region: MemoryRegion) -> &mut [u8] {
        &mut self.memory[memory_region.address..memory_region.address + memory_region.size]
    }
}

impl State {
    fn add_new_variable(&self, variable: il::Variable, decl: &il::VariableDecl) {
        todo!()
    }
    pub(crate) fn load_variable(&self, id: &il::Variable, src_pointer: &il::Variable) {
        todo!()
    }
    pub(crate) fn load_variable_offset(
        &self,
        id: &il::Variable,
        base: &il::Variable,
        offsets: Vec<il::Variable>,
    ) {
        todo!()
    }

    pub(crate) fn load_variable_offset_imm(
        &self,
        id: &il::Variable,
        base: &il::Variable,
        offset: u32,
    ) {
        todo!()
    }

    fn store(&mut self, dst: il::Variable, src: il::Variable) {
        todo!();
    }

    pub(crate) fn store_array(&self, dst_pointer: &il::Variable, src: &Vec<il::Variable>) {
        todo!()
    }
    pub(crate) fn store_pointer(&self, dst_pointer: il::Variable, str: il::Variable) {
        todo!()
    }
    pub(crate) fn store_imm32(&self, variable: &il::Variable, imm: &Vec<u32>) {
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

impl State {
    pub(crate) fn binary_op(
        &self,
        id: &il::Variable,
        op1: &il::Variable,
        op2: &il::Variable,
        kind: BinaryOpKind,
    ) {
        todo!()
    }

    pub(crate) fn convert(&self, id: &il::Variable, op: &il::Variable, kind: ConvertKind) {
        todo!()
    }
}

impl State {
    fn interpret_il_instruction(&mut self, instruction: &il::Instruction) -> bool {
        match dbg!(instruction) {
            il::Instruction::Label { id } => {
                self.labels.try_insert(*id, self.pc).unwrap();
            }
            il::Instruction::VariableDecl { id, decl } => {
                self.add_new_variable(*id, decl);
            }
            il::Instruction::StoreImm32 { dst, imm } => {
                self.store_imm32(dst, &vec![*imm]);
            }
            il::Instruction::StoreImm32Array { dst, imm } => {
                self.store_imm32(dst, imm);
            }
            il::Instruction::LoadVariableOffset { id, base, offsets } => {
                self.load_variable_offset(id, base, offsets.clone());
            }
            il::Instruction::LoadVariableImmOffset { id, base, offset } => {
                self.load_variable_offset_imm(id, base, *offset);
            }
            il::Instruction::StoreVariable { dst_pointer, src } => {
                self.store_pointer(*dst_pointer, *src);
            }
            il::Instruction::StoreVariableArray { dst, values } => {
                self.store_array(dst, values);
            }
            il::Instruction::LoadVariable { id, src_pointer } => {
                self.load_variable(id, src_pointer);
            }
            il::Instruction::MathMulVectorScalar { id, vector, scalar } => {
                self.binary_op(id, vector, scalar, BinaryOpKind::MulVectorScalar);
            }
            il::Instruction::MathAddI32I32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::AddI32I32);
            }
            il::Instruction::MathMulI32I32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::MulI32I32);
            }
            il::Instruction::MathSubF32F32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::SubF32F32);
            }
            il::Instruction::MathDivF32F32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::DivF32F3);
            }
            il::Instruction::MathDivI32I32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::DivI32I32);
            }
            il::Instruction::MathDivU32U32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::DivU32U32);
            }
            il::Instruction::MathModI32I32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::ModI32I32);
            }
            il::Instruction::MathModU32U32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::ModU32U32);
            }
            il::Instruction::MathBitAnd { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::BitAnd);
            }
            il::Instruction::MathBitShiftRight { id, base, shift } => {
                self.binary_op(id, base, shift, BinaryOpKind::BitShiftRight);
            }
            il::Instruction::MathEqualI32I32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::EqualI32I32);
            }
            il::Instruction::MathLessThanU32U32 { id, op1, op2 } => {
                self.binary_op(id, op1, op2, BinaryOpKind::LessThanU32U32);
            }
            il::Instruction::MathConvertI32F32 { id, op } => {
                self.convert(id, op, ConvertKind::I32F32);
            }
            il::Instruction::MathConvertF32U32 { id, op } => {
                self.convert(id, op, ConvertKind::F32U32);
            }
            il::Instruction::MathConvertU32F32 { id, op } => {
                self.convert(id, op, ConvertKind::U32F32);
            }
            il::Instruction::Return => {
                return true;
            }
            il::Instruction::Select {
                id,
                cond,
                obj1,
                obj2,
            } => {
                todo!()
            }
            il::Instruction::SelectionMerge { .. } => {
                todo!()
            }
            il::Instruction::LoopMerge { .. } => {
                todo!()
            }
            il::Instruction::Branch { .. } => {
                todo!()
            }
            il::Instruction::BranchConditional { .. } => {
                todo!()
            }
            il::Instruction::Kill => {
                todo!()
            }
        };
        self.pc += 1;
        false
    }
}
