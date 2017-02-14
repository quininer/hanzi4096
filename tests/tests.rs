extern crate quickcheck;
extern crate hanzi4096;

use std::io::{ Write, Read };
use quickcheck::quickcheck;
use hanzi4096::{ ZiWrite, ZiRead };


#[test]
fn test_encode_then_decode() {
    fn encode_then_decode(input: Vec<u8>) -> bool {
        let output = hanzi4096::decode(&hanzi4096::encode(&input)).unwrap();
        input == output
    }
    quickcheck(encode_then_decode as fn(_: Vec<u8>) -> bool);
}

#[test]
fn test_two_write() {
    fn two_write(input: Vec<u8>) -> bool {
        let x = input.len() / 2;

        let mut w = ZiWrite::new();
        w.write(&input[..x]).unwrap();
        w.write(&input[x..]).unwrap();
        w.flush().unwrap();

        let mut output = Vec::new();
        let mut r = ZiRead::from(w.into_string());
        r.read_to_end(&mut output).unwrap();

        input == output
    }

    quickcheck(two_write as fn(_: Vec<u8>) -> bool);
}

#[test]
fn test_two_read() {
    fn two_read(input: Vec<u8>) -> bool {
        let x = input.len() / 2;

        let mut w = ZiWrite::new();
        w.write(&input).unwrap();
        w.flush().unwrap();

        let mut output = vec![0; input.len()];
        let mut r = ZiRead::from(w.into_string());
        r.read(&mut output[..x]).unwrap();
        r.read(&mut output[x..]).unwrap();

        input == output
    }

    quickcheck(two_read as fn(_: Vec<u8>) -> bool);
}

#[test]
fn test_half() {
    let input = [255; hanzi4096::CHAR_BITS - 1];

    let mut w = ZiWrite::new();
    w.write(&input).unwrap();
    w.flush().unwrap();

    let mut output = Vec::new();
    let mut r = ZiRead::from(w.into_string());
    r.read_to_end(&mut output).unwrap();
    assert_eq!(output, input);
}
