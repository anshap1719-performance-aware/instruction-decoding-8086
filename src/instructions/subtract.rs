use crate::instructions::arithmetic::{ArithmeticInstruction, ArithmeticInstructionTypes};
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::instructions::AnyInstruction;
use crate::memory::EffectiveAddress;
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

pub struct SubtractInstruction(pub AnyInstruction);

impl Display for SubtractInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("sub ")?;

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

impl ArithmeticInstruction for SubtractInstruction {
    fn new(
        is_wide: Wide,
        mode: Option<InstructionMode>,
        source: Operand,
        destination: Operand,
    ) -> Self {
        SubtractInstruction(AnyInstruction {
            is_wide,
            mode,
            source: Some(source),
            destination,
        })
    }
}
