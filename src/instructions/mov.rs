use crate::instructions::operands::{ImmediateValue, Operand};
use crate::instructions::{AnyInstruction, Instruction};
use crate::memory::EffectiveAddress;
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use crate::segment_register::SegmentRegister;
use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Display, Formatter};
use std::io::BufReader;

#[derive(Copy, Clone, PartialEq)]
enum MovInstructionTypes {
    RegisterOrMemoryToOrFromRegister,
    RegisterOrMemoryToOrFromSegmentRegister,
    ImmediateToRegisterOrMemory,
    ImmediateToRegister,
    MemoryToAccumulator,
    AccumulatorToMemory,
}

impl From<Byte> for MovInstructionTypes {
    fn from(value: Byte) -> Self {
        match value {
            value if bit_match!(value, (1, 0, 0, 0, 1, 0, _, _)) => {
                Self::RegisterOrMemoryToOrFromRegister
            }
            value if bit_match!(value, (1, 0, 0, 0, 1, 1, _, _)) => {
                Self::RegisterOrMemoryToOrFromSegmentRegister
            }
            value if bit_match!(value, (1, 1, 0, 0, 0, 1, 1, _)) => {
                Self::ImmediateToRegisterOrMemory
            }
            value if bit_match!(value, (1, 0, 1, 1, _, _, _, _)) => Self::ImmediateToRegister,
            value if bit_match!(value, (1, 0, 1, 0, 0, 0, 0, _)) => Self::MemoryToAccumulator,
            value if bit_match!(value, (1, 0, 1, 0, 0, 0, 1, _)) => Self::AccumulatorToMemory,
            value => panic!("Unable to decode instruction type: {value:b}"),
        }
    }
}

pub struct MovInstruction(pub AnyInstruction);

impl Instruction for MovInstruction {
    fn execute(
        &self,
        register_store: &mut RegisterManager,
        memory_store: &mut MemoryManager,
        segment_register_store: &mut SegmentRegisterManager,
    ) {
        let MovInstruction(AnyInstruction {
            source,
            destination,
            ..
        }) = self;

        let source: Option<ImmediateValue> = match source {
            None => None,
            Some(source) => match source {
                Operand::Accumulator => Some(ImmediateValue::SignedByte(i8::from_le_bytes([
                    register_store.read_byte_from_register(Register::Al),
                ]))),
                Operand::AccumulatorWide => Some(ImmediateValue::SignedWord(i16::from_le_bytes(
                    register_store
                        .read_word_from_register(Register::Ax)
                        .to_le_bytes(),
                ))),
                Operand::Register(register) => {
                    if self.0.is_wide {
                        Some(ImmediateValue::SignedWord(i16::from_le_bytes(
                            register_store
                                .read_word_from_register(*register)
                                .to_le_bytes(),
                        )))
                    } else {
                        Some(ImmediateValue::SignedByte(i8::from_le_bytes([
                            register_store.read_byte_from_register(*register),
                        ])))
                    }
                }
                Operand::Memory(address) => Some(memory_store.read_memory_from_effective_address(
                    *address,
                    self.0.is_wide,
                    register_store,
                )),
                Operand::Immediate(immediate_value) => Some(*immediate_value),
                Operand::SegmentRegister(register) => {
                    if self.0.is_wide {
                        Some(ImmediateValue::SignedWord(
                            segment_register_store.read_word_from_segment_register(*register)
                                as i16,
                        ))
                    } else {
                        Some(ImmediateValue::SignedByte(
                            segment_register_store.read_byte_from_segment_register(*register) as i8,
                        ))
                    }
                }
            },
        };

        if let Some(source) = source {
            match destination {
                Operand::Accumulator => match source {
                    ImmediateValue::SignedByte(value) => {
                        register_store.write_byte_to_register(
                            Register::Al,
                            u8::from_le_bytes(value.to_le_bytes()),
                        );
                    }
                    ImmediateValue::SignedWord(_) => {
                        panic!("Cannot write word to Al")
                    }
                },
                Operand::AccumulatorWide => match source {
                    ImmediateValue::SignedByte(value) => {
                        register_store.write_byte_to_register(
                            Register::Al,
                            u8::from_le_bytes(value.to_le_bytes()),
                        );
                    }
                    ImmediateValue::SignedWord(value) => register_store.write_word_to_register(
                        Register::Ax,
                        u16::from_le_bytes(value.to_le_bytes()),
                    ),
                },
                Operand::Register(register) => match source {
                    ImmediateValue::SignedByte(value) => {
                        register_store.write_byte_to_register(
                            *register,
                            u8::from_le_bytes(value.to_le_bytes()),
                        );
                    }
                    ImmediateValue::SignedWord(value) => register_store
                        .write_word_to_register(*register, u16::from_le_bytes(value.to_le_bytes())),
                },
                Operand::Memory(address) => memory_store.write_to_effective_memory_address(
                    *address,
                    self.0.is_wide,
                    register_store,
                    source,
                ),
                Operand::Immediate(_) => panic!("Cannot move a value to immediate"),
                Operand::SegmentRegister(register) => {
                    if self.0.is_wide {
                        segment_register_store
                            .write_word_to_segment_register(*register, source.into())
                    } else {
                        segment_register_store
                            .write_byte_to_segment_register(*register, source.try_into().unwrap())
                    }
                }
            }
        } else {
            panic!("Mov instruction expects both a source and a destination");
        }
    }
}

