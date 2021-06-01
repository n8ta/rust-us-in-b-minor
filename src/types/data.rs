use super::uint::RustUint;
use crate::{init, into_rust, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::value::ValueType::Hash;
use rutie::{AnyException, AnyObject, Class, Encoding, Float, Integer, Object, RString};
use std::{collections::HashMap, rc::Rc};

const NAME: &str = "Rust_Data";

#[derive(Clone, Debug)]
pub struct RustData;

impl RustData {
    pub fn new() -> Self {
        RustData {}
    }
}

impl BareType for RustData {
    fn encode(&self, data: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let data_as_rstr = data.try_convert_to::<RString>()?;
        let data = data_as_rstr.to_bytes_unchecked();
        RustUint.encode(Integer::from(data.len() as u64).into(), bytes)?;
        bytes.extend(data);
        Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (bytes, size) = RustUint.decode(bytes);
        let size = size.try_convert_to::<Integer>().unwrap().to_u64() as usize;
        let r_str = RString::from_bytes(&bytes[..size], &Encoding::us_ascii());
        (&bytes[size..], r_str.into())
    }
}

type RustDataRc = Rc<RustData>;

wrappable_struct! {
    RustDataRc,
    RustDataRcWrap,
    RUST_DATA_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareStruct,
    RUST_DATA_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustData::new()), &*RUST_DATA_WRAP)
    }
);

init!(data_init, NAME);
