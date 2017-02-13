#![feature(non_ascii_idents, static_in_const, more_struct_aliases)]

#[macro_use] extern crate lazy_static;

use std::io::{ self, Write, Read };
use std::cmp::min;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/table.rs"));

lazy_static! {
    static ref INV_CHINESE_WORD_MAP: HashMap<char, u16> = {
        CHINESE_WORD_TABLE // XXX: maybe const ?
            .iter()
            .take(1 << CHAR_BITS)
            .enumerate()
            .map(|(i, &c)| (c, i as u16))
            .collect()
    };
}

pub const CHAR_BITS: usize = 12;
const BYTE_BITS: usize = 8;


/// ```
/// use std::io::Write;
/// use hanzi4096::ZiWrite;
///
/// let mut w = ZiWrite::new();
/// write!(w, "hello 汉字!").unwrap();
/// assert_eq!(w.into_string(), "拴娃迤交杀萝尻淳");
/// ```
#[derive(Debug, Clone)]
pub struct 字写 {
    buff: String,
    char_buf: u16,
    bits: usize
}

pub type ZiWrite = 字写;

impl Default for 字写 {
    fn default() -> Self {
        Self {
            buff: String::new(),
            char_buf: 0,
            bits: 0
        }
    }
}

impl 字写 {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.buff
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.buff
    }
}

impl Write for 字写 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for mut b in buf.iter().map(|&b| b as u16) {
            let bits_left = CHAR_BITS - self.bits;

            let bits = if let Some(bits) = BYTE_BITS.checked_sub(bits_left) {
                let bb = b & ((1 << bits_left) - 1);
                self.char_buf |= bb << self.bits;
                b >>= bits_left;
                self.bits += bits_left;

                self.flush()?;
                bits
            } else {
                BYTE_BITS
            };

            self.char_buf |= b << self.bits;
            self.bits += bits;

            if self.bits >= CHAR_BITS { self.flush()? };
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.bits > 0 {
            self.buff.push(CHINESE_WORD_TABLE[self.char_buf as usize]);
            self.char_buf = 0;
            self.bits = self.bits.checked_sub(CHAR_BITS).unwrap_or(0);
        }

        Ok(())
    }
}


/// ```
/// use std::io::Read;
/// use hanzi4096::ZiRead;
///
/// let mut r = ZiRead::from("桃之夭夭灼灼其华");
/// let mut output = [0; 12];
/// r.read(&mut output).unwrap();
/// assert_eq!(output, [51, 151, 3, 125, 208, 7, 84, 67, 53, 227, 115, 29]);
/// ```
#[derive(Debug, Clone)]
pub struct 字读 {
    buff: String,
    cursor: usize,
    bits: usize
}

pub type ZiRead = 字读;

impl<'a> From<&'a str> for 字读 {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<String> for 字读 {
    fn from(s: String) -> Self {
        Self {
            buff: s,
            cursor: 0,
            bits: 0
        }
    }
}

impl Read for 字读 {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut count = 0;
        let mut byte_bits = 0;

        for c in self.buff.chars().skip(self.cursor) {
            let mut i = *INV_CHINESE_WORD_MAP.get(&c)
                .ok_or_else(|| io::Error::new(
                    io::ErrorKind::InvalidData,
                    c.to_string()
                ))?;
            i >>= self.bits;

            loop {
                let min_left = min(CHAR_BITS - self.bits, BYTE_BITS - byte_bits);

                let ii = i & ((1 << min_left) - 1);
                buf[count] |= (ii as u8) << byte_bits;
                i >>= min_left;
                self.bits += min_left;
                byte_bits += min_left;

                if byte_bits >= BYTE_BITS {
                    count += 1;
                    byte_bits -= BYTE_BITS;
                }
                if count >= buf.len() {
                    return Ok(count);
                }
                if self.bits >= CHAR_BITS {
                    self.bits -= CHAR_BITS;
                    break;
                }
            }
            self.cursor += 1;
        }

        Ok(count)
    }
}

#[inline]
pub fn encode(input: &[u8]) -> String {
    let mut w = 字写::new();
    w.write(input).expect("unreachable");
    w.flush().expect("unreachable");
    w.into_string()
}

#[inline]
pub fn decode(input: &str, output: &mut [u8]) -> io::Result<()> {
    let mut r = 字读::from(input);
    r.read(output)?;
    Ok(())
}

#[test]
fn test_one_write_read() {
    let input = b"chinese char!";

    let mut w = 字写::new();
    w.write(input).unwrap();
    w.flush().unwrap();

    let mut r = 字读::from(w.into_string());
    let mut output = vec![0; input.len()];
    r.read(&mut output).unwrap();
    assert_eq!(output, input);
}

#[test]
fn test_two_write() {
    let input = b"oh my chinese char!";

    let mut w = 字写::new();
    w.write(&input[..5]).unwrap();
    w.write(&input[5..]).unwrap();
    w.flush().unwrap();

    let mut r = 字读::from(w.into_string());
    let mut output = vec![0; input.len()];
    r.read(&mut output).unwrap();
    assert_eq!(output, input);
}

#[test]
fn test_two_read() {
    let input = b"oh my chinese char!";

    let mut w = 字写::new();
    w.write(input).unwrap();
    w.flush().unwrap();

    let mut r = 字读::from(w.into_string());
    let mut output = vec![0; input.len()];
    r.read(&mut output[..5]).unwrap();
    r.read(&mut output[5..]).unwrap();
    assert_eq!(output, input);
}

#[test]
fn test_full() {
    let input = [255; 11];

    let mut w = 字写::new();
    w.write(&input).unwrap();
    w.flush().unwrap();

    let mut r = 字读::from(w.into_string());
    let mut output = Vec::new();
    r.read_to_end(&mut output).unwrap();
    assert_eq!(output[..input.len()], input);
    assert_ne!(output, input); // FIXME zero bit
}
