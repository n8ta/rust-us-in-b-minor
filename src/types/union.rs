use super::uint::RustUint;
use crate::{init, into_rust, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::value::ValueType::Hash;
use rutie::{
    AnyException, AnyObject, Class, Encoding, Exception, Float, Integer, Object, RString, Symbol,
};
use std::{any::Any, collections::HashMap, rc::Rc};

const NAME: &str = "Rust_Union";

#[derive(Clone /* Debug */)]
pub struct RustUnion {
    int_to_type: rutie::Hash,
    type_to_int: rutie::Hash,
}

impl RustUnion {
    pub fn new(types: AnyObject) -> Self {
        let int_to_type = types.try_convert_to::<rutie::Hash>().unwrap();
        let mut type_to_int = rutie::Hash::new();
        int_to_type.each(|int, typ| {
            type_to_int.store(typ.class(), int);
        });
        RustUnion {
            int_to_type,
            type_to_int,
        }
    }
}

impl BareType for RustUnion {
    fn encode(&self, hash: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let hash = hash.try_convert_to::<rutie::Hash>()?;
        let mut typ = hash.at(&Symbol::new("type"));
        let value = hash.at(&Symbol::new("value"));
        // validate presence of type. Value can be nil (void type)
        if typ.is_nil() {
            return Err(AnyException::new(
                "StandardError",
                Some("Type was not specified for union value."),
            ));
        }
        // find the uint for this type
        let type_key = self
            .type_to_int
            .at(&typ.class())
            .try_convert_to::<Integer>()?;
        let bare_type = into_rust::wrapper_to_rust_type(&mut typ);
        RustUint.encode(type_key.into(), bytes);
        bare_type.encode(value, bytes);
        Ok(())
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (bytes, type_key) = RustUint.decode(bytes);
        let type_key = type_key.try_convert_to::<Integer>().unwrap();
        let mut typ = self.int_to_type.at(&type_key);

        if typ.is_nil() {
            panic!("Invalid type specified for union: {}", type_key.to_u64());
        }

        let bare_type = into_rust::wrapper_to_rust_type(&mut typ);
        let (bytes, value) = bare_type.decode(bytes);
        let mut hash = rutie::Hash::new();
        hash.store(Symbol::new("type"), typ.class());
        // Ruby hashes are implicitly nil.
        if !value.is_nil() {
            hash.store(Symbol::new("value"), value);
        }
        (bytes, hash.into())
    }
}

type RustUnionRc = Rc<RustUnion>;

wrappable_struct! {
    RustUnionRc,
    RustUnionRcWrap,
    RUST_UNION_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareUnion,
    RUST_UNION_WRAP,
    fn new(hash: AnyObject) {
        Class::from_existing(NAME)
            .wrap_data(Rc::new(RustUnion::new(hash.unwrap())), &*RUST_UNION_WRAP)
    }
);

init!(union_init, NAME);
