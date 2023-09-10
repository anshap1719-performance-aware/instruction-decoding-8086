use crate::instructions::operands::{ImmediateValue, Operand};
use crate::instructions::{AnyInstruction, Instruction};
use crate::prelude::*;
use crate::{MemoryManager, RegisterManager, SegmentRegisterManager};
use byteorder::ReadBytesExt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

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

impl Instruction for JumpInstructions {
    fn execute(
        &self,
        register_store: &mut RegisterManager,
        memory_store: &mut MemoryManager,
        segment_register_store: &mut SegmentRegisterManager,
    ) {
        todo!()
    }
}
