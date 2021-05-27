use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use Box;

use crate::into_rust::wrapper_to_rust_type;
use std::rc::Rc;
use crate::types::uint::RustUint;
use rutie::rubysys::value::RubySpecialFlags::FixnumFlag;

pub struct RustArray {
    array_type: Rc<dyn BareType>,
}

const NAME: &str = "Rust_Array";

type RustArrayRC = Rc<RustArray>;

wrappable_struct! {
    RustArrayRC,
    RustArrayWrap,
    RUST_ARRAY_WRAP,
    mark(data) {}
}

impl RustArray {
    pub fn new(typ: AnyObject) -> Self {
        let mut typ = typ.clone();
        RustArray {
            array_type: wrapper_to_rust_type(&mut typ)
        }
    }
}

impl BareType for RustArray {
    fn encode(&self, input: AnyObject, bytes: &mut Vec<u8>) -> std::result::Result<(),AnyException> {
        let array = input.try_convert_to::<Array>()?;
        let len = array.length();

        RustUint::new().encode(Fixnum::new(len as i64).into(), bytes);
        for i in 0..len {
            self.array_type.encode(array.at(i as i64), bytes);
        }
        Result::Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut array = Array::new();

        let (after_uint_bytes, length) = RustUint::new().decode(bytes);

        let mut bytes = after_uint_bytes;

        let len = length.try_convert_to::<Fixnum>().unwrap().to_i64();

        for _ in 0..len {
            let (remaining_bytes, decoded) = self.array_type.decode(bytes);
            bytes = remaining_bytes;
            array.push(decoded);
        }
        (bytes, array.into())
    }
}

ruby_methods!(
    ArrayFixedLen,
    RUST_ARRAY_WRAP,
    fn new(typ: AnyObject,) {
        let fixed_array = Rc::new(RustArray::new(typ.unwrap()));
        let ret = Class::from_existing(NAME).wrap_data(fixed_array, &*RUST_ARRAY_WRAP);
        ret
    }
);


init!(array_init, NAME);