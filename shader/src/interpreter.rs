use std::ops::Range;

use crate::error::Error;
use crate::glsl::{FragmentShaderOutput, VertexShaderOutput};
use crate::il::{self, BinaryOp, UnaryOp};
use common::consts::{MAX_CLIP_DISTANCES, MAX_CULL_DISTANCES};
use common::graphics::VertexInputState;
use common::math::{Format, Fragment, Vector4, Vertex};
use hashbrown::HashMap;

/// Size of the interpreter's scratch memory, in bytes.
const MEMORY_SIZE: usize = 10_000;
const WORD_SIZE: u32 = std::mem::size_of::<u32>() as u32;
const FLOAT_SIZE: u32 = std::mem::size_of::<f32>() as u32;

/// Executes shader stages lowered to [`il::Il`].
#[derive(Debug, Clone)]
pub struct Interpreter {
    il: il::Il,
}

impl Interpreter {
    pub fn new(name: &str, code: Vec<u32>) -> Result<Self, Error> {
        let il = il::Il::new(name, code)?;
        Ok(Self { il })
    }

    pub(crate) fn execute_vertex_shader(
        &self,
        _vertex_input_state: &VertexInputState,
        vertices: Vec<Vertex>,
    ) -> Vec<VertexShaderOutput> {
        self.execute(
            vertices,
            State::set_vertex_shader_input,
            State::vertex_shader_output,
        )
    }

    pub(crate) fn execute_fragment_shader(
        &self,
        fragments: Vec<Fragment>,
    ) -> Vec<FragmentShaderOutput> {
        self.execute(
            fragments,
            State::set_fragment_shader_input,
            State::fragment_shader_output,
        )
    }

    fn execute<I, O>(
        &self,
        inputs: Vec<I>,
        set_input: impl Fn(&mut State, I),
        output: impl Fn(&State) -> O,
    ) -> Vec<O> {
        inputs
            .into_iter()
            .map(|input| {
                let mut state = State::new();
                set_input(&mut state, input);
                state.run(&self.il.instructions);
                output(&state)
            })
            .collect()
    }
}

