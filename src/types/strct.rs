use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;
use rutie::rubysys::value::ValueType::Hash;

const NAME: &str = "Rust_struct";

#[derive(Clone, Debug)]
pub struct RustStruct {
    mapping: Hashmap<rutie::Symbol, Rc<dyn BareType>>
}


impl RustStruct {
    pub fn new(hash: AnyObject) -> Self {
        let hash = hash.try_convert_to::<rutie::Hash>().unwrap();
        hash.each(|key, value| {
            println!("yo");
        });
        RustStruct {
            mapping: Hashmap::new(),
        }
    }
}

impl BareType for RustStruct {
    fn encode(&self, fl: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let x =1;
    }
}

type RustStructRc = Rc<RustStruct>;

wrappable_struct! {
    RustStructRc,
    RustStructRcWrap,
    RUST_STRUCT_RC,
    mark(data) {}
}

ruby_methods!(
    BareFloat64,
    RUST_FLOAT_32_WRAP,
    fn new(hash: AnyObject) {
        Class::from_existing(NAME).wrap_data(Rc::new(RustStruct::new(hash)), &*RUST_STRUCT_RC)
    }
);

init!(struct_init, NAME);