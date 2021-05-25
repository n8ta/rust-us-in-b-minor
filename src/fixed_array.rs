use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
use lazy_static::lazy_static;
use crate::{BareType, init};
use Box;

use crate::into_rust::wrapper_to_rust_type;
use std::rc::Rc;
use crate::float32::RustFloat32;

pub struct RustFixedArray {
    len: i64,
    array_type: Rc<dyn BareType>,
}

const NAME: &str = "ArrayFixedLen";

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

class!(ArrayFixedLen);

methods! {
    ArrayFixedLen,
    rtself,

    fn new(input: Fixnum, typ: AnyObject) -> AnyObject {
        let fixed_array = Rc::new(RustFixedArray::new(input.unwrap(), typ.unwrap()));
        let ret = Class::from_existing(NAME).wrap_data(fixed_array, &*RUST_FIXED_ARRAY_WRAP);
        ret
    }

    fn encode(message: AnyObject) -> RString {
        let array = rtself.get_data_mut(&*RUST_FIXED_ARRAY_WRAP);
        let mut bytes = vec![];
        array.encode(message.unwrap(),
                         &mut bytes);
        RString::from_bytes(bytes.as_slice(), &Encoding::us_ascii())
    }

    fn decode(to_decode: RString) -> AnyObject {
        let rstr = to_decode.unwrap().try_convert_to::<RString>().unwrap();
        let mut bytes = rstr.to_bytes_unchecked();
        let array = rtself.get_data_mut(&*RUST_FIXED_ARRAY_WRAP);
        let (_, array) = array.decode(bytes);
        array
    }

}

init!(fixed_array_init, NAME);