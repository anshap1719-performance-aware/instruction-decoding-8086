use crate::instructions::operands::Operand;
use crate::instructions::operands::Operand::{Accumulator, AccumulatorWide};
use crate::mode::InstructionMode;
use crate::{DestinationFirst, Wide};
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq)]
pub struct AnyInstruction {
    pub is_wide: Wide,
    pub mode: Option<InstructionMode>,
    pub source: Option<Operand>,
    pub destination: Operand,
}

impl Default for AnyInstruction {
    fn default() -> Self {
        Self {
            is_wide: false,
            mode: None,
            source: None,
            destination: AccumulatorWide,
        }
    }
}

pub trait Instruction {
    const MEMORY_SIZE: u8;

    fn execute(&self);
}
