use crate::memory::EffectiveAddress;
use crate::mode::InstructionMode;
use crate::register::Register;
use crate::{Byte, Wide};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone, PartialEq)]
pub enum ImmediateValue {
    SignedByte(i8),
    SignedWord(i16),
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
