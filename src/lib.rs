pub mod helpers;
mod instructions;
pub mod memory;
mod mode;
pub mod prelude;
pub mod register;

pub use crate::instructions::decode::Instructions;
pub use crate::instructions::Instruction;
pub use crate::memory::MemoryManager;
pub use crate::register::RegisterManager;
use byteorder::ReadBytesExt;
pub use prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn simulate(
    mut reader: BufReader<File>,
    register_store: &mut RegisterManager,
    memory_store: &mut MemoryManager,
) {
    loop {
        let Ok(instruction_byte) = reader.read_u8() else {
            break;
        };

        let instruction = Instructions::read(&mut reader, instruction_byte);

        instruction.execute(register_store, memory_store);
    }
}
