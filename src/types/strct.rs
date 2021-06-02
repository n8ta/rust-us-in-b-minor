use crate::{init, into_rust, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::value::ValueType::{Hash, Symbol};
use rutie::{AnyException, AnyObject, Class, Encoding, Fixnum, Float, Object, RString};
use std::{collections::HashMap, rc::Rc};

const NAME: &str = "Rust_Struct";

#[derive(Clone /* Debug */)]
pub struct RustStruct {
    mapping: HashMap<String, Rc<dyn BareType>>,
    order: Vec<String>,
}

impl RustStruct {
    pub fn new(hash: AnyObject) -> Self {

        let mut strct = RustStruct {
            mapping: HashMap::new(),
            order: vec![],
        };
        let mapping = hash.try_convert_to::<rutie::Hash>().unwrap();
        mapping.each(|symbol, btype| {
            let mut btype = btype.to_owned();
            let symbol = symbol.try_convert_to::<rutie::Symbol>().unwrap();
            let to_type = into_rust::wrapper_to_rust_type(&mut btype);
            strct.mapping.insert(
                symbol.to_string(),
                to_type,
            );
            strct.order.push(symbol.to_string());
        });
        strct
    }
}

impl BareType for RustStruct {
    fn encode(&self, strct: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let strct = strct.try_convert_to::<rutie::Hash>()?;
        for key in self.order.iter() {
            let  bare_type = self.mapping.get(key).unwrap().clone();
            let symbol = rutie::Symbol::new(key.as_str());
            let struct_val = strct.at(&symbol);
            bare_type.encode(struct_val, bytes);
        }
        Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut hash = rutie::Hash::new();
        let mut bytes = bytes;
        for key in self.order.iter() {
            let  bare_type = self.mapping.get(key).unwrap().clone();
            let (new_bytes, value) = bare_type.decode(bytes);
            let symbol = rutie::Symbol::new(key);
            hash.store(symbol, value);
            bytes = new_bytes;
        };
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
