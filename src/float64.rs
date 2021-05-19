use rutie::{Class, AnyObject, Object, Float, RString, Encoding};
use lazy_static::lazy_static;

pub struct RustFloat64 {}

impl RustFloat64 {
    pub fn new() -> Self {
        RustFloat64 {}
    }
    pub fn encode(&self, fl: AnyObject, bytes: &mut Vec<u8>) {
        let fl = fl.try_convert_to::<Float>().unwrap();
        let f64_bytes = fl.to_f64().to_le_bytes();
        for idx in 0..8 {
            bytes.push(f64_bytes[idx]);
        }
    }
    pub fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], Float) {
        let float_bs: [u8; 8] = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]];
        return (&bytes[8..], Float::new(f64::from_le_bytes(float_bs)))
    }
}

wrappable_struct! {
    RustFloat64,
    RustFloat64Wrap,
    RUST_FLOAT_64_WRAP,

    mark(data) {
        // GC::mark(&data.val)
    }
}

class!(BareFloat64);

methods! {
    BareFloat64,
    rtself,

    fn new() -> AnyObject {
        let cls = RustFloat64::new();
        Class::from_existing("BareFloat64").wrap_data(cls, &*RUST_FLOAT_64_WRAP)
    }

    fn encode(input: AnyObject) -> RString {
        let rfloat64 = rtself.get_data_mut(&*RUST_FLOAT_64_WRAP);
        let mut bytes: Vec<u8> = vec![];
        rfloat64.encode(input.unwrap(), &mut bytes);
        RString::from_bytes(&mut bytes, &Encoding::us_ascii())
    }

    fn decode(to_decode: AnyObject) -> Float {
        let safe = to_decode.unwrap().try_convert_to::<RString>().unwrap();
        let bytes = safe.to_bytes_unchecked();
        let rfloat64 = rtself.get_data_mut(&*RUST_FLOAT_64_WRAP);
        let (_, float) = rfloat64.decode(bytes);
        return float
    }

}
pub fn float64_init() {
    let data_class = Class::from_existing("Object");
    Class::new("BareFloat64", Some(&data_class)).define(|klass| {
        klass.def_self("new", new);
        klass.def("encode", encode);
        klass.def("decode", decode);
    });
}