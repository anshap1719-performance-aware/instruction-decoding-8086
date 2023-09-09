use crate::prelude::*;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

const REGISTERS_MEMORY_SIZE: usize = 16;

#[derive(Debug)]
pub struct RegisterManager {
    memory: [u8; REGISTERS_MEMORY_SIZE],
}

impl RegisterManager {
    pub fn new() -> Self {
        Self {
            memory: [0b0; REGISTERS_MEMORY_SIZE],
        }
    }
}

impl Display for RegisterManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.memory.fmt(f)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Register {
    Al,
    Ax,
    Cl,
    Cx,
    Dl,
    Dx,
    Bl,
    Bx,
    Ah,
    Sp,
    Ch,
    Bp,
    Dh,
    Si,
    Bh,
    Di,
}

use crate::instructions::operands::ImmediateValue;
use crate::memory::Memory;
use Register::*;

impl From<Byte> for Register {
    fn from(value: Byte) -> Self {
        match 0b0000_1111 & value {
            0b0000 => Al,
            0b0001 => Ax,
            0b0010 => Cl,
            0b0011 => Cx,
            0b0100 => Dl,
            0b0101 => Dx,
            0b0110 => Bl,
            0b0111 => Bx,
            0b1000 => Ah,
            0b1001 => Sp,
            0b1010 => Ch,
            0b1011 => Bp,
            0b1100 => Dh,
            0b1101 => Si,
            0b1110 => Bh,
            0b1111 => Di,
            _ => panic!("Invalid register bits"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Al => f.write_str("al"),
            Ax => f.write_str("ax"),
            Cl => f.write_str("cl"),
            Cx => f.write_str("cx"),
            Dl => f.write_str("dl"),
            Dx => f.write_str("dx"),
            Bl => f.write_str("bl"),
            Bx => f.write_str("bx"),
            Ah => f.write_str("ah"),
            Sp => f.write_str("sp"),
            Ch => f.write_str("ch"),
            Bp => f.write_str("bp"),
            Dh => f.write_str("dh"),
            Si => f.write_str("si"),
            Bh => f.write_str("bh"),
            Di => f.write_str("di"),
        }
    }
}

impl Register {
    pub fn to_memory_address(self) -> u16 {
        match self {
            Ax | Ah => 0,
            Al => 1,
            Bx | Bh => 2,
            Bl => 3,
            Cx | Ch => 4,
            Cl => 5,
            Dx | Dh => 6,
            Dl => 7,
            Sp => 8,
            Bp => 10,
            Si => 12,
            Di => 14,
        }
    }
}

impl Memory<REGISTERS_MEMORY_SIZE> for RegisterManager {
    fn get_memory_mut(&mut self) -> &mut [u8; REGISTERS_MEMORY_SIZE] {
        &mut self.memory
    }

    fn get_memory(&self) -> &[u8; REGISTERS_MEMORY_SIZE] {
        &self.memory
    }
}

impl RegisterManager {
    pub fn read_value(&self, register: Register) -> ImmediateValue {
        match register {
            Al => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Ax => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
            Cl => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Cx => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
            Dl => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Dx => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
            Bl => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Bx => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
            Ah => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Sp => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
            Ch => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Bp => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
            Dh => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Si => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
            Bh => ImmediateValue::SignedByte(i8::from_le_bytes([
                self.read_byte_from_register(register)
            ])),
            Di => ImmediateValue::SignedWord(i16::from_le_bytes(
                self.read_word_from_register(register).to_le_bytes(),
            )),
        }
    }

    pub fn read_byte_from_register(&self, register: Register) -> u8 {
        self.read_byte(register.to_memory_address())
    }

    pub fn read_word_from_register(&self, register: Register) -> u16 {
        self.read_word(register.to_memory_address())
    }

    pub fn write_byte_to_register(&mut self, register: Register, value: u8) {
        self.write_byte(register.to_memory_address(), value);
    }

    pub fn write_word_to_register(&mut self, register: Register, value: u16) {
        self.write_word(register.to_memory_address(), value);
    }

    pub fn register_memory_map(&self) -> Vec<(&str, i16)> {
        vec![
            ("ax", self.read_value(Ax).into()),
            ("bx", self.read_value(Bx).into()),
            ("cx", self.read_value(Cx).into()),
            ("dx", self.read_value(Dx).into()),
            ("sp", self.read_value(Sp).into()),
            ("bp", self.read_value(Bp).into()),
            ("si", self.read_value(Si).into()),
            ("di", self.read_value(Di).into()),
        ]
    }
}
