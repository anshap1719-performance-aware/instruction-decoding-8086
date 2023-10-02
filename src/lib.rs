pub mod cycle;
pub mod flag_register;
pub mod helpers;
mod instructions;
pub mod memory;
mod mode;
pub mod prelude;
pub mod register;
pub mod segment_register;
pub mod store;

use crate::flag_register::FlagRegisterManager;
pub use crate::instructions::decode::Instructions;
pub use crate::instructions::Instruction;
use crate::memory::MemoryManager;
use crate::register::RegisterManager;
use crate::segment_register::SegmentRegisterManager;
pub use crate::store::Store;
use byteorder::ReadBytesExt;
pub use prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

pub fn simulate(mut reader: BufReader<File>, store: &mut Store) -> (BufReader<File>, u32) {
    let mut num_cpu_cycles = 0;

    loop {
        let Ok(instruction_byte) = reader.read_u8() else {
            break;
        };

        let instruction = Instructions::read(&mut reader, instruction_byte);

        num_cpu_cycles += instruction.execute(&mut reader, store);
    }

    (reader, num_cpu_cycles)
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
