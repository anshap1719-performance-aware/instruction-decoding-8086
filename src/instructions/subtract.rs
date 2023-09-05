use crate::instructions::arithmetic::{ArithmeticInstruction, ArithmeticInstructionTypes};
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::memory::EffectiveAddress;
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

pub struct SubtractInstruction {
    variant: ArithmeticInstructionTypes,
    is_destination: Option<DestinationFirst>,
    is_wide: Wide,
    mode: Option<InstructionMode>,
    source: Operand,
    destination: Operand,
}

impl Display for SubtractInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("sub ")?;

        if self.variant == ArithmeticInstructionTypes::ImmediateToRegisterOrMemory {
            if let Operand::Memory(address) = self.destination {
                if let EffectiveAddress::RegisterPlusByte(_, _)
                | EffectiveAddress::RegisterSumPlusByte(_, _, _) = address
                {
                    f.write_str("byte ");
                } else if let EffectiveAddress::RegisterPlusWord(_, _)
                | EffectiveAddress::RegisterSumPlusWord(_, _, _) = address
                {
                    f.write_str("word ");
                }
            }
        }

        self.destination.fmt(f)?;
        f.write_str(", ")?;

        self.source.fmt(f)?;

        Ok(())
    }
}

impl ArithmeticInstruction for SubtractInstruction {
    fn new(
        variant: ArithmeticInstructionTypes,
        is_destination: Option<DestinationFirst>,
        is_wide: Wide,
        mode: Option<InstructionMode>,
        source: Operand,
        destination: Operand,
    ) -> Self {
        SubtractInstruction {
            variant,
            is_destination,
            is_wide,
            mode,
            source,
            destination,
        }
    }
}
