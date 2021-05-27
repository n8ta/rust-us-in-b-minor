use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;
use rutie::rubysys::value::RubySpecialFlags::FixnumFlag;

const NAME: &str = "Rust_I8";

#[derive(Clone, Debug)]
pub struct RustI8;


impl RustI8 {
    pub fn new() -> Self {
        RustI8
    }
}

#[test]
fn test_enc() {
    let mut bytes = vec![];
    RustI8::new().encode(Fixnum::new(-1).into(), &mut bytes);
    assert_eq!(bytes, [0xFF]);

    let mut bytes = vec![];
    RustI8::new().encode(Fixnum::new(7).into(), &mut bytes);
    assert_eq!(bytes, [0x07]);
}

#[test]
fn test_dec() {
    let mut bytes = vec![0xFF];
    let (_, obj) = RustI8::new().decode(&mut bytes);
    let obj = obj.try_convert_to::<Fixnum>().unwrap().to_i64();
    assert_eq!(obj, -1)
}

impl BareType for RustI8 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let val = num.try_convert_to::<Fixnum>()?.to_i64() as i8;
        bytes.push(val as u8);
        return Ok(());
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let byte = bytes[0] as i8;
        let number = Fixnum::new(byte as i64);
        return (&bytes[1..], number.into());
    }
}

type RustI8Rc = Rc<RustI8>;

wrappable_struct! {
    RustI8Rc,
    RustI8Wrap,
    RUST_I8_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareI8,
    RUST_I8_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustI8::new()), &*RUST_I8_WRAP)
    }
);

init!(i8_init, NAME);