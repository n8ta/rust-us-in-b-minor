use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use Box;

use crate::into_rust::wrapper_to_rust_type;
use std::rc::Rc;
use std::path::Iter;
use rutie::rubysys::value::ValueType::Float;
use rutie::rubysys::value::RubySpecialFlags::FixnumFlag;

pub struct RustUint;

const NAME: &str = "Rust_Uint";

type RustUintRC = Rc<RustUint>;

wrappable_struct! {
    RustUintRC,
    RustUintWrap,
    RUST_UINT_WRAP,
    mark(data) {}
}

impl RustUint {
    pub fn new() -> Self {
        RustUint {}
    }
}

struct Next7 {
    num: u64,
    pos: usize,
}

impl Next7 {
    fn new(num: u64) -> Self {
        Next7 { num, pos: 0 }
    }
}

impl Iterator for Next7 {
    type Item = u8;
    // 0b0xxxxxxx
    // first bit is always empty
    // rest are from the data

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        self.pos += 1;
        if pos * 7 < 64 {
            let ret = ((self.num >> (pos * 7)) & 0b01111111) as u8;
            Some(ret)
        } else {
            None
        }
    }
}

fn in_sevens(num: u64) -> Vec<u8> {
    /// Returns a vector of bytes (little endian)
    /// where the highest bit in every byte is zero
    /// and remaining bytes represent a uint le.
    let st = Next7::new(num);
    let mut bytes: Vec<u8> = st.collect();
    for _ in 0..bytes.len() - 1 {
        if let Some(byte) = bytes.pop() {
            if byte == 0 {
                continue;
            } else {
                bytes.push(byte);
                break;
            }
        } else {
            break;
        }
    }
    bytes
}

#[test]
fn test_next_7() {
    assert_eq!(in_sevens(5), vec![0b0000_0101]);
    assert_eq!(in_sevens(0), vec![0b0000_0000]);
    assert_eq!(in_sevens(69), vec![0b0100_0101]);
    assert_eq!(in_sevens(128), vec![0b0000_0000, 0b0000_0001]);
}

#[test]
fn test_encode() {
    let uint = RustUint::new();
    let mut bytes = vec![];
    uint.encode(Fixnum::new(16382).into(), &mut bytes);
    assert_eq!(bytes, vec![0xFE, 0x7F]);
}

#[test]
fn test_decode() {
    let uint = RustUint::new();
    let mut bytes = vec![0xFE, 0x7F];
    let (_, fixnum) = uint.decode(bytes.as_slice());
    let num = fixnum.try_convert_to::<Fixnum>().unwrap().to_i64();
    assert_eq!(num, 16382)
}

impl BareType for RustUint {
    fn encode(&self, input: AnyObject, bytes: &mut Vec<u8>) -> std::result::Result<(), AnyException> {
        let number = input.try_convert_to::<Fixnum>().unwrap().to_i64();
        ;
        let mut encoded_bytes = in_sevens(number as u64);
        for i in 0..encoded_bytes.len() - 1 {
            encoded_bytes[i] = encoded_bytes[i] | 0b1000_0000
        }
        for byte in encoded_bytes {
            bytes.push(byte);
        }
        Result::Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut result: u64 = 0;
        let mut i = 0;
        while let Some(byte) = bytes.get(i) {
            result = result | (((byte & 0b0111_1111) as u64) << (i * 7));
            if byte & 0b1000_0000 == 0 {
                break;
            }
            i += 1
        }
        (bytes, Fixnum::new(result as i64).into())
    }
}

ruby_methods!(
    Uint,
    RUST_UINT_WRAP,
    fn new() {
        let uint = Rc::new(RustUint::new());
        let ret = Class::from_existing(NAME).wrap_data(uint, &*RUST_UINT_WRAP);
        ret
    }
);


init!(uint_init, NAME);