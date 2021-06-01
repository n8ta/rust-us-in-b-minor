use crate::{init, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::{AnyException, AnyObject, Class, Encoding, NilClass, Object, RString};
use std::rc::Rc;

const NAME: &str = "Rust_Void";

#[derive(Clone, Debug)]
pub struct RustVoid;

impl RustVoid {
    pub fn new() -> Self {
        RustVoid
    }
}

impl BareType for RustVoid {
    fn encode(&self, void: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        Ok(())
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        (bytes, NilClass::new().into())
    }
}

type RustVoidRc = Rc<RustVoid>;

wrappable_struct! {
    RustVoidRc,
    RustVoidRcWrap,
    RUST_VOID_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareVoid,
    RUST_VOID_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustVoid::new()), &*RUST_VOID_WRAP)
    }
);

init!(void_init, NAME);
