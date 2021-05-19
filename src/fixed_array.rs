use rutie::{Class, AnyObject, Object, Float, RString, Encoding, Fixnum, Array};
use lazy_static::lazy_static;
use crate::float64::RustFloat64;

pub struct RustFixedArray {
    len: i64
}

wrappable_struct! {
    RustFixedArray,
    RustFixedArrayWrap,
    RUST_FIXED_ARRAY_WRAP,

    mark(data) {}
}

impl RustFixedArray {
    fn new(val: Fixnum) -> Self {
        RustFixedArray {
            len: val.to_i64()
        }
    }

    // pub fn encode(&self, fl: Float, bytes: &mut Vec<u8>) {
    pub fn encode(&self, input: AnyObject, bytes: &mut Vec<u8>) {
        let array = input.try_convert_to::<Array>().unwrap();
        let float = RustFloat64::new();
        for idx in 0..self.len {
            float.encode(array.at(idx), bytes)
        }
    }
    pub fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], Array) {
        let mut array = Array::new();
        let float = RustFloat64::new();
        let mut bytes = bytes;
        for _ in 0..self.len {
            let (remaining_bytes, float) = float.decode(bytes);
            bytes = remaining_bytes;
            array.push(float);
        }
        (bytes, array)
    }
}

class!(BareFixedArray);

methods! {
    BareFixedArray,
    rtself,

    fn new(input: Fixnum) -> AnyObject {
        let fixed_array = RustFixedArray::new(input.unwrap());
        Class::from_existing("BareFixedArray").wrap_data(fixed_array, &*RUST_FIXED_ARRAY_WRAP)
    }

    fn encode(message: AnyObject) -> RString {
        let array = rtself.get_data_mut(&*RUST_FIXED_ARRAY_WRAP);
        let mut bytes = vec![];
        array.encode(message.unwrap(),
                         &mut bytes);
        RString::from_bytes(bytes.as_slice(), &Encoding::us_ascii())
    }

    fn decode(to_decode: RString) -> Array {
        let rstr = to_decode.unwrap().try_convert_to::<RString>().unwrap();
        let mut bytes = rstr.to_bytes_unchecked();
        let array = rtself.get_data_mut(&*RUST_FIXED_ARRAY_WRAP);
        let (_, array) = array.decode(bytes);
        array
    }

}
pub fn fixed_array_init() {
    let data_class = Class::from_existing("Object");
    Class::new("BareFixedArray", Some(&data_class)).define(|klass| {
        klass.def_self("new", new);
        klass.def("encode", encode);
        klass.def("decode", decode);
    });
}