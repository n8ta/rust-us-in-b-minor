use crate::{init, into_rust, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::value::ValueType::Hash;
use rutie::{AnyException, AnyObject, Class, Encoding, Fixnum, Float, Object, RString};
use std::{collections::HashMap, rc::Rc};

const NAME: &str = "Rust_Struct";

#[derive(Clone /* Debug */)]
pub struct RustStruct {
    mapping: rutie::Hash,
}

impl RustStruct {
    pub fn new(hash: AnyObject) -> Self {
        let mapping = hash.try_convert_to::<rutie::Hash>().unwrap();
        RustStruct { mapping }
    }
}

impl BareType for RustStruct {
    fn encode(&self, strct: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let strct = strct.try_convert_to::<rutie::Hash>()?;
        let mut exception = None;
        self.mapping.each(|field, mut value| {
            let bare_type = into_rust::wrapper_to_rust_type(&mut value);
            if let Err(e) = bare_type.encode(strct.at(&field), bytes) {
                exception = Some(e);
            }
        });
        if let Some(e) = exception {
            return Err(e);
        }
        Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut hash = rutie::Hash::new();
        let mut bytes = bytes;
        self.mapping.each(|field, mut value| {
            let bare_type = into_rust::wrapper_to_rust_type(&mut value);
            let (new_bytes, value) = bare_type.decode(bytes);
            bytes = new_bytes;
            hash.store(field, value);
        });

        (bytes, hash.into())
    }
}

type RustStructRc = Rc<RustStruct>;

wrappable_struct! {
    RustStructRc,
    RustStructRcWrap,
    RUST_STRUCT_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareStruct,
    RUST_STRUCT_WRAP,
    fn new(hash: AnyObject) {
        Class::from_existing(NAME)
            .wrap_data(Rc::new(RustStruct::new(hash.unwrap())), &*RUST_STRUCT_WRAP)
    }
);

init!(struct_init, NAME);
