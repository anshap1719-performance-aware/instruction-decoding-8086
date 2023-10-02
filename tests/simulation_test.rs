use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::Path;

#[macro_export]
macro_rules! test_simulate_listing {
    ($listing_name:ident, $should_include_clock_cycles:literal) => {
        #[test]
        fn $listing_name() {
            let store = &mut instruction_decoding_8086::Store::default();

            let path = Path::new(file!())
                .parent()
                .unwrap()
                .join(Path::new("./test_listings"))
                .join(Path::new(stringify!($listing_name)));

            let input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));

            let mut reader = BufReader::new(input);
            let (modded_reader, num_cpu_cycles) =
                instruction_decoding_8086::simulate(reader, store);
            reader = modded_reader;

            insta::assert_debug_snapshot!(reader.stream_position());
            insta::assert_debug_snapshot!(store.register_store().register_memory_map());
            insta::assert_debug_snapshot!(store
                .segment_register_store()
                .segment_register_memory_map());
            insta::assert_debug_snapshot!(store.flag_register_store().flag_register_memory_map());

            if ($should_include_clock_cycles) {
                insta::assert_snapshot!(num_cpu_cycles.to_string())
            }
        }
    };
}

test_simulate_listing!(listing_0043_immediate_movs, false);
test_simulate_listing!(listing_0044_register_movs, false);
test_simulate_listing!(listing_0045_challenge_register_movs, false);
test_simulate_listing!(listing_0046_add_sub_cmp, false);
test_simulate_listing!(listing_0047_challenge_flags, false);
test_simulate_listing!(listing_0048_ip_register, false);
test_simulate_listing!(listing_0049_conditional_jumps, false);
test_simulate_listing!(listing_0050_challenge_jumps, false);
test_simulate_listing!(listing_0051_memory_mov, false);
test_simulate_listing!(listing_0052_memory_add_loop, false);
test_simulate_listing!(listing_0053_add_loop_challenge, false);
test_simulate_listing!(listing_0056_estimating_cycles, true);
test_simulate_listing!(listing_0057_challenge_cycles, true);
