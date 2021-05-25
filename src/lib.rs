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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    fixed_array_init();
    float64_init();
    float32_init()
}
