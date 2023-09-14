use std::fmt::{Debug, Display, Formatter};

const REGISTERS_MEMORY_SIZE: usize = 16;

#[derive(Debug, Default)]
pub struct FlagRegisterManager {
    memory: [bool; REGISTERS_MEMORY_SIZE],
}

impl FlagRegisterManager {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Display for FlagRegisterManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.memory.fmt(f)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum FlagRegister {
    Carry,
    Parity,
    AuxiliaryCarry,
    Zero,
    Sign,
    Trap,
    Interrupt,
    Direction,
    Overflow,
}

use crate::memory::Memory;
use FlagRegister::*;

impl Display for FlagRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Carry => f.write_str("CF"),
            Parity => f.write_str("PF"),
            AuxiliaryCarry => f.write_str("AF"),
            Zero => f.write_str("ZF"),
            Sign => f.write_str("SF"),
            Trap => f.write_str("TF"),
            Interrupt => f.write_str("IF"),
            Direction => f.write_str("DF"),
            Overflow => f.write_str("OF"),
        }
    }
}

impl FlagRegister {
    pub fn to_memory_address(self) -> u16 {
        match self {
            Carry => 0,
            Parity => 2,
            AuxiliaryCarry => 4,
            Zero => 6,
            Sign => 7,
            Trap => 8,
            Interrupt => 9,
            Direction => 10,
            Overflow => 11,
        }
    }
}

impl Memory<bool, REGISTERS_MEMORY_SIZE> for FlagRegisterManager {
    fn get_memory_mut(&mut self) -> &mut [bool; REGISTERS_MEMORY_SIZE] {
        &mut self.memory
    }

    fn get_memory(&self) -> &[bool; REGISTERS_MEMORY_SIZE] {
        &self.memory
    }
}

impl FlagRegisterManager {
    pub fn get_flag(&self, flag: FlagRegister) -> bool {
        self.get_memory()[flag.to_memory_address() as usize]
    }

    pub fn set_flag(&mut self, flag: FlagRegister) {
        self.get_memory_mut()[flag.to_memory_address() as usize] = true;
    }

    pub fn unset_flag(&mut self, flag: FlagRegister) {
        self.get_memory_mut()[flag.to_memory_address() as usize] = false;
    }
}
