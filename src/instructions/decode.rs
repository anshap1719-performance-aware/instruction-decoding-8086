use super::mov::*;
use crate::cycle::EstimatedCycleCount;
use crate::instructions::add::AddInstruction;
use crate::instructions::arithmetic::ArithmeticInstruction;
use crate::instructions::compare::CompareInstruction;
use crate::instructions::jump::JumpInstructions;
use crate::instructions::subtract::SubtractInstruction;
use crate::instructions::Instruction;
use crate::prelude::*;
use crate::store::Store;
use byteorder::ReadBytesExt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

pub enum Instructions {
    Mov(MovInstruction),
    Add(AddInstruction),
    Sub(SubtractInstruction),
    Cmp(CompareInstruction),
    Jump(JumpInstructions),
}

impl EstimatedCycleCount for Instructions {
    fn num_cycles(&self) -> u32 {
        match self {
            Instructions::Mov(instruction) => instruction.num_cycles(),
            Instructions::Add(instruction) => instruction.num_cycles(),
            Instructions::Sub(instruction) => instruction.num_cycles(),
            Instructions::Cmp(instruction) => instruction.num_cycles(),
            Instructions::Jump(instruction) => instruction.num_cycles(),
        }
    }
}

impl Instruction for Instructions {
    fn execute(&self, reader: &mut BufReader<File>, store: &mut Store) -> u32 {
        match self {
            Instructions::Mov(instruction) => instruction.execute(reader, store),
            Instructions::Add(instruction) => instruction.execute(reader, store),
            Instructions::Sub(instruction) => instruction.execute(reader, store),
            Instructions::Cmp(instruction) => instruction.execute(reader, store),
            Instructions::Jump(instruction) => instruction.execute(reader, store),
        }
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Mov(instruction) => instruction.fmt(f),
            Instructions::Add(instruction) => instruction.fmt(f),
            Instructions::Sub(instruction) => instruction.fmt(f),
            Instructions::Cmp(instruction) => instruction.fmt(f),
            Instructions::Jump(instruction) => instruction.fmt(f),
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
            value if JumpInstructions::is_jump_instruction(value) => {
                Instructions::Jump(JumpInstructions::try_from((value, reader)).unwrap())
            }
            _ => panic!("Unsupported instruction: {value}"),
        }
    }
}
