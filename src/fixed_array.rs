use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
use lazy_static::lazy_static;
use crate::BareType;
use Box;

use crate::into_rust::into_rust;


// Ideas:
// 1. Create a table of pointers to each Type and lookup each ruby object's wrapped rust class in that table


// VTable
// for multiple children you get a struct with fields for each member function
// You get a ptr into this table of tables
// [
//     { ... },
//     {"name": 0x123, "blargh()": 0x22222 }
// ]

#[derive(Clone, Debug)]
pub struct RustFixedArray {
    len: i64,
    array_type: Box<dyn BareType>,
}

wrappable_struct! {
    RustFixedArray,
    RustFixedArrayWrap,
    RUST_FIXED_ARRAY_WRAP,

    mark(data) {
        // GC::mark(&data.val)
    }
}

impl RustFixedArray {
    pub fn rust_new(len: i64, array_type: Box<dyn BareType>) -> Self {
        RustFixedArray {
            len,
            array_type,
        }
    }
    pub fn new(val: Fixnum, typ: AnyObject) -> Self {
        let mut typ = typ.clone();
        println!("New Rust Fixed Array");
        let ret = RustFixedArray {
            len: val.to_i64(),
            array_type: into_rust(&mut typ),
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

class!(BareFixedArray);

methods! {
    BareFixedArray,
    rtself,

    fn new(input: Fixnum, typ: AnyObject) -> AnyObject {
        let fixed_array = RustFixedArray::new(input.unwrap(), typ.unwrap());
        let ret = Class::from_existing("BareFixedArray").wrap_data(fixed_array, &*RUST_FIXED_ARRAY_WRAP);
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
pub fn fixed_array_init() {
    let data_class = Class::from_existing("Object");
    Class::new("BareFixedArray", Some(&data_class)).define(|klass| {
        klass.def_self("new", new);
        klass.def("encode", encode);
        klass.def("decode", decode);
    });
}