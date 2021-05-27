use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;

const NAME: &str = "Rust_I16";

#[derive(Clone, Debug)]
pub struct RustI16;


impl RustI16 {
    pub fn new() -> Self {
        RustI16
    }
}

impl BareType for RustI16 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let num_bytes = (num.try_convert_to::<Fixnum>()?.to_i64() as i16).to_le_bytes();
        for byte in num_bytes.iter() {
            bytes.push(*byte)
        }
        return Ok(());
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let num_bytes: [u8; 2] = [bytes[0], bytes[1]];
        let num = i16::from_le_bytes(num_bytes);
        let number = Fixnum::new(num as i64);
        return (&bytes[2..], number.into());
    }
}

type RustI16Rc = Rc<RustI16>;

wrappable_struct! {
    RustI16Rc,
    RustI16Wrap,
    RUST_I16_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareI16,
    RUST_I16_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustI16::new()), &*RUST_I16_WRAP)
    }
);

init!(i16_init, NAME);