use std::env::temp_dir;
use std::fs::{remove_file, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::Command;

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

            let temp_file_path = path
                .join(temp_dir())
                .join(format!("{}.asm", stringify!($listing_name)));
            let mut temp_file = File::create(&temp_file_path)
                .unwrap_or_else(|_| panic!("Failed to create {path:?}"));
            temp_file.write_all(output.as_bytes()).unwrap();

            Command::new("nasm")
                .arg(temp_file_path.to_str().unwrap())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            let mut input = File::open(&path).unwrap_or_else(|_| panic!("Failed to open {path:?}"));
            let mut data = vec![];
            input.read_to_end(&mut data).unwrap();

            let mut temp_input = File::open(&temp_file_path.to_str().unwrap().replace(".asm", ""))
                .unwrap_or_else(|_| panic!("Failed to open {path:?}"));
            let mut generated_bin = vec![];
            temp_input.read_to_end(&mut generated_bin).unwrap();

            remove_file(temp_file_path).unwrap();

            assert_eq!(generated_bin, data);
        }
    };
}

test_decode_listing!(listing_0037_single_register_mov);
test_decode_listing!(listing_0038_many_register_mov);
test_decode_listing!(listing_0039_more_movs);
test_decode_listing!(listing_0040_challenge_movs);
test_decode_listing!(listing_0041_add_sub_cmp_jnz);
