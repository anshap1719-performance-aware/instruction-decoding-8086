pub mod helpers;
mod instructions;
mod memory;
pub mod prelude;
mod register;

use crate::instructions::Instructions;
use byteorder::ReadBytesExt;
pub use prelude::*;
use std::env::args;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args = args().collect::<Vec<_>>();

    let input = File::open(args[1].clone()).expect("Failed to open file");
    let mut reader = BufReader::new(input);

    loop {
        let instruction_byte = match reader.read_u8() {
            Ok(byte) => byte,
            Err(_) => {
                break;
            }
        };

        let instruction = Instructions::from((&mut reader, instruction_byte));

        println!("{}", instruction);
    }
}
