use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use Box;

use crate::into_rust::wrapper_to_rust_type;
use std::rc::Rc;
use crate::float32::RustFloat32;

pub struct RustFixedArray {
    len: i64,
    array_type: Rc<dyn BareType>,
}

const NAME: &str = "Rust_ArrayFixedLen";

type RustFixedArrayRC = Rc<RustFixedArray>;

wrappable_struct! {
    RustFixedArrayRC,
    RustFixedArrayWrap,
    RUST_FIXED_ARRAY_WRAP,
    mark(data) {}
}

impl RustFixedArray {
    pub fn new(val: Fixnum, typ: AnyObject) -> Self {
        let mut typ = typ.clone();
        let ret = RustFixedArray {
            len: val.to_i64(),
            array_type: wrapper_to_rust_type(&mut typ)
        };
        ret
    }
}

impl BareType for RustFixedArray {
    fn encode(&self, input: AnyObject, bytes: &mut Vec<u8>) -> std::result::Result<(),AnyException> {
        let array = input.try_convert_to::<Array>().unwrap();
        for idx in 0..self.len {
            self.array_type.encode(array.at(idx), bytes);
        }
        Result::Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut array = Array::new();
        let mut bytes = bytes;
        for _ in 0..self.len {
            let (remaining_bytes, decoded) = self.array_type.decode(bytes);
            bytes = remaining_bytes;
            array.push(decoded);
        }
        (bytes, array.into())
    }
}


ruby_methods!(
    ArrayFixedLen,
    RUST_FIXED_ARRAY_WRAP
);

methods! {
    ArrayFixedLen,
    rtself,
    fn new(input: Fixnum, typ: AnyObject) -> AnyObject {
        let fixed_array = Rc::new(RustFixedArray::new(input.unwrap(), typ.unwrap()));
        let ret = Class::from_existing(NAME).wrap_data(fixed_array, &*RUST_FIXED_ARRAY_WRAP);
        ret
    }
}

init!(fixed_array_init, NAME);