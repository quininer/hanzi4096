use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut table_file = File::create(
        Path::new(&env::var("OUT_DIR").unwrap()).join("table.rs")
    ).unwrap();
    let mut chinese_chars = include_str!("chinese-chars.txt")
        .lines()
        .filter_map(|line| if line.is_empty() || line.starts_with("//") {
            None
        } else {
            line.split_whitespace()
                .last()
                .and_then(|c| c.chars().next())
        });

    writeln!(
        table_file,
        "const CHINESE_CHAR_TABLE: &[char] = &{:?};",
        chinese_chars
            .by_ref()
            .take(1 << 12)
            .collect::<Vec<char>>()
    ).unwrap();

    writeln!(
        table_file,
        "const END_CHINESE_CHAR_TABLE: &[char] = &{:?};",
        chinese_chars
            .take(1 << 11)
            .collect::<Vec<char>>()
    ).unwrap();
}
