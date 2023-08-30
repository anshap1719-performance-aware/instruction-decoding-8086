#![deny(clippy::correctness)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![allow(deprecated)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::unused_async)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::doc_markdown)]
#![deny(clippy::too_many_lines)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::cargo_common_metadata)]

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
        let Ok(instruction_byte) = reader.read_u8() else {
                break;
            };

        let instruction = Instructions::from((&mut reader, instruction_byte));

        println!("{instruction}");
    }
}
