mod into_rust;
#[allow(unused_imports)]
#[allow(warnings)]
#[allow(dead_code)]
mod types;

#[macro_use]
extern crate rutie;

extern crate lazy_static;

use rutie::{AnyException, AnyObject};
use types::{
    array::array_init, bool::bool_init, data::data_init, data_fixed_len::fixed_data_init,
    enm::enum_init, fixed_array::fixed_array_init, float32::float32_init, float64::float64_init,
    i16::i16_init, i32::i32_init, i64::i64_init, i8::i8_init, int::int_init, map::map_init,
    optional::opt_init, strct::struct_init, u16::u16_init, u32::u32_init, u64::u64_init,
    u8::u8_init, uint::uint_init, union::union_init, void::void_init,
    rstring::rstring_init,
};

pub trait BareType {
    fn encode(&self, input: AnyObject, byte_output: &mut Vec<u8>) -> Result<(), AnyException>;
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject);
}

#[macro_export]
macro_rules! init {
    ($func_name:ident, $btype:expr) => {
        pub fn $func_name() {
            let data_class = Class::from_existing("Object");
            Class::new($btype, Some(&data_class)).define(|klass| {
                klass.def_self("new", new);
                klass.def("encode", encode);
                klass.def("decode", decode);
                klass.const_set("BTYPE", &RString::new_utf8($btype));
            });
        }
    };
}

#[macro_export]
macro_rules! ruby_methods {
    ($class_name:ident,
    $wrap:ident,
    fn new(  $($arg:ident : $argt:ty),* $(,)?   )
        $body:block
    ) => {
        class!($class_name);
        type RutieAny = ::rutie::AnyObject;
        type RutieRString = ::rutie::RString;
        methods! {
            $class_name,
            rtself,

            fn encode(input: ::rutie::AnyObject) -> RutieRString {
                let rust_class = rtself.get_data_mut(&*$wrap);
                let mut bytes: Vec<u8> = vec![];
                rust_class.encode(input.unwrap(), &mut bytes);
                ::rutie::RString::from_bytes(&mut bytes, &Encoding::us_ascii())
            }

            fn decode(to_decode: ::rutie::AnyObject) -> RutieAny {
                let safe = to_decode.unwrap().try_convert_to::<::rutie::RString>().unwrap();
                let bytes = safe.to_bytes_unchecked();
                let rfloat64 = rtself.get_data_mut(&*$wrap);
                let (_, decoded) = rfloat64.decode(bytes);
                return decoded
            }

            fn new(  $($arg: $argt),* ) -> RutieAny
                $body

        }
    }
}

#[macro_export]
macro_rules! ruby_methods_no_encode {
    ($class_name:ident,
    $wrap:ident,
    fn new(  $($arg:ident : $argt:ty),* $(,)?   )
        $body:block
    ) => {
        class!($class_name);
        type RutieAny = ::rutie::AnyObject;
        type RutieRString = ::rutie::RString;
        methods! {
            $class_name,
            rtself,

            fn decode(to_decode: ::rutie::AnyObject) -> RutieAny {
                let safe = to_decode.unwrap().try_convert_to::<::rutie::RString>().unwrap();
                let bytes = safe.to_bytes_unchecked();
                let rfloat64 = rtself.get_data_mut(&*$wrap);
                let (_, decoded) = rfloat64.decode(bytes);
                return decoded
            }

            fn new(  $($arg: $argt),* ) -> RutieAny
                $body

        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    fixed_array_init();
    float64_init();
    float32_init();
    uint_init();
    int_init();
    array_init();
    i8_init();
    i16_init();
    i32_init();
    i64_init();
    opt_init();
    fixed_data_init();
    u8_init();
    u16_init();
    u32_init();
    u64_init();
    union_init();
    struct_init();
    enum_init();
    bool_init();
    void_init();
    data_init();
    map_init();
    rstring_init();
}
