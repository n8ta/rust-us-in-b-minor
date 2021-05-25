use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException};
use lazy_static::lazy_static;
use crate::{BareType, init};
use std::rc::Rc;



#[derive(Clone, Debug)]
pub struct RustFloat64;

const NAME: &str = "F64";

type RustFloat64Rc = Rc<RustFloat64>;

impl RustFloat64 {
    pub fn new() -> Self {
        RustFloat64
    }
}

impl BareType for RustFloat64 {
    fn encode(&self, fl: AnyObject, bytes: &mut Vec<u8>) -> Result<(),AnyException> {
        let fl = fl.try_convert_to::<Float>()?;
        let f64_bytes = fl.to_f64().to_le_bytes();
        for idx in 0..8 {
            bytes.push(f64_bytes[idx]);
        }
        return Ok(());
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        println!("float64 decoding...");
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

class!(BareFloat64);

methods! {
    BareFloat64,
    rtself,

    fn new() -> AnyObject {
        let cls = Rc::new(RustFloat64::new());
        Class::from_existing("BareFloat64").wrap_data(cls, &*RUST_FLOAT_64_WRAP)
    }

    // BareFixedArray(BareFloat32)
    // BareFixedArray(Union(BareFloat32, Uint))
    fn encode(input: AnyObject) -> RString {
        let rfloat64 = rtself.get_data_mut(&*RUST_FLOAT_64_WRAP);
        let mut bytes: Vec<u8> = vec![];
        rfloat64.encode(input.unwrap(), &mut bytes);
        RString::from_bytes(&mut bytes, &Encoding::us_ascii())
    }

    fn decode(to_decode: AnyObject) -> AnyObject {
        let safe = to_decode.unwrap().try_convert_to::<RString>().unwrap();
        let bytes = safe.to_bytes_unchecked();
        let rfloat64 = rtself.get_data_mut(&*RUST_FLOAT_64_WRAP);
        let (_, float) = rfloat64.decode(bytes);
        return float
    }

}

init!(float64_init, NAME);