use rutie::{Class, AnyObject, Object, GC, Float, RString, Encoding};
use lazy_static::lazy_static;

pub struct RustFloat64 {
    val: Float,
}

impl RustFloat64 {
    fn new(val: Float) -> Self {
        RustFloat64 {
            val
        }

    }
}

wrappable_struct! {
    RustFloat64,
    RustFloat64Wrap,
    RUST_FLOAT_64_WRAP,

    mark(data) {
        GC::mark(&data.val)
    }
}

class!(BareFloat64);

methods! {
    BareFloat64,
    rtself,

    fn new(input: Float) -> AnyObject {
        let num = input.unwrap();
        let vec = RustFloat64::new(num);
        Class::from_existing("BareFloat64").wrap_data(vec, &*RUST_FLOAT_64_WRAP)
    }

    fn encode() -> RString {
        let rfloat64 = rtself.get_data_mut(&*RUST_FLOAT_64_WRAP);
        RString::from_bytes(&rfloat64.val.to_f64().to_be_bytes(), &Encoding::us_ascii())
    }

    fn decode(to_decode: RString) -> Float {
        let safe = to_decode.unwrap();
        let u = safe.to_bytes_unchecked();
        let bytes: [u8; 8] = [u[0], u[1], u[2], u[3], u[4], u[5], u[6], u[7]];
        Float::new(f64::from_le_bytes(bytes))
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