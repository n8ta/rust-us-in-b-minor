use super::uint::RustUint;
use crate::{init, into_rust, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::value::ValueType::Hash;
use rutie::{AnyException, AnyObject, Class, Encoding, Float, Integer, Object, RString};
use std::{collections::HashMap, rc::Rc};

const NAME: &str = "Rust_Map";

#[derive(Clone /* Debug */)]
pub struct RustMap {
    key_type: Rc<dyn BareType>,
    value_type: Rc<dyn BareType>,
}

impl RustMap {
    pub fn new(mut key_type: AnyObject, mut value_type: AnyObject) -> Self {
        RustMap {
            key_type: into_rust::wrapper_to_rust_type(&mut key_type),
            value_type: into_rust::wrapper_to_rust_type(&mut value_type),
        }
    }
}

impl BareType for RustMap {
    fn encode(&self, map: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let map = map.try_convert_to::<rutie::Hash>()?;
        let size = map.length();
        RustUint.encode(Integer::from(size as u64).into(), bytes);
        let mut exception = Ok(());
        map.each(|key, value| {
            if let Err(e) = self.key_type.encode(key, bytes) {
                exception = Err(e);
            }
            if let Err(e) = self.value_type.encode(value, bytes) {
                exception = Err(e);
            }
        });
        exception
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (mut bytes, count) = RustUint.decode(bytes);
        let count = count.try_convert_to::<Integer>().unwrap().to_u64() as usize;
        let mut map = rutie::Hash::new();
        for _ in 0..count {
            let (new_bytes, key) = self.key_type.decode(bytes);
            let (new_bytes, value) = self.value_type.decode(new_bytes);
            map.store(key, value);
            bytes = new_bytes;
        }
        (bytes, map.into())
    }
}

type RustMapRc = Rc<RustMap>;

wrappable_struct! {
    RustMapRc,
    RustMapRcWrap,
    RUST_MAP_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareMap,
    RUST_MAP_WRAP,
    fn new(key_type: AnyObject, value_type: AnyObject) {
        Class::from_existing(NAME).wrap_data(
            Rc::new(RustMap::new(key_type.unwrap(), value_type.unwrap())),
            &*RUST_MAP_WRAP,
        )
    }
);

init!(map_init, NAME);
