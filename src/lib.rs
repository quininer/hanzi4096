#![feature(non_ascii_idents, static_in_const, more_struct_aliases)]

#[macro_use] extern crate lazy_static;

use std::io::{ self, Write, Read };
use std::cmp::min;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/table.rs"));

lazy_static! {
    static ref INV_CHINESE_WORD_MAP: HashMap<char, u16> = {
        CHINESE_CHAR_TABLE // XXX: maybe const ?
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i as u16))
            .collect()
    };

    static ref INV_END_CHINESE_CHAR_TABLE: HashMap<char, u16> = {
        END_CHINESE_CHAR_TABLE
            .iter()
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
/// write!(w, "Hello 汉字!").unwrap();
/// assert_eq!(w.into_string(), "贰娃迤交杀萝尻淳");
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
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buff: String::with_capacity(capacity),
            char_buf: 0,
            bits: 0
        }
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
            self.buff.push(if self.bits < 12 {
                END_CHINESE_CHAR_TABLE[self.char_buf as usize]
            } else {
                CHINESE_CHAR_TABLE[self.char_buf as usize]
            });
            self.char_buf = 0;
            self.bits = 0;
        }

        Ok(())
    }
}


/// ```
/// use std::io::Read;
/// use hanzi4096::ZiRead;
///
/// let mut r = ZiRead::from(
///     "桃之夭夭灼灼其华之子于归宜其室家"
/// );
/// let mut output = [0; 24];
/// r.read(&mut output).unwrap();
/// assert_eq!(
///     output,
///     [51, 151, 3, 125, 208, 7, 84, 67, 53, 227, 115, 29, 57, 240, 3, 23, 144, 14, 253, 52, 62, 160, 38, 131]
/// );
/// ```
#[derive(Debug, Clone)]
pub struct 字读 {
    buff: String,
    cursor: usize,
    bits: usize,
    ignore_flag: bool
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
            bits: 0,
            ignore_flag: false
        }
    }
}

impl 字读 {
    /// Ignore invalid char.
    ///
    /// ```
    /// use std::io::Read;
    /// use hanzi4096::{ self, ZiRead };
    ///
    /// let text = "
    ///     南有乔木 不可休息
    ///     汉有游女 不可求思
    ///     汉之广矣 不可泳思
    ///     江之永矣 不可方思
    /// ";
    ///
    /// let mut r = ZiRead::from(text);
    /// let mut output = Vec::new();
    /// r.with_ignore(true);
    /// r.read_to_end(&mut output).unwrap();
    ///
    /// assert_eq!(
    ///     hanzi4096::encode(&output),
    ///     text.lines()
    ///         .flat_map(|line| line.split_whitespace())
    ///         .collect::<String>()
    /// );
    /// ```
    pub fn with_ignore(&mut self, flag: bool) -> &mut Self {
        self.ignore_flag = flag;
        self
    }
}

impl Read for 字读 {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut count = 0;
        let mut byte_bits = 0;
        let mut end = false;

        for c in self.buff.chars().skip(self.cursor) {
            let mut b = match INV_CHINESE_WORD_MAP.get(&c) {
                Some(&b) => b,
                None => match INV_END_CHINESE_CHAR_TABLE.get(&c) {
                    Some(&b) => {
                        end = true;
                        b
                    },
                    None => {
                        self.cursor += 1;
                        if self.ignore_flag {
                            continue;
                        } else {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, c.to_string()));
                        }
                    }
                }
            };
            b >>= self.bits;

            loop {
                if count >= buf.len() {
                    return Ok(count);
                }

                let min_left = min(CHAR_BITS - self.bits, BYTE_BITS - byte_bits);

                let bb = b & ((1 << min_left) - 1);
                buf[count] |= (bb as u8) << byte_bits;
                b >>= min_left;
                self.bits += min_left;
                byte_bits += min_left;

                if end {
                    self.bits = CHAR_BITS;
                    byte_bits = BYTE_BITS;
                }
                if byte_bits >= BYTE_BITS {
                    count += 1;
                    byte_bits -= BYTE_BITS;
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
    let mut w = 字写::with_capacity(
        (input.len() as f64 * BYTE_BITS as f64 / CHAR_BITS as f64).ceil() as usize
        * 3
    );
    w.write(input).expect("unreachable");
    w.flush().expect("unreachable");
    w.into_string()
}

#[inline]
pub fn decode(input: &str) -> io::Result<Vec<u8>> {
    let mut r = 字读::from(input);
    let mut output = Vec::with_capacity(input.chars().count() * CHAR_BITS / 8);
    r.read_to_end(&mut output)?;
    Ok(output)
}
