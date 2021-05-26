#[allow(unused_imports)]
#[allow(warnings)]
#[allow(dead_code)]
mod fixed_array;
#[allow(unused_imports)]
#[allow(warnings)]
#[allow(dead_code)]
mod float64;
#[allow(unused_imports)]
#[allow(warnings)]
#[allow(dead_code)]
mod float32;
#[allow(unused_imports)]
#[allow(warnings)]
#[allow(dead_code)]
mod into_rust;


#[macro_use]
extern crate rutie;

extern crate lazy_static;

use float64::float64_init;
use fixed_array::fixed_array_init;
use rutie::{AnyObject, AnyException};
use float32::float32_init;


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
    }
}


/// Defines the ruby accessible encode and decode functions
/// will also define .new() provided your new function doesn't take any args
/// if new() does take args you'll have to call methods! yourself.
/// see fixed_array.rs for an example.
#[macro_export]
macro_rules! ruby_methods {
    // Case with a 0 arg new function
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
                let rfloat64 = rtself.get_data_mut(&*$wrap);
                let mut bytes: Vec<u8> = vec![];
                rfloat64.encode(input.unwrap(), &mut bytes);
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    fixed_array_init();
    float64_init();
    float32_init()
}
