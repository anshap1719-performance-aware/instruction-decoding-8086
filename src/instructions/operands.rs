use crate::memory::EffectiveAddress;
use crate::mode::InstructionMode;
use crate::register::Register;
use crate::segment_register::SegmentRegister;
use crate::{BoxDynError, Byte, SignedByte, SignedWord, Wide};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq)]
pub enum ImmediateValue {
    SignedByte(i8),
    SignedWord(i16),
}

impl From<i16> for ImmediateValue {
    fn from(value: i16) -> Self {
        Self::SignedWord(value)
    }
}

impl From<i8> for ImmediateValue {
    fn from(value: i8) -> Self {
        Self::SignedByte(value)
    }
}

impl From<ImmediateValue> for i16 {
    fn from(value: ImmediateValue) -> Self {
        match value {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        }
    }
}

impl TryFrom<ImmediateValue> for i8 {
    type Error = BoxDynError;

    fn try_from(value: ImmediateValue) -> Result<Self, Self::Error> {
        match value {
            ImmediateValue::SignedByte(value) => Ok(value),
            ImmediateValue::SignedWord(_) => Err("Cannot read word as byte".into()),
        }
    }
}

impl From<ImmediateValue> for u16 {
    fn from(value: ImmediateValue) -> Self {
        match value {
            ImmediateValue::SignedByte(value) => u16::from_le_bytes((value as i16).to_le_bytes()),
            ImmediateValue::SignedWord(value) => u16::from_le_bytes((value).to_le_bytes()),
        }
    }
}

impl TryFrom<ImmediateValue> for u8 {
    type Error = BoxDynError;

    fn try_from(value: ImmediateValue) -> Result<Self, Self::Error> {
        match value {
            ImmediateValue::SignedByte(value) => Ok(u8::from_le_bytes(value.to_le_bytes())),
            ImmediateValue::SignedWord(_) => Err("Cannot read word as byte".into()),
        }
    }
}

impl Add for ImmediateValue {
    type Output = ImmediateValue;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let rhs = match rhs {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        ImmediateValue::SignedWord(lhs + rhs)
    }
}

impl Sub for ImmediateValue {
    type Output = ImmediateValue;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        let rhs = match rhs {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        ImmediateValue::SignedWord(lhs - rhs)
    }
}

impl Add<SignedByte> for ImmediateValue {
    type Output = i16;

    fn add(self, rhs: SignedByte) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        lhs + (rhs as i16)
    }
}

impl Sub<SignedByte> for ImmediateValue {
    type Output = i16;

    fn sub(self, rhs: SignedByte) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        lhs - (rhs as i16)
    }
}

impl Add<SignedWord> for ImmediateValue {
    type Output = i16;

    fn add(self, rhs: SignedWord) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        lhs + rhs
    }
}

impl Sub<SignedWord> for ImmediateValue {
    type Output = i16;

    fn sub(self, rhs: SignedWord) -> Self::Output {
        let lhs = match self {
            ImmediateValue::SignedByte(value) => value as i16,
            ImmediateValue::SignedWord(value) => value,
        };

        lhs - rhs
    }
}

impl Display for ImmediateValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmediateValue::SignedByte(value) => Display::fmt(value, f),
            ImmediateValue::SignedWord(value) => Display::fmt(value, f),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Operand {
    Accumulator,
    AccumulatorWide,
    Register(Register),
    SegmentRegister(SegmentRegister),
    Memory(EffectiveAddress),
    Immediate(ImmediateValue),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Accumulator => f.write_str("al"),
            Operand::AccumulatorWide => f.write_str("ax"),
            Operand::Register(register) => register.fmt(f),
            Operand::Memory(memory) => memory.fmt(f),
            Operand::Immediate(immediate) => immediate.fmt(f),
            Operand::SegmentRegister(register) => register.fmt(f),
        }
    }
}

impl Operand {
    pub fn read(
        reader: &mut BufReader<File>,
        mode: InstructionMode,
        target_specifier_byte: Byte,
        is_wide: Wide,
    ) -> Self {
        let mem_bytes = 0b00_000_111 & target_specifier_byte;

        if let InstructionMode::Register = mode {
            let mut mem_bytes = mem_bytes << 1;

            if is_wide {
                mem_bytes += 1;
            }

            Operand::Register(Register::from(mem_bytes))
        } else {
            Operand::Memory(EffectiveAddress::read(reader, mode, mem_bytes))
        }
    }
}
