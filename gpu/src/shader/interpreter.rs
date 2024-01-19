use crate::shader::il;
use crate::{
    Format, Fragment, FragmentShaderOutput, Vector4, Vertex, VertexInputState, VertexShaderOutput,
    MAX_CLIP_DISTANCES, MAX_CULL_DISTANCES,
};
use hashbrown::HashMap;
use log::warn;

#[derive(Debug, Clone)]
pub struct Interpreter {
    il: il::Il,
}

impl Interpreter {
    pub fn new(name: &str, code: Vec<u32>) -> anyhow::Result<Self> {
        let il = il::Il::new(name, code)?;
        Ok(Self { il })
    }
}

impl Interpreter {
    pub(crate) fn execute_vertex_shader(
        &self,
        _vertex_input_state: &VertexInputState,
        vertices: Vec<Vertex>,
    ) -> Vec<VertexShaderOutput> {
        warn!("TODO: Create shader input/output interfaces, check if match between stages");

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
    memory_last_idx: u32,

    array_variables: Vec<ArrayVariable>,
    struct_variables: Vec<StructVariable>,
    pointer_variables: Vec<PointerVariable>,

    built_in_variables: HashMap<BuiltIn, Variable>,
    location_variables: HashMap<u32, Variable>,

    il_variables: HashMap<il::Variable, Variable>,
}

impl State {
    fn new() -> Self {
        Self {
            pc: 0,
            labels: Default::default(),
            memory: vec![0_u8; 10000], // TODO: Max memory size.
            memory_last_idx: 0,
            array_variables: vec![],
            struct_variables: vec![],
            pointer_variables: vec![],
            built_in_variables: Default::default(),
            location_variables: Default::default(),
            il_variables: Default::default(),
        }
    }
}

impl State {
    fn store_imm32(&mut self, variable: ArrayVariable, imm: &[u32]) {
        warn!("TODO: Use variable stride");
        for (i, src) in imm.iter().enumerate() {
            self.memory[variable.memory_region.address as usize + 4 * i
                ..variable.memory_region.address as usize + 4 * (i + 1)]
                .copy_from_slice(&src.to_ne_bytes());
        }
    }

    fn load_imm32(&self, variable: ArrayVariable) -> &[u32] {
        warn!("TODO: Use variable stride");
        bytemuck::cast_slice(self.memory(&variable.memory_region))
    }

    fn store_array(&mut self, dst: ArrayVariable, src: ArrayVariable) {
        warn!("TODO: Use variable stride");
        self.copy_memory_region(dst.memory_region, src.memory_region);
    }
}

impl State {
    fn set_vertex_shader_input(&mut self, vertex: Vertex) {
        let memory_region = self.allocate_memory(std::mem::size_of::<f32>() as u32 * 4);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<f32>() as u32,
        });
        self.built_in_variables.insert(BuiltIn::Position, variable);
        self.store_imm32(
            *self.array_variable(self.built_in_variable(BuiltIn::Position)),
            bytemuck::cast_slice(vertex.position.get_as_f32_array().as_slice()),
        );

