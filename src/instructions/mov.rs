use crate::memory::{EffectiveAddress, InstructionMode};
use crate::prelude::*;
use crate::register::Register;
use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use std::io::BufReader;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
enum ImmediateValue {
    Byte(u8),
    Word(u16),
}

impl Display for ImmediateValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmediateValue::Byte(value) => Display::fmt(value, f),
            ImmediateValue::Word(value) => Display::fmt(value, f),
        }
    }
}

#[derive(Copy, Clone)]
enum MovTarget {
    Accumulator,
    Register(Register),
    Memory(EffectiveAddress),
    Immediate(ImmediateValue),
}

impl Display for MovTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MovTarget::Accumulator => f.write_str("ax"),
            MovTarget::Register(register) => register.fmt(f),
            MovTarget::Memory(memory) => memory.fmt(f),
            MovTarget::Immediate(immediate) => immediate.fmt(f),
        }
    }
}

impl From<(&mut BufReader<File>, InstructionMode, Byte, Wide)> for MovTarget {
    fn from(
        (reader, mode, target_specifier_byte, is_wide): (
            &mut BufReader<File>,
            InstructionMode,
            Byte,
            Wide,
        ),
    ) -> Self {
        let mut mem_bytes = 0b00_000_111 & target_specifier_byte;

        if let InstructionMode::Register = mode {
            let mut mem_bytes = mem_bytes << 1;

            if is_wide {
                mem_bytes += 1;
            }

            MovTarget::Register(Register::from(mem_bytes))
        } else {
            MovTarget::Memory(EffectiveAddress::from((reader, mode, mem_bytes)))
        }
    }
}

pub struct MovInstruction {
    variant: MovInstructionTypes,
    is_destination: Option<DestinationFirst>,
    is_wide: Wide,
    mode: Option<InstructionMode>,
    source: MovTarget,
    destination: MovTarget,
}

impl Display for MovInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("mov ")?;

        self.destination.fmt(f)?;
        f.write_char(',')?;
        self.source.fmt(f)?;

        Ok(())
    }
}

impl From<(&mut BufReader<File>, Byte)> for MovInstruction {
    fn from((reader, instruction_byte): (&mut BufReader<File>, Byte)) -> Self {
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

                let register = MovTarget::Register(Register::from(register_byte));
                let register_or_memory =
                    MovTarget::from((reader, mode, target_specifiers, is_wide));

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
                    ImmediateValue::Word(reader.read_u16::<LittleEndian>().unwrap())
                } else {
                    ImmediateValue::Byte(reader.read_u8().unwrap())
                };

                MovInstruction {
                    variant,
                    is_destination: None,
                    is_wide,
                    mode: None,
                    source: MovTarget::Immediate(data),
                    destination: MovTarget::Register(Register::from(register_byte)),
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
                    source: MovTarget::Memory(EffectiveAddress::DirectAddress(memory_location)),
                    destination: MovTarget::Accumulator,
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
                    source: MovTarget::Accumulator,
                    destination: MovTarget::Memory(EffectiveAddress::DirectAddress(
                        memory_location,
                    )),
                }
            }
            ImmediateToRegisterOrMemory => panic!("Not yet supported"),
        }
    }
}
