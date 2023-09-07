use crate::instructions::add::AddInstruction;
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone, PartialEq)]
pub enum ArithmeticInstructionTypes {
    RegisterOrMemoryToOrFromRegister,
    ImmediateToRegisterOrMemory,
    ImmediateToAccumulator,
}

impl From<Byte> for ArithmeticInstructionTypes {
    fn from(value: Byte) -> Self {
        match value {
            value
                if bit_match!(value, (0, 0, 0, 0, 0, 0, _, _))
                    || bit_match!(value, (0, 0, 1, 0, 1, 0, _, _))
                    || bit_match!(value, (0, 0, 1, 1, 1, 0, _, _)) =>
            {
                Self::RegisterOrMemoryToOrFromRegister
            }
            value if bit_match!(value, (1, 0, 0, 0, 0, 0, _, _)) => {
                Self::ImmediateToRegisterOrMemory
            }
            value
                if bit_match!(value, (0, 0, 0, 0, 0, 1, 0, _))
                    || bit_match!(value, (0, 0, 1, 0, 1, 1, 0, _))
                    || bit_match!(value, (0, 0, 1, 1, 1, 1, 0, _)) =>
            {
                Self::ImmediateToAccumulator
            }
            _ => panic!("Unable to decode instruction type"),
        }
    }
}

pub trait ArithmeticInstruction
where
    Self: Sized,
{
    fn new(
        is_wide: Wide,
        mode: Option<InstructionMode>,
        source: Operand,
        destination: Operand,
    ) -> Self;

    fn read(reader: &mut BufReader<File>, instruction_byte: Byte) -> Self {
        use ArithmeticInstructionTypes::*;

        let variant = ArithmeticInstructionTypes::from(instruction_byte);

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

                Self::new(
                    is_wide,
                    Some(mode),
                    if is_destination {
                        register_or_memory
                    } else {
                        register
                    },
                    if is_destination {
                        register
                    } else {
                        register_or_memory
                    },
                )
            }
            ImmediateToAccumulator => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));

                let data = if is_wide {
                    ImmediateValue::SignedWord(reader.read_i16::<LittleEndian>().unwrap())
                } else {
                    ImmediateValue::SignedByte(reader.read_i8().unwrap())
                };

                Self::new(
                    is_wide,
                    None,
                    Operand::Immediate(data),
                    if is_wide {
                        Operand::AccumulatorWide
                    } else {
                        Operand::Accumulator
                    },
                )
            }
            ImmediateToRegisterOrMemory => {
                let is_wide = bit_match!(instruction_byte, (_, _, _, _, _, _, _, 1));
                let is_signed = bit_match!(instruction_byte, (_, _, _, _, _, _, 1, _));

                let target_specifiers = reader.read_u8().expect("Failed to read instruction type");
                let mode = InstructionMode::from(target_specifiers);

                let register_or_memory = Operand::read(reader, mode, target_specifiers, is_wide);

                let data = if !is_signed && is_wide {
                    ImmediateValue::SignedWord(reader.read_i16::<LittleEndian>().unwrap())
                } else if is_wide {
                    ImmediateValue::SignedWord(i16::from(reader.read_i8().unwrap()))
                } else {
                    ImmediateValue::SignedByte(reader.read_i8().unwrap())
                };

                Self::new(
                    is_wide,
                    Some(mode),
                    Operand::Immediate(data),
                    register_or_memory,
                )
            }
        }
    }
}
