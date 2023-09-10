use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[macro_export]
macro_rules! test_decode_listing {
    ($listing_name:ident) => {
        #[test]
        fn $listing_name() {
            let path = Path::new(file!())
                .parent()
                .unwrap()
                .join(Path::new("./test_listings"))
                .join(Path::new(stringify!($listing_name)));

            let input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));
            let reader = BufReader::new(input);
            let output = instruction_decoding_8086::decode(reader);

            insta::assert_snapshot!(output);
        }
    };
}

test_decode_listing!(listing_0037_single_register_mov);
test_decode_listing!(listing_0038_many_register_mov);
test_decode_listing!(listing_0039_more_movs);
test_decode_listing!(listing_0040_challenge_movs);
test_decode_listing!(listing_0041_add_sub_cmp_jnz);
