use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

fn main() {
    let path = Path::new("./tests/test_listings");

    println!("{}", path.to_str().unwrap());

    let files: Vec<String> = path
        .read_dir()
        .expect("read_dir call failed")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().unwrap_or(OsStr::new("")).to_str().unwrap() == "asm" {
                    Some(path.to_str().unwrap().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    for file in files {
        Command::new("nasm")
            .arg(file.clone())
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to compile {file}"));
    }
}
