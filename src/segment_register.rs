use crate::prelude::*;
use std::fmt::{Debug, Display, Formatter};

const SEGMENT_REGISTERS_MEMORY_SIZE: usize = 16;

#[derive(Debug)]
pub struct SegmentRegisterManager {
    memory: [u8; SEGMENT_REGISTERS_MEMORY_SIZE],
}

impl SegmentRegisterManager {
    pub fn new() -> Self {
        Self {
            memory: [0b0; SEGMENT_REGISTERS_MEMORY_SIZE],
        }
    }
}

impl Display for SegmentRegisterManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.memory.fmt(f)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SegmentRegister {
    Es,
    Cs,
    Ss,
    Ds,
}

use crate::memory::Memory;
use SegmentRegister::*;

impl From<Byte> for SegmentRegister {
    fn from(value: Byte) -> Self {
        match 0b0000_0011 & value {
            0b00 => Es,
            0b01 => Cs,
            0b10 => Ss,
            0b11 => Ds,
            _ => panic!("Invalid segment register bits"),
        }
    }
}

impl Display for SegmentRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Es => f.write_str("es"),
            Cs => f.write_str("cs"),
            Ss => f.write_str("ss"),
            Ds => f.write_str("ds"),
        }
    }
}

impl SegmentRegister {
    pub fn to_memory_address(self) -> u16 {
        match self {
            Es => 0,
            Cs => 2,
            Ss => 4,
            Ds => 6,
        }
    }
}

impl Memory<SEGMENT_REGISTERS_MEMORY_SIZE> for SegmentRegisterManager {
    fn get_memory_mut(&mut self) -> &mut [u8; SEGMENT_REGISTERS_MEMORY_SIZE] {
        &mut self.memory
    }

    fn get_memory(&self) -> &[u8; SEGMENT_REGISTERS_MEMORY_SIZE] {
        &self.memory
    }
}

impl SegmentRegisterManager {
    pub fn read_value(&self, segment_register: SegmentRegister) -> u16 {
        self.read_word_from_segment_register(segment_register)
    }

    pub fn read_byte_from_segment_register(&self, segment_register: SegmentRegister) -> u8 {
        self.read_byte(segment_register.to_memory_address())
    }

    pub fn read_word_from_segment_register(&self, segment_register: SegmentRegister) -> u16 {
        self.read_word(segment_register.to_memory_address())
    }

    pub fn write_byte_to_segment_register(&mut self, segment_register: SegmentRegister, value: u8) {
        self.write_byte(segment_register.to_memory_address(), value);
    }

    pub fn write_word_to_segment_register(
        &mut self,
        segment_register: SegmentRegister,
        value: u16,
    ) {
        self.write_word(segment_register.to_memory_address(), value);
    }

    pub fn segment_register_memory_map(&self) -> Vec<(&str, u16)> {
        vec![
            ("es", self.read_value(Es)),
            ("cs", self.read_value(Cs)),
            ("ss", self.read_value(Ss)),
            ("ds", self.read_value(Ds)),
        ]
    }
}
