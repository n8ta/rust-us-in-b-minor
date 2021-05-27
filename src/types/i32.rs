use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;

const NAME: &str = "Rust_I32";

#[derive(Clone, Debug)]
pub struct RustI32;


impl RustI32 {
    pub fn new() -> Self {
        RustI32
    }
}

impl BareType for RustI32 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let num_bytes = (num.try_convert_to::<Fixnum>()?.to_i64() as i32).to_le_bytes();
        for byte in num_bytes.iter() {
            bytes.push(*byte)
        }
        return Ok(());
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let num_bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let num = i32::from_le_bytes(num_bytes);
        let number = Fixnum::new(num as i64);
        return (&bytes[4..], number.into());
    }
}

type RustI32Rc = Rc<RustI32>;

wrappable_struct! {
    RustI32Rc,
    RustI32Wrap,
    RUST_I32_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareI32,
    RUST_I32_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustI32::new()), &*RUST_I32_WRAP)
    }
);

init!(i32_init, NAME);