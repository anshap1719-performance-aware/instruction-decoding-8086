use crate::instructions::operands::{ImmediateValue, Operand};
use crate::memory::EffectiveAddress;
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Display, Formatter, Write};
use std::io::BufReader;

#[derive(Copy, Clone, PartialEq)]
enum MovInstructionTypes {
    RegisterOrMemoryToOrFromRegister,
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
            value if bit_match!(value, (1, 1, 0, 0, 0, 1, 1, _)) => {
                Self::ImmediateToRegisterOrMemory
            }
            value if bit_match!(value, (1, 0, 1, 1, _, _, _, _)) => Self::ImmediateToRegister,
            value if bit_match!(value, (1, 0, 1, 0, 0, 0, 0, _)) => Self::MemoryToAccumulator,
            value if bit_match!(value, (1, 0, 1, 0, 0, 0, 1, _)) => Self::AccumulatorToMemory,
            _ => panic!("Unable to decode instruction type"),
        }
    }
}

pub struct MovInstruction {
    variant: MovInstructionTypes,
    is_destination: Option<DestinationFirst>,
    is_wide: Wide,
    mode: Option<InstructionMode>,
    source: Operand,
    destination: Operand,
}

impl Display for MovInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("mov ")?;

        self.destination.fmt(f)?;
        f.write_str(", ")?;

        if self.variant == MovInstructionTypes::ImmediateToRegisterOrMemory {
            if let Operand::Memory(_) = self.destination {
                if let Operand::Immediate(value) = self.source {
                    match value {
                        ImmediateValue::SignedByte(_) => {
                            f.write_str("byte ");
                        }
                        ImmediateValue::SignedWord(_) => {
                            f.write_str("word ");
                        }
                    }
                }
            }
        }

        self.source.fmt(f)?;

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

                MovInstruction {
                    variant,
                    is_destination: Some(is_destination),
                    is_wide,
                    mode: Some(mode),
                    source: if is_destination {
                        register_or_memory
                    } else {
                        register
                    },
                    destination: if is_destination {
                        register
                    } else {
                        register_or_memory
                    },
                }
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

                MovInstruction {
                    variant,
                    is_destination: None,
                    is_wide,
                    mode: None,
                    source: Operand::Immediate(data),
                    destination: Operand::Register(Register::from(register_byte)),
                }
            }
            MemoryToAccumulator => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));
                let memory_location = reader.read_u16::<LittleEndian>().unwrap();

                MovInstruction {
                    variant,
                    is_destination: None,
                    is_wide,
                    mode: None,
                    source: Operand::Memory(EffectiveAddress::DirectAddress(memory_location)),
                    destination: if is_wide {
                        Operand::AccumulatorWide
                    } else {
                        Operand::Accumulator
                    },
                }
            }
            AccumulatorToMemory => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));
                let memory_location = reader.read_u16::<LittleEndian>().unwrap();

                MovInstruction {
                    variant,
                    is_destination: None,
                    is_wide,
                    mode: None,
                    source: if is_wide {
                        Operand::AccumulatorWide
                    } else {
                        Operand::Accumulator
                    },
                    destination: Operand::Memory(EffectiveAddress::DirectAddress(memory_location)),
                }
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

                MovInstruction {
                    variant,
                    is_destination: None,
                    is_wide,
                    mode: Some(mode),
                    source: Operand::Immediate(data),
                    destination: register_or_memory,
                }
            }
        }
    }
}
