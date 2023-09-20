use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[macro_export]
macro_rules! test_simulate_listing {
    ($listing_name:ident) => {
        #[test]
        fn $listing_name() {
            let register_store = &mut instruction_decoding_8086::RegisterManager::new();
            let memory_store = &mut instruction_decoding_8086::MemoryManager::new();
            let segment_register_store =
                &mut instruction_decoding_8086::SegmentRegisterManager::new();
            let flag_register_store = &mut instruction_decoding_8086::FlagRegisterManager::new();

            let path = Path::new(file!())
                .parent()
                .unwrap()
                .join(Path::new("./test_listings"))
                .join(Path::new(stringify!($listing_name)));
            let input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));

            let reader = BufReader::new(input);

            instruction_decoding_8086::simulate(
                reader,
                register_store,
                memory_store,
                segment_register_store,
                flag_register_store,
            );

            insta::assert_debug_snapshot!(register_store.register_memory_map());
            insta::assert_debug_snapshot!(segment_register_store.segment_register_memory_map());
            insta::assert_debug_snapshot!(flag_register_store.flag_register_memory_map());
        }
    };
}

test_simulate_listing!(listing_0043_immediate_movs);
test_simulate_listing!(listing_0044_register_movs);
test_simulate_listing!(listing_0045_challenge_register_movs);
test_simulate_listing!(listing_0046_add_sub_cmp);
test_simulate_listing!(listing_0047_challenge_flags);
