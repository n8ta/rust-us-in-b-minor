use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;
use rutie::rubysys::value::RubySpecialFlags::FixnumFlag;
use crate::into_rust::wrapper_to_rust_type;

const NAME: &str = "Rust_Opt";

#[derive(Clone)]
pub struct RustOpt {
    typ: Rc<dyn BareType>
}


impl RustOpt {
    pub fn new(mut typ: AnyObject) -> Self {
        RustOpt {
            typ: wrapper_to_rust_type(&mut typ)
        }
    }
}

impl BareType for RustOpt {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        if num.is_nil() {
            bytes.push(0x00);
        } else {
            bytes.push(0x01);
            self.typ.encode(num, bytes)?;
        }
        Result::Ok(())
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let byte = bytes[0] as i8;
        if byte == 0 {
            (&bytes[1..], ::rutie::NilClass::new().into())
        } else {
            self.typ.decode(&bytes[1..])
        }
    }
}

type RustOptRc = Rc<RustOpt>;

wrappable_struct! {
    RustOptRc,
    RustOptRcWrap,
    RUST_OPT_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareOpt,
    RUST_OPT_WRAP,
    fn new(typ: AnyObject) {
        Class::from_existing(NAME).wrap_data(Rc::new(RustOpt::new(typ.unwrap())), &*RUST_OPT_WRAP)
    }
);

init!(opt_init, NAME);