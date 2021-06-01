use crate::{init, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::{AnyException, AnyObject, Boolean, Class, Encoding, Object, RString};
use std::rc::Rc;

const NAME: &str = "Rust_Bool";

#[derive(Clone, Debug)]
pub struct RustBool;

impl RustBool {
    pub fn new() -> Self {
        RustBool
    }
}

impl BareType for RustBool {
    fn encode(&self, boolean: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let boolean = boolean.try_convert_to::<Boolean>()?.to_bool();
        bytes.push(boolean as u8);
        Ok(())
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let val = match bytes[0] {
            0 => false,
            1 => true,
            other => panic!("Invalid value for boolean: {}", other),
        };

        let num = Boolean::new(val.into());
        return (&bytes[1..], num.into());
    }
}

type RustBoolRc = Rc<RustBool>;

wrappable_struct! {
    RustBoolRc,
    RustBoolRcWrap,
    RUST_BOOL_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareBool,
    RUST_BOOL_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustBool::new()), &*RUST_BOOL_WRAP)
    }
);

init!(bool_init, NAME);
