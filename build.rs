use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    writeln!(
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("table.rs")).unwrap(),
        "const CHINESE_CHAR_TABLE: &[char] = &{:?};",
        include_str!("chinese-chars.txt")
            .lines()
            .filter_map(|line| if line.is_empty() || line.starts_with("//") {
                None
            } else {
                line.split_whitespace()
                    .last()
                    .and_then(|c| c.chars().next())
            })
            .take(1 << 12)
            .collect::<Vec<char>>()
    ).unwrap();
}
