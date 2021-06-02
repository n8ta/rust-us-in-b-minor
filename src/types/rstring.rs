use crate::{init, into_rust, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::value::ValueType::Hash;
use rutie::{AnyException, AnyObject, Class, Encoding, Float, Integer, Object, RString};
use std::{collections::HashMap, rc::Rc};
use crate::types::data::RustData;
use crate::types::uint::RustUint;
use std::str;

const NAME: &str = "Rust_String";

#[derive(Clone, Debug)]
pub struct RustString;

impl RustString {
    pub fn new() -> Self {
        RustString {}
    }
}

impl BareType for RustString {
    fn encode(&self, data: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let data = data.try_convert_to::<rutie::RString>().unwrap();
        let bs = data.to_bytes_unchecked();
        RustUint::new().encode(
            Integer::new(bs.len() as i64).into(),
            bytes,
        )?;
        bytes.extend(bs);
        Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (bytes, size) = RustUint::new().decode(bytes);
        let size = size.try_convert_to::<rutie::Integer>().unwrap().to_i64() as usize;
        let string = str::from_utf8(&bytes[0..size]).unwrap();
        let rstring = RString::new_utf8(string);
        (&bytes[size..], rstring.into())
    }
}

type RustStringRc = Rc<RustString>;

wrappable_struct! {
    RustStringRc,
    RustStringRcWrap,
    RUST_STRING_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareData,
    RUST_STRING_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustString::new()), &*RUST_STRING_WRAP)
    }
);

init!(rstring_init, NAME);
