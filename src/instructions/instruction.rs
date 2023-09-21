use crate::instructions::operands::Operand;
use crate::instructions::operands::Operand::AccumulatorWide;
use crate::mode::InstructionMode;
use crate::store::Store;
use crate::Wide;
use std::fs::File;
use std::io::BufReader;

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
    fn execute(&self, reader: &BufReader<File>, store: &mut Store);
}
