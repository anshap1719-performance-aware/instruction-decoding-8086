use crate::instructions::arithmetic::ArithmeticInstruction;
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::instructions::{AnyInstruction, Instruction};
use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::store::Store;
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
    fn execute(&self, _reader: &BufReader<File>, store: &mut Store) {
        let AddInstruction(AnyInstruction {
            source,
            destination,
            ..
        }) = self;

        let rhs = source
            .as_ref()
            .map(|source| source.to_immediate_value(self.0.is_wide, store))
            .expect("add operation expects both source and destination");

        let lhs = destination.to_immediate_value(self.0.is_wide, store);

        let op_result = lhs + rhs;

        store.flag_register_store_mut().set_flags_on_op(op_result);

        destination.write_value(op_result.value(), self.0.is_wide, store);
    }
}
