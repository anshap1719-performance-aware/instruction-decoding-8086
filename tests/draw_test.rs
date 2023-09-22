use std::fs::File;
use std::io::{BufReader, Seek, Write};
use std::path::Path;

#[test]
fn listing_0054_draw_rectangle() {
    let store = &mut instruction_decoding_8086::Store::default();

    let path = Path::new(file!())
        .parent()
        .unwrap()
        .join(Path::new("./test_listings"))
        .join(Path::new("listing_0054_draw_rectangle"));

    let input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));

    let mut reader = BufReader::new(input);
    reader = instruction_decoding_8086::simulate(reader, store);

    File::create("listing_0054_draw_rectangle.dump")
        .unwrap()
        .write_all(&store.memory_store().dump())
        .unwrap();
}

#[test]
fn listing_0055_challenge_rectangle() {
    let store = &mut instruction_decoding_8086::Store::default();

    let path = Path::new(file!())
        .parent()
        .unwrap()
        .join(Path::new("./test_listings"))
        .join(Path::new("listing_0055_challenge_rectangle"));

    let input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));

    let mut reader = BufReader::new(input);
    reader = instruction_decoding_8086::simulate(reader, store);

    File::create("listing_0055_challenge_rectangle.dump")
        .unwrap()
        .write_all(&store.memory_store().dump())
        .unwrap();
}
