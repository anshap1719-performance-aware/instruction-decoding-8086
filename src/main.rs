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
use instruction_decoding_8086::store::Store;
use instruction_decoding_8086::*;
use std::env::args;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let store = &mut Store::new();

    let args = args().collect::<Vec<_>>();

    let input = File::open(args[1].clone()).expect("Failed to open file");
    let mut reader = BufReader::new(input);

    simulate(reader, store);
}
