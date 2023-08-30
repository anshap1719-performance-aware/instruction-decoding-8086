mod mov;

use crate::prelude::*;
use crate::{bit_match, compute_mask, compute_val};
use mov::*;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

pub enum Instructions {
    Mov(MovInstruction),
}

impl Display for Instructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Mov(instruction) => instruction.fmt(f),
        }
    }
}

impl From<(&mut BufReader<File>, Byte)> for Instructions {
    fn from((reader, value): (&mut BufReader<File>, Byte)) -> Self {
        match value {
            value
                if bit_match!(value, (1, 0, 0, 0, 1, 0, _, _))
                    | bit_match!(value, (1, 1, 0, 0, 0, 1, 1, _))
                    | bit_match!(value, (1, 0, 1, 1, _, _, _, _))
                    | bit_match!(value, (1, 0, 1, 0, 0, 0, _, _))
                    | bit_match!(value, (1, 0, 0, 0, 1, 1, 1, 0))
                    | bit_match!(value, (1, 0, 0, 0, 1, 1, 0, 0)) =>
            {
                Instructions::Mov(MovInstruction::from((reader, value)))
            }
            _ => panic!("Only mov instructions are supported"),
        }
    }
}
