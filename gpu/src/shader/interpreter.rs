use crate::shader::il;
use crate::shader::il::VariableDecl;
use crate::{
    Format, Fragment, FragmentShaderOutput, Position, Vector4, Vertex, VertexAttribute,
    VertexInputState, VertexShaderOutput, MAX_CLIP_DISTANCES,
};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Interpreter {
    il: il::Il,
}

impl Interpreter {
    pub fn new(name: &str, code: Vec<u32>) -> Option<Self> {
        let il = il::Il::new(name, code)?;
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
        let vertex_format = match vertex_input_state.attributes[0] {
            None => return vec![],
            Some(attribute) => attribute.format,
        };

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

    il_variables: HashMap<il::Variable, Variable>,
    memory_last_idx: u32,
}

impl State {
    fn new() -> Self {
        Self {
            pc: 0,
            labels: Default::default(),
            memory: vec![0_u8; 10000], // TODO: Max memory size.

            il_variables: Default::default(),
            memory_last_idx: 0,
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
        self.memory_mut(self.memory_region_of_built_in(BuiltIn::ClipDistance))
            .copy_from_slice(bytemuck::cast_slice(vertex.clip_distances.as_slice()));
        self.memory_mut(self.memory_region_of_built_in(BuiltIn::CullDistance))
            .copy_from_slice(bytemuck::cast_slice(&[0.0f32, 0.0f32, 0.0f32, 0.0f32]));
        self.memory_mut(self.memory_region_of_location(0))
            .copy_from_slice(bytemuck::cast_slice(
                vertex.position.get_as_f32_array().as_slice(),
            )); // HIRO use vertex bindings and vertex attributes, use format?
        let last_location = self.memory_region_of_location(0);
        self.memory_last_idx = last_location.address + last_location.size;
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
        let clip_distances = *bytemuck::from_bytes::<[f32; MAX_CLIP_DISTANCES as usize]>(
            self.memory(self.memory_region_of_built_in(BuiltIn::ClipDistance)),
        );
        VertexShaderOutput {
            position,
            point_size,
            vertex_index,
            clip_distances,
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
    FragCoord,
    ClipDistance,
    CullDistance,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MemoryRegion {
    address: u32,
    size: u32,
    stride: u32,
}

#[derive(Debug, Clone)]
pub(crate) enum Variable {
    MemoryRegion(MemoryRegion),
    Struct(Vec<Variable>),
}

impl Variable {
    fn size(decl: &VariableDecl) -> u32 {
        match decl.kind {
            il::VariableKind::F32 => 4,
            il::VariableKind::U32 => 4,
            il::VariableKind::I32 => 4,
            il::VariableKind::Void => unreachable!(),
            il::VariableKind::Bool => 1,
            il::VariableKind::Array => todo!(),
            il::VariableKind::Struct => todo!(),
            il::VariableKind::Pointer => todo!(),
        }
    }

    fn from_il(decl: &VariableDecl, state: &mut State) -> Self {
        match &decl.backing {
            il::VariableBacking::Memory => Self::MemoryRegion(MemoryRegion {
                address: {
                    state.memory_last_idx += Self::size(decl) * decl.component_count;
                    state.memory_last_idx
                },
                size: Self::size(decl) * decl.component_count,
                stride: Self::size(decl),
            }),
            il::VariableBacking::Location { number } => {
                Self::MemoryRegion(state.memory_region_of_location(*number))
            }
            il::VariableBacking::Position => {
                Self::MemoryRegion(state.memory_region_of_built_in(BuiltIn::Position))
            }
            il::VariableBacking::PointSize => {
                Self::MemoryRegion(state.memory_region_of_built_in(BuiltIn::PointSize))
            }
            il::VariableBacking::VertexIndex => {
                Self::MemoryRegion(state.memory_region_of_built_in(BuiltIn::VertexIndex))
            }
            il::VariableBacking::FragCoord => {
                Self::MemoryRegion(state.memory_region_of_built_in(BuiltIn::FragCoord))
            }
            il::VariableBacking::ClipDistance => {
                Self::MemoryRegion(state.memory_region_of_built_in(BuiltIn::ClipDistance))
            }
            il::VariableBacking::CullDistance => {
                Self::MemoryRegion(state.memory_region_of_built_in(BuiltIn::CullDistance))
            }
            il::VariableBacking::Array {
                element_kind,
                array_stride,
            } => {
                let variable = Self::from_il(element_kind, state);
                let element_size = Self::size(element_kind);
                let array_size = element_size * decl.component_count;
                let memory_region = state.unwrap_variable(&variable, &[]);
                assert!(memory_region.size >= array_size); // TODO: Extend memory region to at least decl.component_count
                Self::MemoryRegion(memory_region)
            }
            il::VariableBacking::Struct { members } => {
                Self::Struct(members.iter().map(|x| Self::from_il(x, state)).collect())
            }
            il::VariableBacking::Pointer { kind } => Self::MemoryRegion(MemoryRegion {
                address: {
                    state.memory_last_idx += std::mem::size_of::<u32>() as u32;
                    state.memory_last_idx
                },
                size: std::mem::size_of::<u32>() as u32,
                stride: std::mem::size_of::<u32>() as u32,
            }),
        }
    }
}

impl State {
    fn memory_region_of_built_in(&self, built_in: BuiltIn) -> MemoryRegion {
        match built_in {
            BuiltIn::Position => MemoryRegion {
                address: 0,
                size: std::mem::size_of::<f32>() as u32 * 4,
                stride: std::mem::size_of::<f32>() as u32,
            },
            BuiltIn::PointSize => {
                let prev = self.memory_region_of_built_in(BuiltIn::Position);
                MemoryRegion {
                    address: prev.address + prev.size,
                    size: std::mem::size_of::<f32>() as u32,
                    stride: std::mem::size_of::<f32>() as u32,
                }
            }
            BuiltIn::VertexIndex => {
                let prev = self.memory_region_of_built_in(BuiltIn::PointSize);
                MemoryRegion {
                    address: prev.address + prev.size,
                    size: std::mem::size_of::<u32>() as u32,
                    stride: std::mem::size_of::<u32>() as u32,
                }
            }
            BuiltIn::FragCoord => {
                let prev = self.memory_region_of_built_in(BuiltIn::VertexIndex);
                MemoryRegion {
                    address: prev.address + prev.size,
                    size: std::mem::size_of::<u32>() as u32,
                    stride: std::mem::size_of::<u32>() as u32,
                }
            }
            BuiltIn::ClipDistance => {
                let prev = self.memory_region_of_built_in(BuiltIn::FragCoord);
                let max_clip_distances = 4; // TODO: Use maxClipDistances limit.
                MemoryRegion {
                    address: prev.address + prev.size,
                    size: std::mem::size_of::<f32>() as u32 * max_clip_distances,
                    stride: std::mem::size_of::<f32>() as u32,
                }
            }
            BuiltIn::CullDistance => {
                let prev = self.memory_region_of_built_in(BuiltIn::ClipDistance);
                let max_cull_distances = 4; // TODO: Use maxCullDistances limit.
                MemoryRegion {
                    address: prev.address + prev.size,
                    size: std::mem::size_of::<f32>() as u32 * max_cull_distances,
                    stride: std::mem::size_of::<f32>() as u32,
                }
            }
        }
    }

    pub(crate) fn memory_region_of_location(&self, number: u32) -> MemoryRegion {
        let prev = self.memory_region_of_built_in(BuiltIn::CullDistance);
        match number {
            // TODO: Location depends on shader type (i.e. vertex attribute input / fragment shader output / uniform buffer)
            0 => MemoryRegion {
                address: prev.address + prev.size,
                size: std::mem::size_of::<f32>() as u32 * 4,
                stride: std::mem::size_of::<f32>() as u32,
            },
            _ => unimplemented!(),
        }
    }

    fn memory(&self, memory_region: MemoryRegion) -> &[u8] {
        &self.memory[memory_region.address as usize
            ..memory_region.address as usize + memory_region.size as usize]
    }

    fn memory_mut(&mut self, memory_region: MemoryRegion) -> &mut [u8] {
        &mut self.memory[memory_region.address as usize
            ..memory_region.address as usize + memory_region.size as usize]
    }

    fn copy_memory_region(&mut self, dst: MemoryRegion, src: MemoryRegion) {
        let size = src.size.min(dst.size) as usize;
        self.memory.copy_within(
            src.address as usize..src.address as usize + size,
            dst.address as usize,
        );
    }

    fn get_memory_region(&self, variable: &il::Variable, offsets: &[u32]) -> MemoryRegion {
        let variable = self.il_variables.get(variable).unwrap();
        self.unwrap_variable(variable, offsets)
    }

    fn unwrap_variable(&self, variable: &Variable, offsets: &[u32]) -> MemoryRegion {
        match variable {
            Variable::MemoryRegion(inner) => {
                if let &[] = offsets {
                    *inner
                } else if let &[offset] = offsets {
                    MemoryRegion {
                        address: inner.address + offset * inner.stride,
                        size: inner.size - offset * inner.stride,
                        stride: inner.stride,
                    }
                } else {
                    unreachable!()
                }
            }
            Variable::Struct(inner) => {
                self.unwrap_variable(&inner[offsets[0] as usize], &offsets[1..])
            }
        }
    }
}

impl State {
    fn add_new_variable(&mut self, il_variable: il::Variable, decl: &il::VariableDecl) {
        if !self.il_variables.contains_key(&il_variable) {
            let variable = Variable::from_il(decl, self);
            self.il_variables.insert(il_variable, variable);
        } else {
            unreachable!()
        }
    }
    pub(crate) fn load_variable(&mut self, id: &il::Variable, src_pointer: &il::Variable) {
        self.load_pointer_offset(id, src_pointer, &[])
    }
    pub(crate) fn load_pointer_offset(
        &mut self,
        id: &il::Variable,
        base: &il::Variable,
        offsets: &[il::Variable],
    ) {
        let id = self.get_memory_region(id, &[]);
        let offsets = offsets
            .iter()
            .map(|x| *bytemuck::from_bytes::<u32>(self.memory(self.get_memory_region(x, &[]))))
            .collect::<Vec<_>>();

        let src = self.get_memory_region(base, &offsets).address;
        self.store_imm32(id, &[src]);
    }

    pub(crate) fn load_variable_offset_imm(
        &mut self,
        id: &il::Variable,
        base: &il::Variable,
        offset: u32,
    ) {
        let id = self.get_memory_region(id, &[]);
        let src = self.get_memory_region(base, &[offset]).address;
        self.store_imm32(id, &[src]);
    }

    fn store(&mut self, dst: il::Variable, src: il::Variable) {
        todo!();
    }

    pub(crate) fn store_array(&mut self, dst_pointer: il::Variable, src: &[il::Variable]) {
        let dst_pointer = self.get_memory_region(&dst_pointer, &[]);
        for (i, src) in src.iter().enumerate() {
            let src = self.get_memory_region(&src, &[]);
            let dst = MemoryRegion {
                address: dst_pointer.address + i as u32 * dst_pointer.stride,
                size: src.size,
                stride: src.stride,
            };
            self.copy_memory_region(dst, src);
        }
    }
    pub(crate) fn store_through_pointer(&mut self, dst_pointer: il::Variable, src: il::Variable) {
        let dst_pointer = self.get_memory_region(&dst_pointer, &[]);
        let src = self.get_memory_region(&src, &[]);
        let dst_pointer =
            *bytemuck::from_bytes::<u32>(&self.memory(dst_pointer)[..std::mem::size_of::<u32>()]);
        let dst = MemoryRegion {
            address: dst_pointer,
            size: src.size,
            stride: src.stride,
        };
        self.copy_memory_region(dst, src);
    }
    pub(crate) fn store_imm32(&mut self, variable: MemoryRegion, imm: &[u32]) {
        // TODO: Use variable stride.
        for i in 0..imm.len() {
            self.memory[variable.address as usize + 4 * i..variable.address as usize + 4 * (i + 1)]
                .copy_from_slice(&imm[i].to_ne_bytes());
        }
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
        &mut self,
        id: &il::Variable,
        op1: &il::Variable,
        op2: &il::Variable,
        kind: BinaryOpKind,
    ) {
        let id = self.get_memory_region(id, &[]);
        let op1 = self.get_memory_region(op1, &[]);
        let op2 = self.get_memory_region(op2, &[]);
        match kind {
            BinaryOpKind::MulVectorScalar => {
                let vector: Vec<f32> = bytemuck::cast_slice(self.memory(op1)).to_vec();
                let scalar = *bytemuck::from_bytes::<f32>(self.memory(op2));
                for (i, value) in vector.iter().enumerate() {
                    let value = value * scalar;
                    self.memory_mut(id)
                        [i * std::mem::size_of::<f32>()..(i + 1) * std::mem::size_of::<f32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            BinaryOpKind::AddI32I32 => todo!(),
            BinaryOpKind::MulI32I32 => todo!(),
            BinaryOpKind::SubF32F32 => {
                let op1: Vec<f32> = bytemuck::cast_slice(self.memory(op1)).to_vec();
                let op2: Vec<f32> = bytemuck::cast_slice(self.memory(op2)).to_vec();
                for (i, (op1, op2)) in itertools::izip!(op1, op2).enumerate() {
                    let value = op1 - op2;
                    self.memory_mut(id)
                        [i * std::mem::size_of::<f32>()..(i + 1) * std::mem::size_of::<f32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            BinaryOpKind::DivF32F3 => todo!(),
            BinaryOpKind::DivI32I32 => todo!(),
            BinaryOpKind::DivU32U32 => todo!(),
            BinaryOpKind::ModI32I32 => todo!(),
            BinaryOpKind::ModU32U32 => todo!(),
            BinaryOpKind::BitAnd => todo!(),
            BinaryOpKind::BitShiftRight => todo!(),
            BinaryOpKind::EqualI32I32 => todo!(),
            BinaryOpKind::LessThanU32U32 => todo!(),
        }
    }

    pub(crate) fn convert(&self, id: &il::Variable, op: &il::Variable, kind: ConvertKind) {
        todo!()
    }
}

impl State {
    fn interpret_il_instruction(&mut self, instruction: &il::Instruction) -> bool {
        match instruction {
            il::Instruction::Label { id } => {
                self.labels.try_insert(*id, self.pc).unwrap();
            }
            il::Instruction::VariableDecl { id, decl } => {
                self.add_new_variable(*id, decl);
            }
            il::Instruction::StoreImm32 { dst, imm } => {
                let dst = self.get_memory_region(dst, &[]);
                self.store_imm32(dst, &[*imm]);
            }
            il::Instruction::StoreImm32Array { dst, imm } => {
                let dst = self.get_memory_region(dst, &[]);
                self.store_imm32(dst, imm);
            }
            il::Instruction::LoadVariableOffset { id, base, offsets } => {
                self.load_pointer_offset(id, base, offsets.as_slice());
            }
            il::Instruction::LoadVariableImmOffset { id, base, offset } => {
                self.load_variable_offset_imm(id, base, *offset);
            }
            il::Instruction::StoreVariable { dst_pointer, src } => {
                self.store_through_pointer(*dst_pointer, *src);
            }
            il::Instruction::StoreVariableArray { dst, values } => {
                self.store_array(*dst, values);
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
