use crate::mode::InstructionMode;
use crate::prelude::*;
use crate::register::Register;
use crate::*;
use byteorder::LittleEndian;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub enum EffectiveAddress {
    Register(Register),
    RegisterSum(Register, Register),
    RegisterPlusByte(Register, SignedByte),
    RegisterPlusWord(Register, SignedWord),
    RegisterSumPlusByte(Register, Register, SignedByte),
    RegisterSumPlusWord(Register, Register, SignedWord),
    DirectAddress(Word),
}

impl EffectiveAddress {
    pub fn read(reader: &mut BufReader<File>, mode: InstructionMode, mem_byte: Byte) -> Self {
        use register::Register::*;
        use EffectiveAddress::*;

        match mode {
            InstructionMode::Memory => match mem_byte {
                0b000 => RegisterSum(Bx, Si),
                0b001 => RegisterSum(Bx, Di),
                0b010 => RegisterSum(Bp, Si),
                0b011 => RegisterSum(Bp, Di),
                0b100 => Register(Si),
                0b101 => Register(Di),
                0b110 => {
                    let displacement = reader.read_u16::<LittleEndian>().unwrap();

                    DirectAddress(displacement)
                }
                0b111 => Register(Bx),
                _ => panic!("Unable to compute effective address"),
            },
            InstructionMode::MemoryPlusByte => {
                let displacement = reader.read_i8().unwrap();

                match mem_byte {
                    0b000 => RegisterSumPlusByte(Bx, Si, displacement),
                    0b001 => RegisterSumPlusByte(Bx, Di, displacement),
                    0b010 => RegisterSumPlusByte(Bp, Si, displacement),
                    0b011 => RegisterSumPlusByte(Bp, Di, displacement),
                    0b100 => RegisterPlusByte(Si, displacement),
                    0b101 => RegisterPlusByte(Di, displacement),
                    0b110 => RegisterPlusByte(Bp, displacement),
                    0b111 => RegisterPlusByte(Bx, displacement),
                    _ => panic!("Unable to compute effective address"),
                }
            }
            InstructionMode::MemoryPlusWord => {
                let displacement = reader.read_i16::<LittleEndian>().unwrap();

                match mem_byte {
                    0b000 => RegisterSumPlusWord(Bx, Si, displacement),
                    0b001 => RegisterSumPlusWord(Bx, Di, displacement),
                    0b010 => RegisterSumPlusWord(Bp, Si, displacement),
                    0b011 => RegisterSumPlusWord(Bp, Di, displacement),
                    0b100 => RegisterPlusWord(Si, displacement),
                    0b101 => RegisterPlusWord(Di, displacement),
                    0b110 => RegisterPlusWord(Bp, displacement),
                    0b111 => RegisterPlusWord(Bx, displacement),
                    _ => panic!("Unable to compute effective address"),
                }
            }
            InstructionMode::Register => {
                panic!("Unable to compute effective address for register-register mode")
            }
        }
    }
}

impl Display for EffectiveAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectiveAddress::Register(register) => f.write_fmt(format_args!("[{register}]")),
            EffectiveAddress::RegisterSum(register1, register2) => {
                f.write_fmt(format_args!("[{register1} + {register2}]"))
            }
            EffectiveAddress::RegisterPlusByte(register, byte) => {
                let sign = if byte.is_negative() { "-" } else { "+" };
                f.write_fmt(format_args!("[{register} {sign} {}]", byte.abs()))
            }
            EffectiveAddress::RegisterPlusWord(register, word) => {
                let sign = if word.is_negative() { "-" } else { "+" };
                f.write_fmt(format_args!("[{register} {sign} {}]", word.abs()))
            }
            EffectiveAddress::RegisterSumPlusByte(register1, register2, byte) => {
                let sign = if byte.is_negative() { "-" } else { "+" };
                f.write_fmt(format_args!(
                    "[{register1} + {register2} {sign} {}]",
                    byte.abs()
                ))
            }
            EffectiveAddress::RegisterSumPlusWord(register1, register2, word) => {
                let sign = if word.is_negative() { "-" } else { "+" };
                f.write_fmt(format_args!(
                    "[{register1} + {register2} {sign} {}]",
                    word.abs()
                ))
            }
            EffectiveAddress::DirectAddress(word) => f.write_fmt(format_args!("[{word}]")),
        }
    }
}

pub trait Memory<const MEMORY_SIZE: u16> {
    fn verify_address(&self, address: u16) {
        assert!(address <= MEMORY_SIZE);
    }

    fn read_byte(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
    fn write_byte(&mut self, address: u16, value: u8);
    fn write_word(&mut self, address: u16, value: u16);
}
