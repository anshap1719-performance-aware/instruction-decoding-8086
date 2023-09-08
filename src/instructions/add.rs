use crate::instructions::arithmetic::ArithmeticInstruction;
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::instructions::{AnyInstruction, Instruction};
use crate::memory::MemoryManager;
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::RegisterManager;
use std::fmt::{Display, Formatter};

pub struct AddInstruction(pub AnyInstruction);

impl Display for AddInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("add ")?;

        if let Operand::Memory(_) = self.0.destination {
            if let Some(Operand::Immediate(value)) = self.0.source {
                match value {
                    ImmediateValue::SignedByte(_) => {
                        f.write_str("byte ")?;
                    }
                    ImmediateValue::SignedWord(_) => {
                        f.write_str("word ")?;
                    }
                }
            }
        }

        self.0.destination.fmt(f)?;

        if let Some(source) = self.0.source {
            f.write_str(", ")?;
            source.fmt(f)?;
        }

        Ok(())
    }
}

impl ArithmeticInstruction for AddInstruction {
    fn new(
        is_wide: Wide,
        mode: Option<InstructionMode>,
        source: Operand,
        destination: Operand,
    ) -> Self {
        AddInstruction(AnyInstruction {
            is_wide,
            mode,
            source: Some(source),
            destination,
        })
    }
}

impl Instruction for AddInstruction {
    fn execute(&self, register_store: &mut RegisterManager, memory_store: &mut MemoryManager) {
        todo!()
    }
}
