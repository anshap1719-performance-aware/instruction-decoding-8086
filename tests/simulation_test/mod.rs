mod listing_43_test;
mod listing_44_test;
mod listing_45_test;

#[macro_export]
macro_rules! test_simulate_listing {
    ($listing_name:ident) => {
        use std::fs::File;
        use std::io::BufReader;
        use std::path::Path;

        #[test]
        fn $listing_name() {
            let register_store = &mut instruction_decoding_8086::RegisterManager::new();
            let memory_store = &mut instruction_decoding_8086::MemoryManager::new();

            let path = Path::new(file!())
                .parent()
                .unwrap()
                .join(Path::new("../test_listings"))
                .join(Path::new(stringify!($listing_name)));
            let input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));

            let reader = BufReader::new(input);

            instruction_decoding_8086::simulate(reader, register_store, memory_store);

            insta::assert_debug_snapshot!(register_store.register_memory_map());
        }
    };
}
