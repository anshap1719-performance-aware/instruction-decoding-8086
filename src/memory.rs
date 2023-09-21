use crate::instructions::operands::ImmediateValue;
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

pub trait Memory<T: Sized, const MEMORY_SIZE: usize> {
    fn verify_address(&self, address: u16) {
        assert!(address <= MEMORY_SIZE as u16);
    }

    fn get_memory_mut(&mut self) -> &mut [T; MEMORY_SIZE];

    fn get_memory(&self) -> &[T; MEMORY_SIZE];
}

pub trait ByteMemory<const MEMORY_SIZE: usize>
where
    Self: Memory<u8, MEMORY_SIZE>,
{
    fn read_byte(&self, address: u16) -> u8 {
        self.verify_address(address);

        self.get_memory()[address as usize]
    }

    fn read_signed_byte(&self, address: u16) -> i8 {
        self.verify_address(address);

        self.get_memory()[address as usize] as i8
    }

    fn read_word(&self, address: u16) -> u16 {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let memory = self.get_memory();

        let high = memory[address as usize];
        let low = memory[low_byte_address as usize];

        (u16::from(high) << 8) + u16::from(low)
    }

    fn read_signed_word(&self, address: u16) -> i16 {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let memory = self.get_memory();

        let high = memory[address as usize];
        let low = memory[low_byte_address as usize];

        (i16::from(high) << 8) + i16::from(low)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.verify_address(address);

        self.get_memory_mut()[address as usize] = value;
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let [high, low] = value.to_be_bytes();

        let memory = self.get_memory_mut();

        memory[address as usize] = high;
        memory[low_byte_address as usize] = low;
    }
}

impl<T, const MEMORY_SIZE: usize> ByteMemory<MEMORY_SIZE> for T
where
    T: Memory<u8, MEMORY_SIZE>,
{
    fn read_byte(&self, address: u16) -> u8 {
        self.verify_address(address);

        self.get_memory()[address as usize]
    }

    fn read_signed_byte(&self, address: u16) -> i8 {
        self.verify_address(address);

        self.get_memory()[address as usize] as i8
    }

    fn read_word(&self, address: u16) -> u16 {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let memory = self.get_memory();

        let high = memory[address as usize];
        let low = memory[low_byte_address as usize];

        (u16::from(high) << 8) + u16::from(low)
    }

    fn read_signed_word(&self, address: u16) -> i16 {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let memory = self.get_memory();

        let high = memory[address as usize];
        let low = memory[low_byte_address as usize];

        (i16::from(high) << 8) + i16::from(low)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.verify_address(address);

        self.get_memory_mut()[address as usize] = value;
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.verify_address(address);

        let low_byte_address = address + 1;

        self.verify_address(low_byte_address);

        let [high, low] = value.to_be_bytes();

        let memory = self.get_memory_mut();

        memory[address as usize] = high;
        memory[low_byte_address as usize] = low;
    }
}

const MAIN_MEMORY_SIZE: usize = u16::MAX as usize;

#[derive(Debug)]
pub struct MemoryManager {
    memory: [u8; MAIN_MEMORY_SIZE],
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            memory: [0b0; MAIN_MEMORY_SIZE],
        }
    }
}

impl MemoryManager {
    pub fn effective_address_to_address(
        &self,
        address: EffectiveAddress,
        register_manager: &RegisterManager,
    ) -> u16 {
        match address {
            EffectiveAddress::Register(register) => {
                let address: i16 = register_manager.read_value(register).into();
                address as u16
            }
            EffectiveAddress::RegisterSum(register1, register2) => {
                let address: i16 = (register_manager.read_value(register1)
                    + register_manager.read_value(register2))
                .into();

                address as u16
            }
            EffectiveAddress::RegisterPlusByte(register, value) => {
                let address = register_manager.read_value(register) + value;

                address.value() as u16
            }
            EffectiveAddress::RegisterPlusWord(register, value) => {
                let address = register_manager.read_value(register) + value;

                address.value() as u16
            }
            EffectiveAddress::RegisterSumPlusByte(register1, register2, value) => {
                let address = (register_manager.read_value(register1)
                    + register_manager.read_value(register2))
                .value()
                    + (value as i16);

                address.value() as u16
            }
            EffectiveAddress::RegisterSumPlusWord(register1, register2, value) => {
                let address = (register_manager.read_value(register1)
                    + register_manager.read_value(register2))
                .value()
                    + value;

                address.value() as u16
            }
            EffectiveAddress::DirectAddress(address) => address,
        }
    }

    pub fn read_memory_from_effective_address(
        &self,
        address: EffectiveAddress,
        is_wide: bool,
        register_manager: &RegisterManager,
    ) -> ImmediateValue {
        let address = self.effective_address_to_address(address, register_manager);

        if is_wide {
            self.read_signed_word(address).into()
        } else {
            self.read_signed_byte(address).into()
        }
    }

    pub fn write_to_effective_memory_address(
        &mut self,
        address: EffectiveAddress,
        is_wide: bool,
        register_manager: &RegisterManager,
        value: ImmediateValue,
    ) {
        let address = self.effective_address_to_address(address, register_manager);

        if is_wide {
            self.write_word(address, value.into())
        } else {
            self.write_byte(
                address,
                value.try_into().expect("Not is wide but value is word"),
            )
        }
    }
}

impl Memory<u8, MAIN_MEMORY_SIZE> for MemoryManager {
    fn get_memory_mut(&mut self) -> &mut [u8; MAIN_MEMORY_SIZE] {
        &mut self.memory
    }

    fn get_memory(&self) -> &[u8; MAIN_MEMORY_SIZE] {
        &self.memory
    }
}
