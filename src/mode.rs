use crate::{bit_match, compute_mask, compute_val, Byte};

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