#[derive(Debug)]
struct State {
    pc: usize,
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
            memory: vec![0; MEMORY_SIZE], // TODO: Max memory size.
            memory_last_idx: 0,
            array_variables: Vec::new(),
            struct_variables: Vec::new(),
            pointer_variables: Vec::new(),
            built_in_variables: HashMap::default(),
            location_variables: HashMap::default(),
            il_variables: HashMap::default(),
        }
    }

    fn run(&mut self, instructions: &[il::Instruction]) {
        loop {
            let instruction = &instructions[self.pc];
            if self.interpret_il_instruction(instruction) {
                break;
            }
        }
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

#[derive(Debug, Copy, Clone)]
struct MemoryRegion {
    address: u32,
    size: u32,
}

#[derive(Debug, Copy, Clone)]
enum Variable {
    Array(ArrayVariableId),
    Struct(StructVariableId),
    Pointer(PointerVariableId),
}

#[derive(Debug, Copy, Clone)]
struct ArrayVariableId(usize);

#[derive(Debug, Copy, Clone)]
struct StructVariableId(usize);

#[derive(Debug, Copy, Clone)]
struct PointerVariableId(usize);

#[derive(Debug, Copy, Clone)]
struct ArrayVariable {
    memory_region: MemoryRegion,
    stride: u32,
}

impl ArrayVariable {
    const fn len(&self) -> u32 {
        self.memory_region.size / self.stride
    }

    const fn is_bool(&self) -> bool {
        self.stride == 1
    }

    fn indexed(&self, index: u32) -> Self {
        let offset = index
            .checked_mul(self.stride)
            .expect("array offset overflows");
        let max_offset = self
            .memory_region
            .size
            .checked_sub(self.stride)
            .expect("array element does not fit in its memory region");
        assert!(offset <= max_offset, "array index {index} is out of bounds");
        Self {
            memory_region: MemoryRegion {
                address: self.memory_region.address + offset,
                size: self.stride,
            },
            stride: self.stride,
        }
    }
}

#[derive(Debug, Clone)]
struct StructVariable {
    members: Vec<Variable>,
}

#[derive(Debug, Clone)]
struct PointerVariable {
    pointer: Option<Variable>,
}

impl Variable {
    fn size(decl: &il::VariableDecl) -> u32 {
        match decl.kind {
            il::VariableKind::F32 => FLOAT_SIZE,
            il::VariableKind::U32 | il::VariableKind::I32 => WORD_SIZE,
            il::VariableKind::Void => unreachable!("void variables have no size"),
            il::VariableKind::Bool => 1,
            il::VariableKind::Array | il::VariableKind::Struct | il::VariableKind::Pointer => {
                unimplemented!("memory size of {:?} variables", decl.kind)
            }
        }
    }

    fn byte_size(decl: &il::VariableDecl) -> u32 {
        match decl.kind {
            il::VariableKind::Array => {
                let il::VariableBacking::Array {
                    element_kind,
                    array_stride,
                } = &decl.backing
                else {
                    unreachable!("array variable does not have array backing");
                };
                let element_size = Self::byte_size(element_kind);
                let stride = if *array_stride == 0 {
                    element_size
                } else {
                    *array_stride
                };
                stride * decl.component_count
            }
            il::VariableKind::Struct => {
                let il::VariableBacking::Struct { members } = &decl.backing else {
                    unreachable!("struct variable does not have struct backing");
                };
                members.iter().map(Self::byte_size).sum()
            }
            il::VariableKind::Pointer => WORD_SIZE,
            _ => Self::size(decl) * decl.component_count,
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
                if !matches!(element_kind.backing, il::VariableBacking::Memory) {
                    // Arrays of built-ins (e.g. gl_ClipDistance) are already
                    // represented by the built-in variable.
                    let element = Self::from_il(element_kind, state);
                    let array = state.array_variable(element);
                    if *array_stride == 0 || array.stride == *array_stride {
                        return element;
                    }
                }

                let element_size = Self::byte_size(element_kind);
                let stride = if *array_stride == 0 {
                    element_size
                } else {
                    *array_stride
                };
                let memory_region = state.allocate_memory(stride * decl.component_count);
                state.add_array_variable(ArrayVariable {
                    memory_region,
                    stride,
                })
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
        let id = ArrayVariableId(self.array_variables.len());
        self.array_variables.push(variable);
        Variable::Array(id)
    }

    fn add_struct_variable(&mut self, variable: StructVariable) -> Variable {
        let id = StructVariableId(self.struct_variables.len());
        self.struct_variables.push(variable);
        Variable::Struct(id)
    }

    fn add_pointer_variable(&mut self, variable: PointerVariable) -> Variable {
        let id = PointerVariableId(self.pointer_variables.len());
        self.pointer_variables.push(variable);
        Variable::Pointer(id)
    }

    fn array_variable(&self, variable: Variable) -> ArrayVariable {
        let Variable::Array(id) = variable else {
            unreachable!("expected an array variable");
        };
        self.array_variables[id.0]
    }

    fn struct_variable(&self, variable: Variable) -> &StructVariable {
        let Variable::Struct(id) = variable else {
            unreachable!("expected a struct variable");
        };
        &self.struct_variables[id.0]
    }

    fn pointer_variable(&self, variable: Variable) -> &PointerVariable {
        let Variable::Pointer(id) = variable else {
            unreachable!("expected a pointer variable");
        };
        &self.pointer_variables[id.0]
    }

    fn pointer_variable_mut(&mut self, variable: Variable) -> &mut PointerVariable {
        let Variable::Pointer(id) = variable else {
            unreachable!("expected a pointer variable");
        };
        &mut self.pointer_variables[id.0]
    }

    fn built_in_variable(&self, built_in: BuiltIn) -> Variable {
        *self
            .built_in_variables
            .get(&built_in)
            .expect("built-in variable is initialized before execution")
    }

    fn location_variable(&self, number: u32) -> Variable {
        *self
            .location_variables
            .get(&number)
            .expect("location variable is initialized before execution")
    }

    const fn allocate_memory(&mut self, size: u32) -> MemoryRegion {
        let address = self.memory_last_idx;
        self.memory_last_idx += size;
        MemoryRegion { address, size }
    }

    fn memory(&self, region: &MemoryRegion) -> &[u8] {
        &self.memory[region_range(region)]
    }

    fn memory_mut(&mut self, region: &MemoryRegion) -> &mut [u8] {
        &mut self.memory[region_range(region)]
    }

    fn copy_memory_region(&mut self, dst: MemoryRegion, src: MemoryRegion) {
        let size = to_usize(src.size.min(dst.size));
        let src_start = to_usize(src.address);
        self.memory
            .copy_within(src_start..src_start + size, to_usize(dst.address));
    }
}

impl State {
    /// Stores an input value in a newly allocated array variable.
    fn add_input_variable<T: bytemuck::Pod>(&mut self, values: &[T]) -> Variable {
        let stride = u32::try_from(std::mem::size_of::<T>())
            .expect("input element size fits in a SPIR-V word");
        let count = u32::try_from(values.len()).expect("input length fits in u32");
        let memory_region = self.allocate_memory(stride * count);
        let variable = ArrayVariable {
            memory_region,
            stride,
        };
        self.store_pod(variable, values);
        self.add_array_variable(variable)
    }

    fn set_vertex_shader_input(&mut self, vertex: Vertex) {
        let position = self.add_input_variable(&vertex.position.get_as_f32_array());
        self.built_in_variables.insert(BuiltIn::Position, position);

        let point_size = self.add_input_variable(&[vertex.point_size]);
        self.built_in_variables
            .insert(BuiltIn::PointSize, point_size);

        let vertex_index = self.add_input_variable(&[vertex.index]);
        self.built_in_variables
            .insert(BuiltIn::VertexIndex, vertex_index);

        let clip_distance = self.add_input_variable(&vertex.clip_distances);
        self.built_in_variables
            .insert(BuiltIn::ClipDistance, clip_distance);

        let cull_distance = self.add_input_variable(&[0.0f32; MAX_CULL_DISTANCES as usize]);
        self.built_in_variables
            .insert(BuiltIn::CullDistance, cull_distance);

        let location = self.add_input_variable(&vertex.position.get_as_f32_array());
        self.location_variables.insert(0, location);

        // TODO: Use the vertex input state to fetch vertex attributes.
    }

    fn vertex_shader_output(&self) -> VertexShaderOutput {
        let position = Vector4::from_vertex_buffer_bytes(
            Format::R32G32B32A32Sfloat,
            self.bytes(self.array_variable(self.built_in_variable(BuiltIn::Position))),
        );
        let point_size = *bytemuck::from_bytes::<f32>(
            self.bytes(self.array_variable(self.built_in_variable(BuiltIn::PointSize))),
        );
        let vertex_index = *bytemuck::from_bytes::<u32>(
            self.bytes(self.array_variable(self.built_in_variable(BuiltIn::VertexIndex))),
        );
        let clip_distances = *bytemuck::from_bytes::<[f32; MAX_CLIP_DISTANCES as usize]>(
            self.bytes(self.array_variable(self.built_in_variable(BuiltIn::ClipDistance))),
        );
        VertexShaderOutput {
            position,
            point_size,
            vertex_index,
            clip_distances,
        }
    }

    fn set_fragment_shader_input(&mut self, fragment: Fragment) {
        let frag_coord = self.add_input_variable(&fragment.position.get_as_f32_array());
        self.built_in_variables
            .insert(BuiltIn::FragCoord, frag_coord);

        let color = self.add_input_variable(&fragment.color.get_as_f32_array());
        self.location_variables.insert(0, color);

        // TODO: Use descriptors to fetch shader inputs.
    }

    fn fragment_shader_output(&self) -> FragmentShaderOutput {
        let position = Vector4::from_vertex_buffer_bytes(
            Format::R32G32B32A32Sfloat,
            self.bytes(self.array_variable(self.built_in_variable(BuiltIn::FragCoord))),
        );
        // TODO: Determine the color from the fragment shader output interface.
        let color = Vector4::from_vertex_buffer_bytes(
            Format::R32G32B32A32Sfloat,
            self.bytes(self.array_variable(self.location_variable(0))),
        );
        FragmentShaderOutput { position, color }
    }
}

impl State {
    /// Returns the raw bytes backing an array variable.
    fn bytes(&self, variable: ArrayVariable) -> &[u8] {
        self.memory(&variable.memory_region)
    }

    /// Reinterprets an array variable's memory region as a slice of `T`.
    ///
    /// # Panics
    ///
    /// Panics unless the region is aligned for `T` and its size is a multiple
    /// of `size_of::<T>()`.
    fn pod_slice<T: bytemuck::Pod>(&self, variable: ArrayVariable) -> &[T] {
        bytemuck::cast_slice(self.bytes(variable))
    }

    /// Reads one 32-bit word from an array variable's memory region.
    ///
    /// # Panics
    ///
    /// Panics unless the region is exactly one word and is word-aligned.
    fn word(&self, variable: ArrayVariable) -> u32 {
        *bytemuck::from_bytes(self.bytes(variable))
    }

    /// Copies `bytes` into an array variable's memory region.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` does not fit in the region.
    fn store_bytes(&mut self, variable: ArrayVariable, bytes: &[u8]) {
        let region = variable.memory_region;
        self.memory_mut(&region)[..bytes.len()].copy_from_slice(bytes);
    }

    /// Copies `values` into an array variable's memory region.
    ///
    /// # Panics
    ///
    /// Panics if `values` does not fit in the region.
    fn store_pod<T: bytemuck::Pod>(&mut self, variable: ArrayVariable, values: &[T]) {
        self.store_bytes(variable, bytemuck::cast_slice(values));
    }
}

impl State {
    fn il_add_new_variable(&mut self, il_variable: &il::Variable, decl: &il::VariableDecl) {
        assert!(
            !self.il_variables.contains_key(il_variable),
            "IL variable is declared more than once"
        );
        let variable = Variable::from_il(decl, self);
        self.il_variables.insert(*il_variable, variable);
    }

    fn il_variable(&self, il_variable: &il::Variable) -> Variable {
        *self
            .il_variables
            .get(il_variable)
            .expect("IL variable is declared before it is used")
    }

    fn il_load_offset_to_pointer(
        &mut self,
        result: &il::Variable,
        base: &il::Variable,
        offsets: &[il::Variable],
    ) {
        let offsets = offsets
            .iter()
            .map(|offset| {
                let variable = self.array_variable(self.il_variable(offset));
                self.word(variable)
            })
            .collect::<Vec<_>>();

        let mut src = *self
            .pointer_variable(self.il_variable(base))
            .pointer
            .as_ref()
            .expect("pointer variable has a target");

        for offset in offsets {
            src = match src {
                Variable::Array(_) => {
                    self.add_array_variable(self.array_variable(src).indexed(offset))
                }
                Variable::Struct(_) => self.struct_variable(src).members[to_usize(offset)],
                Variable::Pointer(_) => {
                    unreachable!("cannot index through a pointer variable");
                }
            };
        }

        let pointer = self
            .pointer_variable_mut(self.il_variable(result))
            .pointer
            .as_mut()
            .expect("pointer variable has a target");
        *pointer = src;
    }

    fn il_load_pointer_to_variable(&mut self, result: &il::Variable, src_pointer: &il::Variable) {
        let src = self.array_variable(
            self.pointer_variable(self.il_variable(src_pointer))
                .pointer
                .expect("pointer variable has a target"),
        );
        let result = self.array_variable(self.il_variable(result));
        self.store_array(result, src);
    }

    fn il_load_composite_member_to_variable(
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
            Variable::Struct(_) => self.struct_variable(base).members[to_usize(offset)],
            Variable::Pointer(_) => {
                unreachable!("cannot extract a member from a pointer variable");
            }
        };

        let result = self.array_variable(self.il_variable(result));
        let src = self.array_variable(src);
        self.store_array(result, src);
    }

    fn il_store_variables_to_composite(&mut self, dst: &il::Variable, values: &[il::Variable]) {
        let dst = self.il_variable(dst);
        let values = values
            .iter()
            .map(|value| self.il_variable(value))
            .collect::<Vec<_>>();

        match dst {
            Variable::Array(_) => {
                let mut index = 0;
                for value in values {
                    let src = self.array_variable(value);
                    let dst = self.array_variable(dst).indexed(index);
                    self.store_array(dst, src);
                    index += src.len();
                }
            }
            Variable::Struct(_) => unimplemented!("storing values in struct variables"),
            Variable::Pointer(_) => unreachable!("composite destination cannot be a pointer"),
        }
    }

    fn il_store_through_pointer(&mut self, dst_pointer: &il::Variable, src: &il::Variable) {
        let src = self.il_variable(src);
        let dst = self
            .pointer_variable(self.il_variable(dst_pointer))
            .pointer
            .expect("pointer variable has a target");

        // TODO: Match identical object types before storing.
        self.store_array(self.array_variable(dst), self.array_variable(src));
    }

    fn il_store_imm32(&mut self, variable: &il::Variable, values: &[u32]) {
        let variable = self.array_variable(self.il_variable(variable));
        self.store_pod(variable, values);
    }

    fn il_select(
        &mut self,
        result: &il::Variable,
        cond: &il::Variable,
        obj1: &il::Variable,
        obj2: &il::Variable,
    ) {
        let result = self.array_variable(self.il_variable(result));
        let cond_variable = self.array_variable(self.il_variable(cond));
        let obj1 = self.array_variable(self.il_variable(obj1));
        let obj2 = self.array_variable(self.il_variable(obj2));
        assert!(
            cond_variable.is_bool(),
            "select condition must be a boolean"
        );
        let condition = self.bytes(cond_variable) != [0];
        self.store_array(result, if condition { obj1 } else { obj2 });
    }

    fn store_array(&mut self, dst: ArrayVariable, src: ArrayVariable) {
        self.copy_memory_region(dst.memory_region, src.memory_region);
    }
}

impl State {
    fn il_binary_op(
        &mut self,
        result: &il::Variable,
        op1: &il::Variable,
        op2: &il::Variable,
        op: BinaryOp,
    ) {
        let result = self.array_variable(self.il_variable(result));
        let op1 = self.array_variable(self.il_variable(op1));
        let op2 = self.array_variable(self.il_variable(op2));
        match op {
            BinaryOp::MulVectorScalar => {
                let scalar = self.pod_slice::<f32>(op2)[0];
                let values = self
                    .pod_slice::<f32>(op1)
                    .iter()
                    .map(|value| value * scalar)
                    .collect::<Vec<_>>();
                self.store_pod(result, &values);
            }
            BinaryOp::AddI32I32 => self.binary_map::<i32, i32>(result, op1, op2, |a, b| a + b),
            BinaryOp::MulI32I32 => self.binary_map::<i32, i32>(result, op1, op2, |a, b| a * b),
            BinaryOp::SubF32F32 => self.binary_map::<f32, f32>(result, op1, op2, |a, b| a - b),
            BinaryOp::DivF32F32 => self.binary_map::<f32, f32>(result, op1, op2, |a, b| a / b),
            BinaryOp::DivI32I32 => self.binary_map::<i32, i32>(result, op1, op2, |a, b| a / b),
            BinaryOp::DivU32U32 => self.binary_map::<u32, u32>(result, op1, op2, |a, b| a / b),
            BinaryOp::ModI32I32 => self.binary_map::<i32, i32>(result, op1, op2, |a, b| a % b),
            BinaryOp::ModU32U32 => self.binary_map::<u32, u32>(result, op1, op2, |a, b| a % b),
            BinaryOp::BitAnd => self.binary_map::<u32, u32>(result, op1, op2, |a, b| a & b),
            BinaryOp::BitShiftRight => {
                self.binary_map::<u32, u32>(result, op1, op2, |a, b| a >> b);
            }
            BinaryOp::EqualI32I32 => {
                self.binary_map::<i32, u8>(result, op1, op2, |a, b| u8::from(a == b));
            }
            BinaryOp::LessThanU32U32 => {
                self.binary_map::<u32, u8>(result, op1, op2, |a, b| u8::from(a < b));
            }
        }
    }

    fn binary_map<T, U>(
        &mut self,
        result: ArrayVariable,
        op1: ArrayVariable,
        op2: ArrayVariable,
        op: impl Fn(T, T) -> U,
    ) where
        T: bytemuck::Pod,
        U: bytemuck::Pod,
    {
        let op1 = self.pod_slice::<T>(op1);
        let op2 = self.pod_slice::<T>(op2);
        let values = op1
            .iter()
            .copied()
            .zip(op2.iter().copied())
            .map(|(a, b)| op(a, b))
            .collect::<Vec<U>>();
        self.store_pod(result, &values);
    }

    fn il_convert(&mut self, result: &il::Variable, operand: &il::Variable, op: UnaryOp) {
        let result = self.array_variable(self.il_variable(result));
        let operand = self.array_variable(self.il_variable(operand));
        match op {
            UnaryOp::ConvertI32ToF32 => {
                self.unary_map::<i32, f32>(result, operand, |value| value as f32);
            }
            UnaryOp::ConvertF32ToU32 => {
                self.unary_map::<f32, u32>(result, operand, |value| value as u32);
            }
            UnaryOp::ConvertU32ToF32 => {
                self.unary_map::<u32, f32>(result, operand, |value| value as f32);
            }
        }
    }

    fn unary_map<T, U>(
        &mut self,
        result: ArrayVariable,
        operand: ArrayVariable,
        op: impl Fn(T) -> U,
    ) where
        T: bytemuck::Pod,
        U: bytemuck::Pod,
    {
        let values = self
            .pod_slice::<T>(operand)
            .iter()
            .copied()
            .map(op)
            .collect::<Vec<U>>();
        self.store_pod(result, &values);
    }
}

impl State {
    fn interpret_il_instruction(&mut self, instruction: &il::Instruction) -> bool {
        match instruction {
            il::Instruction::Label { .. } => {}
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
                self.il_load_offset_to_pointer(id, base, offsets);
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
            il::Instruction::MathBinary { id, op1, op2, op } => {
                self.il_binary_op(id, op1, op2, *op);
            }
            il::Instruction::MathUnary { id, operand, op } => {
                self.il_convert(id, operand, *op);
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
                self.il_select(id, cond, obj1, obj2);
            }
            il::Instruction::SelectionMerge { .. } => {
                todo!("selection merge is not implemented");
            }
            il::Instruction::LoopMerge { .. } => {
                todo!("loop merge is not implemented");
            }
            il::Instruction::Branch { .. } => {
                todo!("branching is not implemented");
            }
            il::Instruction::BranchConditional { .. } => {
                todo!("conditional branching is not implemented");
            }
            il::Instruction::Kill => {
                todo!("discard is not implemented");
            }
        }
        self.pc += 1;
        false
    }
}

fn region_range(region: &MemoryRegion) -> Range<usize> {
    let start = to_usize(region.address);
    let end = to_usize(region.address + region.size);
    start..end
}

fn to_usize(value: u32) -> usize {
    usize::try_from(value).expect("SPIR-V word fits in usize")
}
