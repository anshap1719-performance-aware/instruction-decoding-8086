use instruction_decoding_8086::memory::Memory;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn test_listing_43() {
    let register_store = &mut instruction_decoding_8086::RegisterManager::new();
    let memory_store = &mut instruction_decoding_8086::MemoryManager::new();

    let path = Path::new(file!())
        .parent()
        .unwrap()
        .join(Path::new("../test_listings/listing_43"));
    let input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));

    let reader = BufReader::new(input);

    instruction_decoding_8086::simulate(reader, register_store, memory_store);

    assert_eq!(
        *register_store.get_memory(),
        [0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8]
    )
}
