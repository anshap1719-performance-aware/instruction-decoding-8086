pub mod helpers;
mod instructions;
pub mod memory;
mod mode;
pub mod prelude;
pub mod register;
pub mod segment_register;

pub use crate::instructions::decode::Instructions;
pub use crate::instructions::Instruction;
pub use crate::memory::MemoryManager;
pub use crate::register::RegisterManager;
pub use crate::segment_register::SegmentRegisterManager;
use byteorder::ReadBytesExt;
pub use prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

pub fn simulate(
    mut reader: BufReader<File>,
    register_store: &mut RegisterManager,
    memory_store: &mut MemoryManager,
    segment_register_store: &mut SegmentRegisterManager,
) {
    loop {
        let Ok(instruction_byte) = reader.read_u8() else {
            break;
        };

        let instruction = Instructions::read(&mut reader, instruction_byte);

        instruction.execute(register_store, memory_store, segment_register_store);
    }
}

pub fn decode(mut reader: BufReader<File>) -> String {
    let mut output = BufWriter::new(Vec::new());

    output.write_all("bits 16\n\n".as_bytes()).unwrap();

    loop {
        let Ok(instruction_byte) = reader.read_u8() else {
            break;
        };

        let instruction = Instructions::read(&mut reader, instruction_byte);

        output
            .write_all(format!("{instruction}\n").as_bytes())
            .unwrap();
    }

    output.flush().unwrap();

    let bytes = output.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}
