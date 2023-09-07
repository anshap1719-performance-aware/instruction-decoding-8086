use crate::instructions::operands::Operand;
use crate::mode::InstructionMode;
use crate::{DestinationFirst, Wide};
use std::fmt::Display;

pub trait Instruction {
    fn execute(&self);
}
