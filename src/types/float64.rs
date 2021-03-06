use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;



#[derive(Clone, Debug)]
pub struct RustFloat64;

const NAME: &str = "Rust_F64";

type RustFloat64Rc = Rc<RustFloat64>;

impl RustFloat64 {
    pub fn new() -> Self {
        RustFloat64
    }
}

impl BareType for RustFloat64 {
    fn encode(&self, fl: AnyObject, bytes: &mut Vec<u8>) -> Result<(),AnyException> {
        let int = fl.try_convert_to::<Fixnum>();
        let fl = fl.try_convert_to::<Float>();
        let float;
        if fl.is_ok() {
            float = fl.unwrap().to_f64();
        } else {
            float = int?.to_i64() as f64;
        }
        let f64_bytes = float.to_le_bytes();
        for idx in 0..8 {
            bytes.push(f64_bytes[idx]);
        }
        return Ok(());
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut float_bs: [u8; 8] = [0; 8];
        for (float_bs_ref, value) in float_bs.iter_mut().zip(bytes) {
            *float_bs_ref = *value;
        }
        let float = Float::new(f64::from_le_bytes(float_bs));
        return (&bytes[8..], float.into())
    }
}

wrappable_struct! {
    RustFloat64Rc,
    RustFloat64Wrap,
    RUST_FLOAT_64_WRAP,
    mark(data) {}
}

ruby_methods!(
    F64,
    RUST_FLOAT_64_WRAP,
    fn new() {
        let cls = Rc::new(RustFloat64::new());
        Class::from_existing(NAME).wrap_data(cls, &*RUST_FLOAT_64_WRAP)
    }
);

init!(float64_init, NAME);