use crate::cycle::EstimatedCycleCount;
use crate::flag_register::FlagRegister;
use crate::instructions::operands::{ImmediateValue, Operand};
use crate::instructions::{AnyInstruction, Instruction};
use crate::prelude::*;
use crate::register::Register::Cx;
use crate::store::Store;
use byteorder::ReadBytesExt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Seek};

#[derive(Copy, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum JumpInstructions {
    JumpOnEqualOrZero(AnyInstruction),
    JumpOnLess(AnyInstruction),
    JumpOnLessOrEqual(AnyInstruction),
    JumpOnBelow(AnyInstruction),
    JumpOnBelowOrEqual(AnyInstruction),
    JumpOnParityEven(AnyInstruction),
    JumpOnOverflow(AnyInstruction),
    JumpOnSign(AnyInstruction),
    JumpOnNotEqualAndNotZero(AnyInstruction),
    JumpOnNotLess(AnyInstruction),
    JumpOnNotLessAndNotEqual(AnyInstruction),
    JumpOnNotBelow(AnyInstruction),
    JumpOnNotBelowAndNotEqual(AnyInstruction),
    JumpOnParityOdd(AnyInstruction),
    JumpOnNotOverflow(AnyInstruction),
    JumpOnNotSign(AnyInstruction),
    Loop(AnyInstruction),
    LoopWhileZeroOrEqual(AnyInstruction),
    LoopWhileNotZeroAndNotEqual(AnyInstruction),
    JumpOnCxZero(AnyInstruction),
}

