use crate::prelude::*;
use std::fmt::{Display, Formatter, Write};

#[derive(Copy, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum JumpInstructionTypes {
    JumpOnEqualOrZero,
    JumpOnLess,
    JumpOnLessOrEqual,
    JumpOnBelow,
    JumpOnBelowOrEqual,
    JumpOnParityEven,
    JumpOnOverflow,
    JumpOnSign,
    JumpOnNotEqualAndNotZero,
    JumpOnNotLess,
    JumpOnNotLessAndNotEqual,
    JumpOnNotBelow,
    JumpOnNotBelowAndNotEqual,
    JumpOnParityOdd,
    JumpOnNotOverflow,
    JumpOnNotSign,
    Loop,
    LoopWhileZeroOrEqual,
    LoopWhileNotZeroAndNotEqual,
    JumpOnCxZero,
}

impl Display for JumpInstructionTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpInstructionTypes::JumpOnEqualOrZero => f.write_str("je"),
            JumpInstructionTypes::JumpOnLess => f.write_str("jl"),
            JumpInstructionTypes::JumpOnLessOrEqual => f.write_str("jle"),
            JumpInstructionTypes::JumpOnBelow => f.write_str("jb"),
            JumpInstructionTypes::JumpOnBelowOrEqual => f.write_str("jbe"),
            JumpInstructionTypes::JumpOnParityEven => f.write_str("jp"),
            JumpInstructionTypes::JumpOnOverflow => f.write_str("jo"),
            JumpInstructionTypes::JumpOnSign => f.write_str("js"),
            JumpInstructionTypes::JumpOnNotEqualAndNotZero => f.write_str("jne"),
            JumpInstructionTypes::JumpOnNotLess => f.write_str("jnl"),
            JumpInstructionTypes::JumpOnNotLessAndNotEqual => f.write_str("jnle"),
            JumpInstructionTypes::JumpOnNotBelow => f.write_str("jnb"),
            JumpInstructionTypes::JumpOnNotBelowAndNotEqual => f.write_str("jnbe"),
            JumpInstructionTypes::JumpOnParityOdd => f.write_str("jnp"),
            JumpInstructionTypes::JumpOnNotOverflow => f.write_str("jno"),
            JumpInstructionTypes::JumpOnNotSign => f.write_str("jns"),
            JumpInstructionTypes::Loop => f.write_str("loop"),
            JumpInstructionTypes::LoopWhileZeroOrEqual => f.write_str("loope"),
            JumpInstructionTypes::LoopWhileNotZeroAndNotEqual => f.write_str("loopne"),
            JumpInstructionTypes::JumpOnCxZero => f.write_str("jcxz"),
        }
    }
}

pub struct OptionalJumpInstructionTypes(pub Option<JumpInstructionTypes>);

impl From<Byte> for OptionalJumpInstructionTypes {
    fn from(value: Byte) -> Self {
        use JumpInstructionTypes::*;

        OptionalJumpInstructionTypes(match value {
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 0, 0)) => Some(JumpOnEqualOrZero),
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 0, 0)) => Some(JumpOnLess),
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 1, 0)) => Some(JumpOnLessOrEqual),
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 1, 0)) => Some(JumpOnBelow),
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 1, 0)) => Some(JumpOnBelowOrEqual),
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 1, 0)) => Some(JumpOnParityEven),
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 0, 0)) => Some(JumpOnOverflow),
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 0, 0)) => Some(JumpOnSign),
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 0, 1)) => Some(JumpOnNotEqualAndNotZero),
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 0, 1)) => Some(JumpOnNotLess),
            value if bit_match!(value, (0, 1, 1, 1, 1, 1, 1, 1)) => Some(JumpOnNotLessAndNotEqual),
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 1, 1)) => Some(JumpOnNotBelow),
            value if bit_match!(value, (0, 1, 1, 1, 0, 1, 1, 1)) => Some(JumpOnNotBelowAndNotEqual),
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 1, 1)) => Some(JumpOnParityOdd),
            value if bit_match!(value, (0, 1, 1, 1, 0, 0, 0, 1)) => Some(JumpOnNotOverflow),
            value if bit_match!(value, (0, 1, 1, 1, 1, 0, 0, 1)) => Some(JumpOnNotSign),
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 1, 0)) => Some(Loop),
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 0, 1)) => Some(LoopWhileZeroOrEqual),
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 0, 0)) => {
                Some(LoopWhileNotZeroAndNotEqual)
            }
            value if bit_match!(value, (1, 1, 1, 0, 0, 0, 1, 1)) => Some(JumpOnCxZero),
            _ => None,
        })
    }
}

pub struct JumpInstruction {
    pub variant: JumpInstructionTypes,
    pub displacement: SignedByte,
}

impl Display for JumpInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.variant.fmt(f)?;

        let computed_displacement = 2 + self.displacement;
        let sign = if computed_displacement.is_negative() {
            "-"
        } else {
            "+"
        };

        f.write_fmt(format_args!(" ${sign}{}", computed_displacement.abs()))
    }
}
