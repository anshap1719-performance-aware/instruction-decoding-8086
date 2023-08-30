use crate::prelude::*;
use crate::register::Register;
use crate::*;
use byteorder::LittleEndian;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum InstructionMode {
    Register = 0b11,
    Memory = 0b00,
    MemoryPlusByte = 0b01,
    MemoryPlusWord = 0b10,
}

impl From<Byte> for InstructionMode {
    fn from(value: Byte) -> Self {
        match value {
            value if bit_match!(value, (1, 1, _, _, _, _, _, _)) => Self::Register,
            value if bit_match!(value, (0, 0, _, _, _, _, _, _)) => Self::Memory,
            value if bit_match!(value, (0, 1, _, _, _, _, _, _)) => Self::MemoryPlusByte,
            value if bit_match!(value, (1, 0, _, _, _, _, _, _)) => Self::MemoryPlusWord,
            _ => panic!("Unable to decode move mode"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum EffectiveAddress {
    Register(Register),
    RegisterSum(Register, Register),
    RegisterPlusByte(Register, Byte),
    RegisterPlusWord(Register, Word),
    RegisterSumPlusByte(Register, Register, Byte),
    RegisterSumPlusWord(Register, Register, Word),
    DirectAddress(Word),
}

impl From<(&mut BufReader<File>, InstructionMode, Byte)> for EffectiveAddress {
    fn from((reader, mode, mem_byte): (&mut BufReader<File>, InstructionMode, Byte)) -> Self {
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
                let displacement = reader.read_u8().unwrap();

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
                let displacement = reader.read_u16::<LittleEndian>().unwrap();

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
            EffectiveAddress::Register(register) => register.fmt(f),
            EffectiveAddress::RegisterSum(register1, register2) => {
                f.write_fmt(format_args!("[{register1} + {register2}]"))
            }
            EffectiveAddress::RegisterPlusByte(register, byte) => {
                f.write_fmt(format_args!("[{register} + {byte}]"))
            }
            EffectiveAddress::RegisterPlusWord(register, word) => {
                f.write_fmt(format_args!("[{register} + {word}]"))
            }
            EffectiveAddress::RegisterSumPlusByte(register1, register2, byte) => {
                f.write_fmt(format_args!("[{register1} + {register2} + {byte}]"))
            }
            EffectiveAddress::RegisterSumPlusWord(register1, register2, word) => {
                f.write_fmt(format_args!("[{register1} + {register2} + {word}]"))
            }
            EffectiveAddress::DirectAddress(word) => f.write_fmt(format_args!("[{word}]")),
        }
    }
}
