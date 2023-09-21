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
    fn execute(&self, _reader: &BufReader<File>, store: &mut Store) {
        let MovInstruction(AnyInstruction {
            source,
            destination,
            ..
        }) = self;

        let value: Option<ImmediateValue> = source
            .as_ref()
            .map(|source| source.to_immediate_value(self.0.is_wide, store));

        if let Some(value) = value {
            destination.write_value(value, self.0.is_wide, store);
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

                let target_specifiers = reader.read_u8().expect("Failed to read instruction type");
                let mode = InstructionMode::from(target_specifiers);

                let segment_register_byte = (0b00_011_000 & target_specifiers) >> 3;

                let segment_register =
                    Operand::SegmentRegister(SegmentRegister::from(segment_register_byte));
                let register_or_memory = Operand::read(reader, mode, target_specifiers, true);

                MovInstruction(AnyInstruction {
                    is_wide: true,
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
