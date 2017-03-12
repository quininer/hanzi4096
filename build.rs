extern crate phf_codegen;

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
    let chinese_char_table = chinese_chars
        .by_ref()
        .take(1 << 12)
        .collect::<Vec<char>>();
    let end_chinese_char_table = chinese_chars
        .take(1 << 11)
        .collect::<Vec<char>>();

    writeln!(
        table_file,
        "const CHINESE_CHAR_TABLE: &[char] = &{:?};",
        chinese_char_table
    ).unwrap();

    writeln!(
        table_file,
        "const END_CHINESE_CHAR_TABLE: &[char] = &{:?};",
        end_chinese_char_table
    ).unwrap();

    writeln!(
        table_file,
        "const INV_CHINESE_CHAR_MAP: phf::Map<char, u16> ="
    ).unwrap();
    let mut builder = phf_codegen::Map::new();
    for (c, i) in chinese_char_table.iter()
        .enumerate()
        .map(|(i, &c)| (c, i.to_string()))
    {
        builder.entry(c, &i);
    }
    builder.build(&mut table_file).unwrap();
    writeln!(table_file, ";").unwrap();

    writeln!(
        table_file,
        "const INV_END_CHINESE_CHAR_TABLE: phf::Map<char, u16> ="
    ).unwrap();
    let mut builder = phf_codegen::Map::new();
    for (c, i) in end_chinese_char_table.iter()
        .enumerate()
        .map(|(i, &c)| (c, i.to_string()))
    {
        builder.entry(c, &i);
    }
    builder.build(&mut table_file).unwrap();
    writeln!(table_file, ";").unwrap();
}