impl Display for JumpInstructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn write_jump_instruction(
            opcode: &str,
            displacement: i8,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            f.write_str(opcode)?;

            let computed_displacement = 2 + displacement;
            let sign = if computed_displacement.is_negative() {
                "-"
            } else {
                "+"
            };

            f.write_fmt(format_args!(" ${sign}{}", computed_displacement.abs()))
        }

        match self {
            JumpInstructions::JumpOnEqualOrZero(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("je", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnLess(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jl", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnLessOrEqual(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jle", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnBelow(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jb", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnBelowOrEqual(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jbe", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnParityEven(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jp", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnOverflow(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jo", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnSign(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("js", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnNotEqualAndNotZero(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jne", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnNotLess(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jnl", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnNotLessAndNotEqual(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jnle", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnNotBelow(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jnb", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnNotBelowAndNotEqual(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jnbe", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnParityOdd(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jnp", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnNotOverflow(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jno", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnNotSign(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jns", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::Loop(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("loop", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::LoopWhileZeroOrEqual(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("loope", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::LoopWhileNotZeroAndNotEqual(AnyInstruction {
                destination, ..
            }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("loopne", *displacement, f)
                } else {
                    Ok(())
                }
            }
            JumpInstructions::JumpOnCxZero(AnyInstruction { destination, .. }) => {
                if let Operand::Immediate(ImmediateValue::SignedByte(displacement)) = destination {
                    write_jump_instruction("jcxz", *displacement, f)
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl JumpInstructions {
    pub fn is_jump_instruction(value: Byte) -> bool {
        bit_match!(value, (0, 1, 1, 1, 0, 1, 0, 0))
            || bit_match!(value, (0, 1, 1, 1, 1, 1, 0, 0))
            || bit_match!(value, (0, 1, 1, 1, 1, 1, 1, 0))
            || bit_match!(value, (0, 1, 1, 1, 0, 0, 1, 0))
            || bit_match!(value, (0, 1, 1, 1, 0, 1, 1, 0))
            || bit_match!(value, (0, 1, 1, 1, 1, 0, 1, 0))
            || bit_match!(value, (0, 1, 1, 1, 0, 0, 0, 0))
            || bit_match!(value, (0, 1, 1, 1, 1, 0, 0, 0))
            || bit_match!(value, (0, 1, 1, 1, 0, 1, 0, 1))
            || bit_match!(value, (0, 1, 1, 1, 1, 1, 0, 1))
            || bit_match!(value, (0, 1, 1, 1, 1, 1, 1, 1))
            || bit_match!(value, (0, 1, 1, 1, 0, 0, 1, 1))
            || bit_match!(value, (0, 1, 1, 1, 0, 1, 1, 1))
            || bit_match!(value, (0, 1, 1, 1, 1, 0, 1, 1))
            || bit_match!(value, (0, 1, 1, 1, 0, 0, 0, 1))
            || bit_match!(value, (0, 1, 1, 1, 1, 0, 0, 1))
            || bit_match!(value, (1, 1, 1, 0, 0, 0, 1, 0))
            || bit_match!(value, (1, 1, 1, 0, 0, 0, 0, 1))
            || bit_match!(value, (1, 1, 1, 0, 0, 0, 0, 0))
            || bit_match!(value, (1, 1, 1, 0, 0, 0, 1, 1))
    }
}

impl TryFrom<(Byte, &mut BufReader<File>)> for JumpInstructions {
    type Error = BoxDynError;

    fn try_from((value, reader): (Byte, &mut BufReader<File>)) -> Result<Self, Self::Error> {
        use JumpInstructions::*;

        match value {
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 0, 0)) => {
                Ok(JumpOnEqualOrZero(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 0, 0)) => {
                Ok(JumpOnLess(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 1, 0)) => {
                Ok(JumpOnLessOrEqual(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 1, 0)) => {
                Ok(JumpOnBelow(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 1, 0)) => {
                Ok(JumpOnBelowOrEqual(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 1, 0)) => {
                Ok(JumpOnParityEven(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 0, 0)) => {
                Ok(JumpOnOverflow(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 0, 0)) => {
                Ok(JumpOnSign(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 0, 1)) => {
                Ok(JumpOnNotEqualAndNotZero(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 0, 1)) => {
                Ok(JumpOnNotLess(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 1, 1)) => {
                Ok(JumpOnNotLessAndNotEqual(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 1, 1)) => {
                Ok(JumpOnNotBelow(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 1, 1)) => {
                Ok(JumpOnNotBelowAndNotEqual(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 1, 1)) => {
                Ok(JumpOnParityOdd(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 0, 1)) => {
                Ok(JumpOnNotOverflow(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 0, 1)) => {
                Ok(JumpOnNotSign(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 1, 0)) => Ok(Loop(AnyInstruction {
                destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                ..Default::default()
            })),
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 0, 1)) => {
                Ok(LoopWhileZeroOrEqual(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 0, 0)) => {
                Ok(LoopWhileNotZeroAndNotEqual(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 1, 1)) => {
                Ok(JumpOnCxZero(AnyInstruction {
                    destination: Operand::Immediate(ImmediateValue::SignedByte(reader.read_i8()?)),
                    ..Default::default()
                }))
            }
            _ => Err("Not a jump instruction".into()),
        }
    }
}

impl EstimatedCycleCount for JumpInstructions {
    fn num_cycles(&self) -> u32 {
        match self {
            JumpInstructions::JumpOnEqualOrZero(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnLess(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnLessOrEqual(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnBelow(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnBelowOrEqual(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnParityEven(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnOverflow(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnSign(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnNotEqualAndNotZero(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnNotLess(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnNotLessAndNotEqual(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnNotBelow(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnNotBelowAndNotEqual(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnParityOdd(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnNotOverflow(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnNotSign(AnyInstruction { .. }) => 16,
            JumpInstructions::Loop(AnyInstruction { .. }) => 16,
            JumpInstructions::LoopWhileZeroOrEqual(AnyInstruction { .. }) => 16,
            JumpInstructions::LoopWhileNotZeroAndNotEqual(AnyInstruction { .. }) => 16,
            JumpInstructions::JumpOnCxZero(AnyInstruction { .. }) => 16,
        }
    }
}

impl Instruction for JumpInstructions {
    fn execute(&self, reader: &mut BufReader<File>, store: &mut Store) -> u32 {
        use FlagRegister::*;

        let (should_jump, (displacement, clock_penalty)) = match self {
            JumpInstructions::JumpOnEqualOrZero(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (store.flag_register_store().get_flag(Zero), displacement)
            }
            JumpInstructions::JumpOnLess(AnyInstruction { destination, .. }) => {
                let should_jump = store.flag_register_store().get_flag(Sign)
                    ^ store.flag_register_store().get_flag(Overflow);

                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::JumpOnLessOrEqual(AnyInstruction { destination, .. }) => {
                let should_jump = (store.flag_register_store().get_flag(Sign)
                    ^ store.flag_register_store().get_flag(Overflow))
                    || store.flag_register_store().get_flag(Zero);

                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::JumpOnBelow(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (store.flag_register_store().get_flag(Carry), displacement)
            }
            JumpInstructions::JumpOnBelowOrEqual(AnyInstruction { destination, .. }) => {
                let should_jump = store.flag_register_store().get_flag(Carry)
                    || store.flag_register_store().get_flag(Zero);

                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::JumpOnParityEven(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (store.flag_register_store().get_flag(Parity), displacement)
            }
            JumpInstructions::JumpOnOverflow(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (store.flag_register_store().get_flag(Overflow), displacement)
            }
            JumpInstructions::JumpOnSign(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (store.flag_register_store().get_flag(Sign), displacement)
            }
            JumpInstructions::JumpOnNotEqualAndNotZero(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (!store.flag_register_store().get_flag(Zero), displacement)
            }
            JumpInstructions::JumpOnNotLess(AnyInstruction { destination, .. }) => {
                let should_jump = !(store.flag_register_store().get_flag(Sign)
                    ^ store.flag_register_store().get_flag(Overflow));
                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::JumpOnNotLessAndNotEqual(AnyInstruction { destination, .. }) => {
                let should_jump = !((store.flag_register_store().get_flag(Sign)
                    ^ store.flag_register_store().get_flag(Overflow))
                    || store.flag_register_store().get_flag(Zero));

                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::JumpOnNotBelow(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (!store.flag_register_store().get_flag(Carry), displacement)
            }
            JumpInstructions::JumpOnNotBelowAndNotEqual(AnyInstruction { destination, .. }) => {
                let should_jump = !store.flag_register_store().get_flag(Carry)
                    && !store.flag_register_store().get_flag(Zero);

                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::JumpOnParityOdd(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (!store.flag_register_store().get_flag(Parity), displacement)
            }
            JumpInstructions::JumpOnNotOverflow(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (
                    !store.flag_register_store().get_flag(Overflow),
                    displacement,
                )
            }
            JumpInstructions::JumpOnNotSign(AnyInstruction { destination, .. }) => {
                let displacement = destination.to_immediate_value(true, store);

                (!store.flag_register_store().get_flag(Sign), displacement)
            }
            JumpInstructions::Loop(AnyInstruction { destination, .. }) => {
                let cx_value: i16 = store.register_store().read_value(Cx).into();
                let cx_value = cx_value - 1;

                store
                    .register_store_mut()
                    .write_word_to_register(Cx, ImmediateValue::SignedWord(cx_value).into());

                let should_jump = cx_value != 0;
                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::LoopWhileZeroOrEqual(AnyInstruction { destination, .. }) => {
                let cx_value: i16 = store.register_store().read_value(Cx).into();
                let cx_value = cx_value - 1;

                store
                    .register_store_mut()
                    .write_word_to_register(Cx, ImmediateValue::SignedWord(cx_value).into());

                let should_jump = cx_value != 0 && store.flag_register_store().get_flag(Zero);
                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::LoopWhileNotZeroAndNotEqual(AnyInstruction {
                destination, ..
            }) => {
                let cx_value: i16 = store.register_store().read_value(Cx).into();
                let cx_value = cx_value - 1;

                store
                    .register_store_mut()
                    .write_word_to_register(Cx, ImmediateValue::SignedWord(cx_value).into());

                let should_jump = cx_value != 0 && !store.flag_register_store().get_flag(Zero);
                let displacement = destination.to_immediate_value(true, store);

                (should_jump, displacement)
            }
            JumpInstructions::JumpOnCxZero(AnyInstruction { destination, .. }) => {
                let cx_value: i16 = store.register_store().read_value(Cx).into();
                let displacement = destination.to_immediate_value(true, store);

                (cx_value == 0, displacement)
            }
        };

        if should_jump {
            let displacement: i16 = displacement.into();

            reader
                .seek_relative(displacement as i64)
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to jump {displacement} from {:?}",
                        reader.stream_position()
                    )
                });
        }

        self.num_cycles() + if clock_penalty { 4 } else { 0 }
    }
}
