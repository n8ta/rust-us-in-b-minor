use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;

const NAME: &str = "Rust_F32";

#[derive(Clone, Debug)]
pub struct RustFloat32;


impl RustFloat32 {
    pub fn new() -> Self {
        RustFloat32
    }
}

impl BareType for RustFloat32 {
    fn encode(&self, fl: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let int = fl.try_convert_to::<Fixnum>();
        let fl = fl.try_convert_to::<Float>();  // ?.to_f64();
        let float;
        if fl.is_ok() {
            float = fl.unwrap().to_f64();
        } else {
            float = int?.to_i64() as f64;
        }
        let fl32 = float as f32;
        let fl32_bytes = fl32.to_le_bytes();
        for idx in 0..4 {
            bytes.push(fl32_bytes[idx]);
        }
        return Ok(());
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut float_bs: [u8; 4] = [0; 4];
        for (float_bs_ref, value) in float_bs.iter_mut().zip(bytes) {
            *float_bs_ref = *value;
        }
        let float = Float::new(f32::from_le_bytes(float_bs) as f64);
        return (&bytes[4..], float.into());
    }
}

type RustFloat32Rc = Rc<RustFloat32>;

wrappable_struct! {
    RustFloat32Rc,
    RustFloat32Wrap,
    RUST_FLOAT_32_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareFloat64,
    RUST_FLOAT_32_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustFloat32::new()), &*RUST_FLOAT_32_WRAP)
    }
);

init!(float32_init, NAME);