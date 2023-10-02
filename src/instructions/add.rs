use crate::cycle::EstimatedCycleCount;
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
            clock_penalty: None,
        })
    }
}

impl EstimatedCycleCount for AddInstruction {
    fn num_cycles(&self) -> u32 {
        use Operand::*;

        match (self.0.destination, self.0.source.unwrap()) {
            (Register(_), Register(_)) => 3,
            (Register(_), Memory(ea)) => 9 + ea.num_cycles(),
            (Memory(ea), Register(_)) => 16 + ea.num_cycles(),
            (Register(_), Immediate(_)) => 4,
            (Memory(ea), Immediate(_)) => 17 + ea.num_cycles(),
            (Accumulator | AccumulatorWide, Immediate(_)) => 4,
            _ => panic!("Invalid ADD operation"),
        }
    }
}

impl Instruction for AddInstruction {
    fn execute(&self, _reader: &mut BufReader<File>, store: &mut Store) -> u32 {
        let AddInstruction(AnyInstruction {
            source,
            destination,
            ..
        }) = self;

        let (rhs, clock_penalty_rhs) = source
            .as_ref()
            .map(|source| source.to_immediate_value(self.0.is_wide, store))
            .expect("add operation expects both source and destination");

        let (lhs, clock_penalty_lhs) = destination.to_immediate_value(self.0.is_wide, store);

        let op_result = lhs + rhs;

        store.flag_register_store_mut().set_flags_on_op(op_result);

        destination.write_value(op_result.value(), self.0.is_wide, store);

        let clock_penalty =
            if clock_penalty_rhs { 4 } else { 0 } + if clock_penalty_lhs { 4 } else { 0 };

        self.num_cycles() + clock_penalty
    }
}
