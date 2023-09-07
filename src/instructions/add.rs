use crate::instructions::arithmetic::{ArithmeticInstruction, ArithmeticInstructionTypes};
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::instructions::AnyInstruction;
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

pub struct AddInstruction(pub AnyInstruction);

impl Display for AddInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("add ")?;

        if let Operand::Memory(_) = self.0.destination {
            if let Some(Operand::Immediate(value)) = self.0.source {
                match value {
                    ImmediateValue::SignedByte(_) => {
                        f.write_str("byte ");
                    }
                    ImmediateValue::SignedWord(_) => {
                        f.write_str("word ");
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