impl Display for MovInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("mov ")?;

        self.0.destination.fmt(f)?;
        f.write_str(", ")?;

        if let Some(source) = self.0.source {
            if let Operand::Memory(_) = self.0.destination {
                if let Some(Operand::Immediate(value)) = self.0.source {
                    match value {
                        ImmediateValue::SignedByte(_) => {
                            f.write_str("byte ")?;
                        }
                        ImmediateValue::SignedWord(_) => {
                            f.write_str("word ")?;
                        }
                    }
                }
            }

            source.fmt(f)?;
        }

        Ok(())
    }
}

impl MovInstruction {
    pub fn read(reader: &mut BufReader<File>, instruction_byte: Byte) -> Self {
        use MovInstructionTypes::*;

        let variant = MovInstructionTypes::from(instruction_byte);

        match variant {
            RegisterOrMemoryToOrFromRegister => {
                let is_destination = bit_match!(instruction_byte, (_, _, _, _, _, _, 1, _));
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));

                let target_specifiers = reader.read_u8().expect("Failed to read instruction type");
                let mode = InstructionMode::from(target_specifiers);

                let register_byte = (0b00_111_000 & target_specifiers) >> 3;
                let mut register_byte = register_byte << 1;

                if is_wide {
                    register_byte += 1;
                }

                let register_byte = register_byte;

                let register = Operand::Register(Register::from(register_byte));
                let register_or_memory = Operand::read(reader, mode, target_specifiers, is_wide);

                MovInstruction(AnyInstruction {
                    is_wide,
                    mode: Some(mode),
                    source: Some(if is_destination {
                        register_or_memory
                    } else {
                        register
                    }),
                    destination: if is_destination {
                        register
                    } else {
                        register_or_memory
                    },
                })
            }
            RegisterOrMemoryToOrFromSegmentRegister => {
                let is_destination = bit_match!(instruction_byte, (_, _, _, _, _, _, 1, _));
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));

                let target_specifiers = reader.read_u8().expect("Failed to read instruction type");
                let mode = InstructionMode::from(target_specifiers);

                let segment_register_byte = (0b00_011_000 & target_specifiers) >> 3;

                let segment_register =
                    Operand::SegmentRegister(SegmentRegister::from(segment_register_byte));
                let register_or_memory = Operand::read(reader, mode, target_specifiers, is_wide);

                MovInstruction(AnyInstruction {
                    is_wide,
                    mode: Some(mode),
                    source: Some(if is_destination {
                        register_or_memory
                    } else {
                        segment_register
                    }),
                    destination: if is_destination {
                        segment_register
                    } else {
                        register_or_memory
                    },
                })
            }
            ImmediateToRegister => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, 1, _, _, _));
                let mut register_byte = (0b00_000_111 & instruction_byte) << 1;

                if is_wide {
                    register_byte += 1;
                }

                let data = if is_wide {
                    ImmediateValue::SignedWord(reader.read_i16::<LittleEndian>().unwrap())
                } else {
                    ImmediateValue::SignedByte(reader.read_i8().unwrap())
                };

                MovInstruction(AnyInstruction {
                    is_wide,
                    mode: None,
                    source: Some(Operand::Immediate(data)),
                    destination: Operand::Register(Register::from(register_byte)),
                })
            }
            MemoryToAccumulator => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));
                let memory_location = reader.read_u16::<LittleEndian>().unwrap();

                MovInstruction(AnyInstruction {
                    is_wide,
                    mode: None,
                    source: Some(Operand::Memory(EffectiveAddress::DirectAddress(
                        memory_location,
                    ))),
                    destination: if is_wide {
                        Operand::AccumulatorWide
                    } else {
                        Operand::Accumulator
                    },
                })
            }
            AccumulatorToMemory => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));
                let memory_location = reader.read_u16::<LittleEndian>().unwrap();

                MovInstruction(AnyInstruction {
                    is_wide,
                    mode: None,
                    source: Some(if is_wide {
                        Operand::AccumulatorWide
                    } else {
                        Operand::Accumulator
                    }),
                    destination: Operand::Memory(EffectiveAddress::DirectAddress(memory_location)),
                })
            }
            ImmediateToRegisterOrMemory => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));

                let target_specifiers = reader.read_u8().expect("Failed to read instruction type");
                let mode = InstructionMode::from(target_specifiers);

                let register_or_memory = Operand::read(reader, mode, target_specifiers, is_wide);

                let data = if is_wide {
                    ImmediateValue::SignedWord(reader.read_i16::<LittleEndian>().unwrap())
                } else {
                    ImmediateValue::SignedByte(reader.read_i8().unwrap())
                };

                MovInstruction(AnyInstruction {
                    is_wide,
                    mode: Some(mode),
                    source: Some(Operand::Immediate(data)),
                    destination: register_or_memory,
                })
            }
        }
    }
}
