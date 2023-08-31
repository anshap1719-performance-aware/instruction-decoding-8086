#![deny(clippy::correctness)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![allow(deprecated)]
#![allow(unused)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::cargo_common_metadata)]

pub mod helpers;
mod instructions;
mod memory;
mod mode;
pub mod prelude;
mod register;

use crate::instructions::decode::Instructions;
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
        let Ok(instruction_byte) = reader.read_u8() else {
                break;
            };

        let instruction = Instructions::read(&mut reader, instruction_byte);

        println!("{instruction}");
    }
}
