use super::mov::*;
use crate::instructions::add::AddInstruction;
use crate::instructions::arithmetic::ArithmeticInstruction;
use crate::instructions::compare::CompareInstruction;
use crate::instructions::subtract::SubtractInstruction;
use crate::prelude::*;
use byteorder::ReadBytesExt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

pub enum Instructions {
    Mov(MovInstruction),
    Add(AddInstruction),
    Sub(SubtractInstruction),
    Cmp(CompareInstruction),
}

impl Display for Instructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Mov(instruction) => instruction.fmt(f),
            Instructions::Add(instruction) => instruction.fmt(f),
            Instructions::Sub(instruction) => instruction.fmt(f),
            Instructions::Cmp(instruction) => instruction.fmt(f),
        }
    }
}

impl Instructions {
    pub fn read(reader: &mut BufReader<File>, value: Byte) -> Self {
        match value {
            value
                if bit_match!(value, (1, 0, 0, 0, 1, 0, _, _))
                    | bit_match!(value, (1, 1, 0, 0, 0, 1, 1, _))
                    | bit_match!(value, (1, 0, 1, 1, _, _, _, _))
                    | bit_match!(value, (1, 0, 1, 0, 0, 0, _, _))
                    | bit_match!(value, (1, 0, 0, 0, 1, 1, 1, 0))
                    | bit_match!(value, (1, 0, 0, 0, 1, 1, 0, 0)) =>
            {
                Instructions::Mov(MovInstruction::read(reader, value))
            }
            value
                if bit_match!(value, (0, 0, 0, 0, 0, 0, _, _))
                    | bit_match!(value, (0, 0, 0, 0, 0, 1, 0, _)) =>
            {
                Instructions::Add(AddInstruction::read(reader, value))
            }
            value
                if bit_match!(value, (0, 0, 1, 0, 1, 0, _, _))
                    | bit_match!(value, (0, 0, 1, 0, 1, 1, 0, _)) =>
            {
                Instructions::Sub(SubtractInstruction::read(reader, value))
            }
            value
                if bit_match!(value, (0, 0, 1, 1, 1, 0, _, _))
                    | bit_match!(value, (0, 0, 1, 1, 1, 1, 0, _)) =>
            {
                Instructions::Cmp(CompareInstruction::read(reader, value))
            }
            value if bit_match!(value, (1, 0, 0, 0, 0, 0, _, _)) => {
                let target_specifier_bytes = reader.read_u8().unwrap();
                reader.seek_relative(-1).unwrap();

                match target_specifier_bytes {
                    target if bit_match!(target, (_, _, 0, 0, 0, _, _, _)) => {
                        Instructions::Add(AddInstruction::read(reader, value))
                    }
                    target if bit_match!(target, (_, _, 1, 0, 1, _, _, _)) => {
                        Instructions::Sub(SubtractInstruction::read(reader, value))
                    }
                    target if bit_match!(target, (_, _, 1, 1, 1, _, _, _)) => {
                        Instructions::Cmp(CompareInstruction::read(reader, value))
                    }
                    _ => panic!("Invalid instruction found"),
                }
            }
            _ => panic!("Unsupported instruction"),
        }
    }
}
