use crate::memory::{EffectiveAddress, InstructionMode};
use crate::prelude::*;
use crate::register::Register;
use crate::*;
use byteorder::ReadBytesExt;
use std::fmt::{Binary, Display, Formatter, Write};
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
            _ => panic!("Unable to decode instruction type"),
        }
    }
}

#[derive(Copy, Clone)]
enum ImmediateValue {
    Byte(u8),
    Word(u16),
}

#[derive(Copy, Clone)]
enum MovSource {
    Register(u16),
    Memory(u16),
    Immediate(ImmediateValue),
}

#[derive(Copy, Clone)]
enum MovTarget {
    Register(Register),
    Memory(EffectiveAddress),
}

impl Display for MovTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MovTarget::Register(register) => register.fmt(f),
            MovTarget::Memory(memory) => memory.fmt(f),
        }
    }
}

impl From<(&mut BufReader<File>, InstructionMode, Byte, Wide)> for MovTarget {
    fn from(
        (BufReader, mode, target_specifier_byte, is_wide): (
            &mut BufReader<File>,
            InstructionMode,
            Byte,
            Wide,
        ),
    ) -> Self {
        let mut mem_bytes = 0b00000111 & target_specifier_byte;

        if is_wide {
            mem_bytes += 1;
        }

        match mode {
            InstructionMode::Register => MovTarget::Register(Register::from(mem_bytes)),
            _ => MovTarget::Memory(EffectiveAddress::from((
                BufReader,
                mode,
                target_specifier_byte,
            ))),
        }
    }
}

pub struct MovInstruction {
    variant: MovInstructionTypes,
    is_destination: DestinationFirst,
    is_wide: Wide,
    mode: InstructionMode,
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

                let register_byte = (0b00111000 & target_specifiers) >> 3;

                let register = MovTarget::Register(Register::from(register_byte));
                let register_or_memory =
                    MovTarget::from((reader, mode, target_specifiers, is_wide));

                MovInstruction {
                    variant,
                    is_destination,
                    is_wide,
                    mode,
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
            _ => panic!("Not yet supported"),
        }
    }
}
