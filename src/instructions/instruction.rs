use crate::instructions::operands::Operand;
use crate::instructions::operands::Operand::AccumulatorWide;
use crate::memory::MemoryManager;
use crate::mode::InstructionMode;
use crate::register::RegisterManager;
use crate::{FlagRegisterManager, SegmentRegisterManager, Wide};

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
    fn execute(
        &self,
        register_store: &mut RegisterManager,
        memory_store: &mut MemoryManager,
        segment_register_store: &mut SegmentRegisterManager,
        flag_register_store: &mut FlagRegisterManager,
    );
}
