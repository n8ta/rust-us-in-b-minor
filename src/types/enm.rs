use super::uint::RustUint;
use crate::{init, into_rust, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::value::ValueType::Hash;
use rutie::{AnyException, AnyObject, Class, Encoding, Exception, Fixnum, Float, Object, RString};
use std::{collections::HashMap, rc::Rc};

const NAME: &str = "Rust_Enum";

#[derive(Clone /* Debug */)]
pub struct RustEnum {
    val_to_int: rutie::Hash,
    int_to_val: rutie::Hash,
}

impl RustEnum {
    pub fn new(hash: AnyObject) -> Self {
        let int_to_val = hash.try_convert_to::<rutie::Hash>().unwrap();
        let mut val_to_int = rutie::Hash::new();
        int_to_val.each(|key, value| {
            val_to_int.store(value, key);
        });
        RustEnum {
            int_to_val,
            val_to_int,
        }
    }
}

impl BareType for RustEnum {
    fn encode(&self, variant: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let uint_repr = self.val_to_int.at(&variant);
        if uint_repr.is_nil() {
            return Err(AnyException::new(
                "StandardError",
                Some("Uint representation not defined for enum value"),
            ));
        }

        RustUint.encode(uint_repr, bytes)?;
        Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (bytes, ruby_uint) = RustUint.decode(bytes);
        let value = self.int_to_val.at(&ruby_uint);

        if value.is_nil() {
            panic!("No enum variant exists for given uint: {:?}", ruby_uint);
        }

        (bytes, value)
    }
}

type RustEnumRc = Rc<RustEnum>;

wrappable_struct! {
    RustEnumRc,
    RustEnumRcWrap,
    RUST_ENUM_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareStruct,
    RUST_ENUM_WRAP,
    fn new(hash: AnyObject) {
        Class::from_existing(NAME)
            .wrap_data(Rc::new(RustEnum::new(hash.unwrap())), &*RUST_ENUM_WRAP)
    }
);

init!(enum_init, NAME);
