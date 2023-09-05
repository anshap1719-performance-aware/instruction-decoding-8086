use crate::instructions::arithmetic::{ArithmeticInstruction, ArithmeticInstructionTypes};
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

pub struct AddInstruction {
    variant: ArithmeticInstructionTypes,
    is_destination: Option<DestinationFirst>,
    is_wide: Wide,
    mode: Option<InstructionMode>,
    source: Operand,
    destination: Operand,
}

impl Display for AddInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("add ")?;

        if self.variant == ArithmeticInstructionTypes::ImmediateToRegisterOrMemory {
            if let Operand::Memory(_) = self.destination {
                if let Operand::Immediate(value) = self.source {
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
        }

        self.destination.fmt(f)?;
        f.write_str(", ")?;

        self.source.fmt(f)?;

        Ok(())
    }
}

impl ArithmeticInstruction for AddInstruction {
    fn new(
        variant: ArithmeticInstructionTypes,
        is_destination: Option<DestinationFirst>,
        is_wide: Wide,
        mode: Option<InstructionMode>,
        source: Operand,
        destination: Operand,
    ) -> Self {
        AddInstruction {
            variant,
            is_destination,
            is_wide,
            mode,
            source,
            destination,
        }
    }
}
