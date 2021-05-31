use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods, ruby_methods_no_encode};
use Box;

use crate::into_rust::wrapper_to_rust_type;
use std::rc::Rc;
use std::panic::resume_unwind;

pub struct RustFixedData {
    len: usize,
}

const NAME: &str = "Rust_DataFixedLen";

type RustFixedDataRc = Rc<RustFixedData>;

wrappable_struct! {
    RustFixedDataRc,
    RustFixedDataRcWrap,
    RUST_FIXED_DATA_WRAP,
    mark(data) {}
}

impl RustFixedData {
    pub fn new(val: Fixnum) -> Self {
        RustFixedData {
            len: val.to_i64() as usize,
        }
    }
}

impl BareType for RustFixedData {
    fn encode(&self, input: AnyObject, bytes: &mut Vec<u8>) -> std::result::Result<(),AnyException> {
        let rstr = input.try_convert_to::<RString>()?;
        let data = rstr.to_bytes_unchecked();
        bytes.extend(&data[0..self.len]);
        Result::Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let r_str = RString::from_bytes(&bytes[0..self.len],
                                          &Encoding::us_ascii());
        (&bytes[self.len..], r_str.into())
    }
}

ruby_methods!(
    DataFixedLen,
    RUST_FIXED_DATA_WRAP,
    fn new(input: Fixnum) {
        let fixed_array = Rc::new(RustFixedData::new(input.unwrap()));
        let ret = Class::from_existing(NAME).wrap_data(fixed_array, &*RUST_FIXED_DATA_WRAP);
        ret
    }
);


init!(fixed_data_init, NAME);