use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    writeln!(
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("table.rs")).unwrap(),
        "const CHINESE_WORD_TABLE: &[char] = &{:?};",
        include_str!("chinese-words-2500.txt").chars().collect::<Vec<char>>()
    ).unwrap();
}
