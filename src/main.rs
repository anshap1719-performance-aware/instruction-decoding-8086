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

use byteorder::ReadBytesExt;
use instruction_decoding_8086::*;
use std::env::args;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let register_store = &mut RegisterManager::new();
    let memory_store = &mut MemoryManager::new();

    let args = args().collect::<Vec<_>>();

    let input = File::open(args[1].clone()).expect("Failed to open file");
    let mut reader = BufReader::new(input);

    simulate(reader, register_store, memory_store);
}

fn decode() {
    let args = args().collect::<Vec<_>>();

    let input = File::open(args[1].clone()).expect("Failed to open file");
    let mut reader = BufReader::new(input);
    let mut output = String::from("bits 16\n\n");

    loop {
        let Ok(instruction_byte) = reader.read_u8() else {
            break;
        };

        let instruction = Instructions::read(&mut reader, instruction_byte);

        output += &format!("{instruction}\n");
    }

    println!("{output}");
}
