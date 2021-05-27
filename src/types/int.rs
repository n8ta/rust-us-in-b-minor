use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use Box;

use crate::into_rust::wrapper_to_rust_type;
use std::rc::Rc;
use std::path::Iter;
use rutie::rubysys::value::ValueType::Float;
use rutie::rubysys::value::RubySpecialFlags::FixnumFlag;
use crate::types::uint::RustUint;

pub struct RustInt;

const NAME: &str = "Rust_Int";

type RustIntRC = Rc<RustInt>;

wrappable_struct! {
    RustIntRC,
    RustIntRCWrap,
    RUST_INT_WRAP,
    mark(data) {}
}

impl RustInt {
    pub fn new() -> Self {
        RustInt {}
    }
}


impl BareType for RustInt {

    // Just re-use uint, fun lil hack
    // https://developers.google.com/protocol-buffers/docs/encoding

    fn encode(&self, input: AnyObject, bytes: &mut Vec<u8>) -> std::result::Result<(), AnyException> {
        let num = input.try_convert_to::<Fixnum>()?.to_i64();
        let map_to_integer = if num < 0 { (-2 * num) - 1 } else {  num * 2  };
        RustUint.encode(Fixnum::new(map_to_integer).into(), bytes)
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (bytes, number) = RustUint::new().decode(bytes);
        let number = number.try_convert_to::<Fixnum>().unwrap().to_i64();
        let number = if number % 2 != 0 { (number + 1) / -2 } else { number / 2 };
        (bytes, Fixnum::new(number).into())
    }
}

ruby_methods!(
    Int,
    RUST_INT_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustInt::new()), &*RUST_INT_WRAP)
    }
);


init!(int_init, NAME);