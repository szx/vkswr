use crate::shader::spirv;
use crate::shader::spirv::Spirv;
use crate::{Position, Vertex, VertexInputState, VertexShaderOutput};
use hashbrown::HashMap;
use std::sync::Arc;

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
}

#[derive(Debug, Clone)]
enum VariableBacking {
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
    Return,
}

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
                                    for (i, imm) in imm.iter_mut().enumerate() {
                                        *imm = vertex.get_as_uint32(i);
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
        }
    }

    fn from_spirv_decorations(decorations: &spirv::Decorations) -> VariableBacking {
        if let Some(builtin) = decorations.builtin {
            match builtin {
                spirv::BuiltInDecoration::Position => VariableBacking::Position,
                spirv::BuiltInDecoration::PointSize => VariableBacking::PointSize,
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
                assert_eq!(decorations.block, true);
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
