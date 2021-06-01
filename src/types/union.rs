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
    mapping: HashMap<usize, AnyObject>,
}

impl RustUnion {
    pub fn new(types: AnyObject) -> Self {
        let types = types.try_convert_to::<rutie::Hash>().unwrap();
        let mut mapping = HashMap::new();
        types.each(|key, mut typ| {
            let key = key.try_convert_to::<Integer>().unwrap().to_u64() as usize;
            mapping.insert(key, typ.clone());
        });
        RustUnion { mapping }
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
                Some("Type must be specified for union."),
            ));
        }
        // find the uint for this type
        let type_key = self
            .mapping
            .iter()
            .find_map(|(key, val)| (val.class() == typ.class()).then(|| *key))
            .ok_or_else(|| {
                AnyException::new(
                    "StandardError",
                    Some("Type found not specified by union definition."),
                )
            })?;
        // encode
        let bare_type = into_rust::wrapper_to_rust_type(&mut typ);
        RustUint.encode(Integer::from(type_key as u64).into(), bytes);
        bare_type.encode(value, bytes);
        Ok(())
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (bytes, type_key) = RustUint.decode(bytes);
        let type_key = type_key.try_convert_to::<Integer>().unwrap().to_u64() as usize;
        let mut typ = self
            .mapping
            .get(&type_key)
            .unwrap_or_else(|| panic!("Type key not found in union definition: {}", type_key))
            .clone();

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
