use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;
use rutie::rubysys::value::RubySpecialFlags::FixnumFlag;

const NAME: &str = "Rust_I64";

#[derive(Clone, Debug)]
pub struct RustI64;


impl RustI64 {
    pub fn new() -> Self {
        RustI64
    }
}

impl BareType for RustI64 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let num_bytes = num.try_convert_to::<Fixnum>()?.to_i64().to_le_bytes();
        for byte in num_bytes.iter() {
            bytes.push(*byte)
        }
        return Ok(());
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let num_bytes: [u8; 8] = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]];
        let num = i64::from_le_bytes(num_bytes);
        let number = Fixnum::new(num);
        return (&bytes[8..], number.into());
    }
}

type RustI64Rc = Rc<RustI64>;

wrappable_struct! {
    RustI64Rc,
    RustI64Wrap,
    RUST_I64_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareI64,
    RUST_I64_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustI64::new()), &*RUST_I64_WRAP)
    }
);

init!(i64_init, NAME);