use crate::prelude::*;
use std::fmt::{Debug, Display, Formatter};

const REGISTERS_MEMORY_SIZE: u16 = 16;

#[derive(Debug)]
pub struct RegisterManager {
    memory: [u8; REGISTERS_MEMORY_SIZE as usize],
}

impl RegisterManager {
    pub fn new() -> Self {
        Self {
            memory: [
                0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0, 0b0,
            ],
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
    fn read_byte(&self, address: u16) -> u8 {
        self.verify_address(address);

        self.memory[address as usize]
    }

    fn read_word(&self, address: u16) -> u16 {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let high = self.memory[address as usize];
        let low = self.memory[low_byte_address as usize];

        (u16::from(high) << 8) + u16::from(low)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.verify_address(address);

        self.memory[address as usize] = value;
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let [high, low] = value.to_be_bytes();

        self.memory[address as usize] = high;
        self.memory[low_byte_address as usize] = low;
    }
}

impl RegisterManager {
    fn read_byte_from_register(&self, register: Register) -> u8 {
        self.read_byte(register.to_memory_address())
    }

    fn read_word_from_register(&self, register: Register) -> u16 {
        self.read_word(register.to_memory_address())
    }

    fn write_byte_to_register(&mut self, register: Register, value: u8) {
        self.write_byte(register.to_memory_address(), value);
    }

    fn write_word_to_register(&mut self, register: Register, value: u16) {
        self.write_word(register.to_memory_address(), value);
    }
}