        let memory_region = self.allocate_memory(std::mem::size_of::<f32>() as u32);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<f32>() as u32,
        });
        self.built_in_variables.insert(BuiltIn::PointSize, variable);
        self.store_imm32(
            *self.array_variable(self.built_in_variable(BuiltIn::PointSize)),
            bytemuck::cast_slice(&[f32::to_bits(vertex.point_size)]),
        );

        let memory_region = self.allocate_memory(std::mem::size_of::<u32>() as u32);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<u32>() as u32,
        });
        self.built_in_variables
            .insert(BuiltIn::VertexIndex, variable);
        self.store_imm32(
            *self.array_variable(self.built_in_variable(BuiltIn::VertexIndex)),
            bytemuck::cast_slice(&[vertex.index]),
        );

        let memory_region =
            self.allocate_memory(std::mem::size_of::<f32>() as u32 * MAX_CLIP_DISTANCES);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<f32>() as u32,
        });
        self.built_in_variables
            .insert(BuiltIn::ClipDistance, variable);
        self.store_imm32(
            *self.array_variable(self.built_in_variable(BuiltIn::ClipDistance)),
            bytemuck::cast_slice(vertex.clip_distances.as_slice()),
        );

        let memory_region =
            self.allocate_memory(std::mem::size_of::<f32>() as u32 * MAX_CULL_DISTANCES);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<f32>() as u32,
        });
        self.built_in_variables
            .insert(BuiltIn::CullDistance, variable);
        self.store_imm32(
            *self.array_variable(self.built_in_variable(BuiltIn::CullDistance)),
            bytemuck::cast_slice(&[0.0f32, 0.0f32, 0.0f32, 0.0f32]),
        );

        let memory_region = self.allocate_memory(std::mem::size_of::<f32>() as u32 * 4);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<f32>() as u32,
        });
        self.location_variables.insert(0, variable);
        self.store_imm32(
            *self.array_variable(self.location_variable(0)),
            bytemuck::cast_slice(vertex.position.get_as_f32_array().as_slice()),
        );
        warn!("TODO: use vertex bindings and vertex attributes?");
    }

    fn vertex_shader_output(&self) -> VertexShaderOutput {
        let position = Vector4::from_vertex_buffer_bytes(
            Format::R32G32B32A32Sfloat,
            bytemuck::cast_slice(
                self.load_imm32(*self.array_variable(self.built_in_variable(BuiltIn::Position))),
            ),
        );
        let point_size = *bytemuck::from_bytes::<f32>(bytemuck::cast_slice(
            self.load_imm32(*self.array_variable(self.built_in_variable(BuiltIn::PointSize))),
        ));
        let vertex_index = *bytemuck::from_bytes::<u32>(bytemuck::cast_slice(
            self.load_imm32(*self.array_variable(self.built_in_variable(BuiltIn::VertexIndex))),
        ));
        let clip_distances =
            *bytemuck::from_bytes::<[f32; MAX_CLIP_DISTANCES as usize]>(bytemuck::cast_slice(
                self.load_imm32(
                    *self.array_variable(self.built_in_variable(BuiltIn::ClipDistance)),
                ),
            ));
        VertexShaderOutput {
            position,
            point_size,
            vertex_index,
            clip_distances,
        }
    }

    fn set_fragment_shader_input(&mut self, fragment: Fragment) {
        let memory_region = self.allocate_memory(std::mem::size_of::<f32>() as u32 * 4);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<f32>() as u32,
        });
        self.built_in_variables.insert(BuiltIn::FragCoord, variable);
        self.store_imm32(
            *self.array_variable(self.built_in_variable(BuiltIn::FragCoord)),
            bytemuck::cast_slice(fragment.position.get_as_f32_array().as_slice()),
        );

        let memory_region = self.allocate_memory(std::mem::size_of::<f32>() as u32 * 4);
        let variable = self.add_array_variable(ArrayVariable {
            memory_region,
            stride: std::mem::size_of::<f32>() as u32,
        });
        self.location_variables.insert(0, variable);
        self.store_imm32(
            *self.array_variable(self.location_variable(0)),
            bytemuck::cast_slice(fragment.color.get_as_f32_array().as_slice()),
        );
        warn!("TODO: use descriptors");
    }

    fn fragment_shader_output(&mut self) -> FragmentShaderOutput {
        let position = Vector4::from_vertex_buffer_bytes(
            Format::R32G32B32A32Sfloat,
            bytemuck::cast_slice(
                self.load_imm32(*self.array_variable(self.built_in_variable(BuiltIn::FragCoord))),
            ),
        );
        warn!("TODO: Determine color using fragment shader interface");
        let color = Vector4::from_vertex_buffer_bytes(
            Format::R32G32B32A32Sfloat,
            bytemuck::cast_slice(self.load_imm32(*self.array_variable(self.location_variable(0)))),
        );
        FragmentShaderOutput { position, color }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum BuiltIn {
    Position,
    FragCoord,
    PointSize,
    VertexIndex,
    ClipDistance,
    CullDistance,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    address: u32,
    size: u32,
}
#[derive(Debug, Copy, Clone)]
pub enum Variable {
    Array(ArrayVariableId),
    Struct(StructVariableId),
    Pointer(PointerVariableId),
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct ArrayVariableId(u32);

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct StructVariableId(u32);
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct PointerVariableId(u32);

#[derive(Debug, Clone, Copy)]
pub struct ArrayVariable {
    memory_region: MemoryRegion,
    stride: u32,
}

impl ArrayVariable {
    const fn len(&self) -> u32 {
        self.memory_region.size / self.stride
    }

    pub(crate) fn indexed(&self, index: u32) -> Self {
        assert!(index * self.stride <= self.memory_region.size - self.stride);
        Self {
            memory_region: MemoryRegion {
                address: self.memory_region.address + index * self.stride,
                size: self.stride,
            },
            stride: self.stride,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructVariable {
    members: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct PointerVariable {
    pointer: Option<Variable>,
}

impl Variable {
    fn size(decl: &il::VariableDecl) -> u32 {
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

    fn from_il(decl: &il::VariableDecl, state: &mut State) -> Self {
        match &decl.backing {
            il::VariableBacking::Memory => {
                let memory_region = state.allocate_memory(Self::size(decl) * decl.component_count);
                state.add_array_variable(ArrayVariable {
                    memory_region,
                    stride: Self::size(decl),
                })
            }
            il::VariableBacking::Location { number } => state.location_variable(*number),
            il::VariableBacking::Position => state.built_in_variable(BuiltIn::Position),
            il::VariableBacking::PointSize => state.built_in_variable(BuiltIn::PointSize),
            il::VariableBacking::VertexIndex => state.built_in_variable(BuiltIn::VertexIndex),
            il::VariableBacking::FragCoord => state.built_in_variable(BuiltIn::FragCoord),
            il::VariableBacking::ClipDistance => state.built_in_variable(BuiltIn::ClipDistance),
            il::VariableBacking::CullDistance => state.built_in_variable(BuiltIn::CullDistance),
            il::VariableBacking::Array {
                element_kind,
                array_stride,
            } => {
                let element = Self::from_il(element_kind, state);
                let array = state.array_variable(element);
                if (array.stride == *array_stride
                    && array.memory_region.size / array.stride >= decl.component_count)
                    || *array_stride == 0
                {
                    element
                } else {
                    unimplemented!("{element_kind:#?} {array_stride:#?} {element:#?}")
                }
            }
            il::VariableBacking::Struct { members } => {
                let variable = StructVariable {
                    members: members.iter().map(|x| Self::from_il(x, state)).collect(),
                };
                state.add_struct_variable(variable)
            }
            il::VariableBacking::Pointer { kind } => {
                let variable = PointerVariable {
                    pointer: Some(Self::from_il(kind, state)),
                };
                state.add_pointer_variable(variable)
            }
        }
    }
}

impl State {
    fn add_array_variable(&mut self, variable: ArrayVariable) -> Variable {
        let id = ArrayVariableId(self.array_variables.len() as u32);
        self.array_variables.push(variable);
        Variable::Array(id)
    }

    fn add_struct_variable(&mut self, variable: StructVariable) -> Variable {
        let id = StructVariableId(self.struct_variables.len() as u32);
        self.struct_variables.push(variable);
        Variable::Struct(id)
    }

    fn add_pointer_variable(&mut self, variable: PointerVariable) -> Variable {
        let id = PointerVariableId(self.pointer_variables.len() as u32);
        self.pointer_variables.push(variable);
        Variable::Pointer(id)
    }

    fn array_variable(&self, variable: Variable) -> &ArrayVariable {
        let Variable::Array(id) = variable else {
            unreachable!()
        };
        &self.array_variables[id.0 as usize]
    }

    fn struct_variable(&self, variable: Variable) -> &StructVariable {
        let Variable::Struct(id) = variable else {
            unreachable!()
        };
        &self.struct_variables[id.0 as usize]
    }

    fn pointer_variable(&self, variable: Variable) -> &PointerVariable {
        let Variable::Pointer(id) = variable else {
            unreachable!()
        };
        &self.pointer_variables[id.0 as usize]
    }

    fn pointer_variable_mut(&mut self, variable: Variable) -> &mut PointerVariable {
        let Variable::Pointer(id) = variable else {
            unreachable!()
        };
        &mut self.pointer_variables[id.0 as usize]
    }

    fn built_in_variable(&self, built_in: BuiltIn) -> Variable {
        *self
            .built_in_variables
            .get(&built_in)
            .unwrap_or_else(|| unreachable!())
    }

    pub(crate) fn location_variable(&self, number: u32) -> Variable {
        *self
            .location_variables
            .get(&number)
            .unwrap_or_else(|| unreachable!())
    }

    fn allocate_memory(&mut self, size: u32) -> MemoryRegion {
        MemoryRegion {
            address: {
                let address = self.memory_last_idx;
                self.memory_last_idx += size;
                address
            },
            size,
        }
    }

    fn memory(&self, memory_region: &MemoryRegion) -> &[u8] {
        &self.memory[memory_region.address as usize
            ..memory_region.address as usize + memory_region.size as usize]
    }

    fn memory_mut(&mut self, memory_region: &MemoryRegion) -> &mut [u8] {
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
}

impl State {
    fn il_add_new_variable(&mut self, il_variable: &il::Variable, decl: &il::VariableDecl) {
        if !self.il_variables.contains_key(il_variable) {
            let variable = Variable::from_il(decl, self);
            self.il_variables.insert(*il_variable, variable);
        } else {
            unreachable!()
        }
    }

    fn il_variable(&self, il_variable: &il::Variable) -> Variable {
        *self
            .il_variables
            .get(il_variable)
            .unwrap_or_else(|| unreachable!())
    }

    pub(crate) fn il_load_offset_to_pointer(
        &mut self,
        result: &il::Variable,
        base: &il::Variable,
        offsets: &[il::Variable],
    ) {
        let base = self.pointer_variable(self.il_variable(base));
        let offsets = offsets
            .iter()
            .map(|x| {
                *bytemuck::from_bytes::<u32>(
                    self.memory(&self.array_variable(self.il_variable(x)).memory_region),
                )
            })
            .collect::<Vec<_>>();

        let mut src = *base.pointer.as_ref().unwrap_or_else(|| unreachable!());

        for offset in offsets {
            src = match src {
                Variable::Array(_) => {
                    self.add_array_variable(self.array_variable(src).indexed(offset))
                }
                Variable::Struct(_) => self.struct_variable(src).members[offset as usize],
                Variable::Pointer(_) => {
                    unreachable!()
                }
            };
        }

        let result = self.pointer_variable_mut(self.il_variable(result));
        let result = result.pointer.as_mut().unwrap_or_else(|| unreachable!());
        *result = src;
    }

    pub(crate) fn il_load_pointer_to_variable(
        &mut self,
        result: &il::Variable,
        src_pointer: &il::Variable,
    ) {
        let src_pointer = self.pointer_variable(self.il_variable(src_pointer));

        let src = *self.array_variable(src_pointer.pointer.unwrap_or_else(|| unreachable!()));
        let result = *self.array_variable(self.il_variable(result));
        self.store_array(result, src);
    }

    pub(crate) fn il_load_composite_member_to_variable(
        &mut self,
        result: &il::Variable,
        base: &il::Variable,
        offset: u32,
    ) {
        let base = self.il_variable(base);
        let src = match base {
            Variable::Array(_) => {
                self.add_array_variable(self.array_variable(base).indexed(offset))
            }
            Variable::Struct(_) => self.struct_variable(base).members[offset as usize],
            Variable::Pointer(_) => {
                unreachable!()
            }
        };

        let result = *self.array_variable(self.il_variable(result));
        let src = *self.array_variable(src);
        self.store_array(result, src);
    }

    fn il_store(&mut self, _dst: &il::Variable, _src: &il::Variable) {
        todo!();
    }

    pub(crate) fn il_store_variables_to_composite(
        &mut self,
        dst: &il::Variable,
        src: &[il::Variable],
    ) {
        let dst = self.il_variable(dst);
        let src = src.iter().map(|x| self.il_variable(x)).collect::<Vec<_>>();

        match dst {
            Variable::Array(_) => {
                let mut index = 0;
                for src in src {
                    let src = *self.array_variable(src);
                    self.store_array(self.array_variable(dst).indexed(index), src);
                    index += src.len();
                }
            }
            Variable::Struct(_) => {
                unimplemented!()
            }
            Variable::Pointer(_) => unreachable!(),
        }
    }

    pub(crate) fn il_store_through_pointer(
        &mut self,
        dst_pointer: &il::Variable,
        src: &il::Variable,
    ) {
        let src = self.il_variable(src);
        let dst = *self
            .pointer_variable(self.il_variable(dst_pointer))
            .pointer
            .as_ref()
            .unwrap_or_else(|| unreachable!());

        warn!("TODO: Match for identical object types");
        self.store_array(*self.array_variable(dst), *self.array_variable(src));
    }

    pub(crate) fn il_store_imm32(&mut self, variable: &il::Variable, imm: &[u32]) {
        warn!("TODO: Use variable stride");
        let dst = self
            .array_variable(self.il_variable(variable))
            .memory_region;
        for (i, src) in imm.iter().enumerate() {
            self.memory[dst.address as usize + 4 * i..dst.address as usize + 4 * (i + 1)]
                .copy_from_slice(&src.to_ne_bytes());
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum BinaryOpKind {
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

#[derive(Debug, Copy, Clone)]
enum ConvertKind {
    I32F32,
    F32U32,
    U32F32,
}

impl State {
    pub(crate) fn il_binary_op(
        &mut self,
        result: &il::Variable,
        op1: &il::Variable,
        op2: &il::Variable,
        kind: BinaryOpKind,
    ) {
        let result = *self.array_variable(self.il_variable(result));
        let op1 = *self.array_variable(self.il_variable(op1));
        let op2 = *self.array_variable(self.il_variable(op2));
        match kind {
            BinaryOpKind::MulVectorScalar => {
                let vector: Vec<f32> =
                    bytemuck::cast_slice(self.memory(&op1.memory_region)).to_vec();
                let scalar = *bytemuck::from_bytes::<f32>(self.memory(&op2.memory_region));
                for (i, value) in vector.iter().enumerate() {
                    let value = value * scalar;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<f32>()..(i + 1) * std::mem::size_of::<f32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            BinaryOpKind::AddI32I32 => todo!(),
            BinaryOpKind::MulI32I32 => todo!(),
            BinaryOpKind::SubF32F32 => {
                let op1: Vec<f32> = bytemuck::cast_slice(self.memory(&op1.memory_region)).to_vec();
                let op2: Vec<f32> = bytemuck::cast_slice(self.memory(&op2.memory_region)).to_vec();
                for (i, (op1, op2)) in itertools::izip!(op1, op2).enumerate() {
                    let value = op1 - op2;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<f32>()..(i + 1) * std::mem::size_of::<f32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            BinaryOpKind::DivF32F32 => {
                let op1: Vec<f32> = bytemuck::cast_slice(self.memory(&op1.memory_region)).to_vec();
                let op2: Vec<f32> = bytemuck::cast_slice(self.memory(&op2.memory_region)).to_vec();
                for (i, (op1, op2)) in itertools::izip!(op1, op2).enumerate() {
                    let value = op1 / op2;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<f32>()..(i + 1) * std::mem::size_of::<f32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            BinaryOpKind::DivI32I32 => {
                let op1: Vec<i32> = bytemuck::cast_slice(self.memory(&op1.memory_region)).to_vec();
                let op2: Vec<i32> = bytemuck::cast_slice(self.memory(&op2.memory_region)).to_vec();
                for (i, (op1, op2)) in itertools::izip!(op1, op2).enumerate() {
                    let value = op1 / op2;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<i32>()..(i + 1) * std::mem::size_of::<i32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            BinaryOpKind::DivU32U32 => todo!(),
            BinaryOpKind::ModI32I32 => {
                let op1: Vec<i32> = bytemuck::cast_slice(self.memory(&op1.memory_region)).to_vec();
                let op2: Vec<i32> = bytemuck::cast_slice(self.memory(&op2.memory_region)).to_vec();
                for (i, (op1, op2)) in itertools::izip!(op1, op2).enumerate() {
                    let value = op1 % op2;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<i32>()..(i + 1) * std::mem::size_of::<i32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            BinaryOpKind::ModU32U32 => todo!(),
            BinaryOpKind::BitAnd => todo!(),
            BinaryOpKind::BitShiftRight => todo!(),
            BinaryOpKind::EqualI32I32 => todo!(),
            BinaryOpKind::LessThanU32U32 => todo!(),
        }
    }

    pub(crate) fn il_convert(
        &mut self,
        result: &il::Variable,
        op: &il::Variable,
        kind: ConvertKind,
    ) {
        let result = *self.array_variable(self.il_variable(result));
        let op = *self.array_variable(self.il_variable(op));
        match kind {
            ConvertKind::I32F32 => {
                let op: Vec<i32> = bytemuck::cast_slice(self.memory(&op.memory_region)).to_vec();
                for (i, &op) in op.iter().enumerate() {
                    let value = op as f32;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<f32>()..(i + 1) * std::mem::size_of::<f32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            ConvertKind::F32U32 => {
                let op: Vec<f32> = bytemuck::cast_slice(self.memory(&op.memory_region)).to_vec();
                for (i, &op) in op.iter().enumerate() {
                    let value = op as u32;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<u32>()..(i + 1) * std::mem::size_of::<u32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
            ConvertKind::U32F32 => {
                let op: Vec<u32> = bytemuck::cast_slice(self.memory(&op.memory_region)).to_vec();
                for (i, &op) in op.iter().enumerate() {
                    let value = op as f32;
                    self.memory_mut(&result.memory_region)
                        [i * std::mem::size_of::<f32>()..(i + 1) * std::mem::size_of::<f32>()]
                        .copy_from_slice(bytemuck::bytes_of(&value));
                }
            }
        }
    }
}

impl State {
    fn interpret_il_instruction(&mut self, instruction: &il::Instruction) -> bool {
        match instruction {
            il::Instruction::Label { id } => {
                self.labels.insert_unique_unchecked(*id, self.pc);
            }
            il::Instruction::VariableDecl { id, decl } => {
                self.il_add_new_variable(id, decl);
            }
            il::Instruction::StoreImm32 { dst, imm } => {
                self.il_store_imm32(dst, &[*imm]);
            }
            il::Instruction::StoreImm32Array { dst, imm } => {
                self.il_store_imm32(dst, imm);
            }
            il::Instruction::LoadVariableOffset { id, base, offsets } => {
                self.il_load_offset_to_pointer(id, base, offsets.as_slice());
            }
            il::Instruction::LoadVariableImmOffset { id, base, offset } => {
                self.il_load_composite_member_to_variable(id, base, *offset);
            }
            il::Instruction::StoreVariable { dst_pointer, src } => {
                self.il_store_through_pointer(dst_pointer, src);
            }
            il::Instruction::StoreVariableArray { dst, values } => {
                self.il_store_variables_to_composite(dst, values);
            }
            il::Instruction::LoadVariable { id, src_pointer } => {
                self.il_load_pointer_to_variable(id, src_pointer);
            }
            il::Instruction::MathMulVectorScalar { id, vector, scalar } => {
                self.il_binary_op(id, vector, scalar, BinaryOpKind::MulVectorScalar);
            }
            il::Instruction::MathAddI32I32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::AddI32I32);
            }
            il::Instruction::MathMulI32I32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::MulI32I32);
            }
            il::Instruction::MathSubF32F32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::SubF32F32);
            }
            il::Instruction::MathDivF32F32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::DivF32F32);
            }
            il::Instruction::MathDivI32I32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::DivI32I32);
            }
            il::Instruction::MathDivU32U32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::DivU32U32);
            }
            il::Instruction::MathModI32I32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::ModI32I32);
            }
            il::Instruction::MathModU32U32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::ModU32U32);
            }
            il::Instruction::MathBitAnd { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::BitAnd);
            }
            il::Instruction::MathBitShiftRight { id, base, shift } => {
                self.il_binary_op(id, base, shift, BinaryOpKind::BitShiftRight);
            }
            il::Instruction::MathEqualI32I32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::EqualI32I32);
            }
            il::Instruction::MathLessThanU32U32 { id, op1, op2 } => {
                self.il_binary_op(id, op1, op2, BinaryOpKind::LessThanU32U32);
            }
            il::Instruction::MathConvertI32F32 { id, op } => {
                self.il_convert(id, op, ConvertKind::I32F32);
            }
            il::Instruction::MathConvertF32U32 { id, op } => {
                self.il_convert(id, op, ConvertKind::F32U32);
            }
            il::Instruction::MathConvertU32F32 { id, op } => {
                self.il_convert(id, op, ConvertKind::U32F32);
            }
            il::Instruction::Return => {
                return true;
            }
            il::Instruction::Select {
                id: _,
                cond: _,
                obj1: _,
                obj2: _,
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
