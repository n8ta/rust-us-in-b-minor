#[macro_use]
extern crate rutie;

use rutie::{Class, NilClass, AnyObject, Array, Fixnum, VM};
use crate::rutie::Object;

class!(BareRustInterface);
methods!(
    BareRustInterface,
    rtself,
    fn encode(arr_type: AnyObject, length: AnyObject) -> NilClass {
        let length = length.unwrap()
            .try_convert_to::<Fixnum>().expect("Unable to convert len into a number")
            .to_i64();
        let arr_type = arr_type.unwrap();
//        let arr_type = arr_type
//            .try_convert_to::<Class>()
//            .expect("Should be a class");
        // let arr_type_enum = arr_type.const_get("TYPE").try_convert_to::<Fixnum>().expect("TYPE must be present")
        // println!("Arr_type_num: {}", arr_type_enum.to_i64());

        println!("Hello from rust testing! {} {}", length, 123);
        NilClass::new()
    }
);


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    Class::new("BareRustInterface", None).define(|klass| {
        klass.def_self("encode", encode);
    });
}
