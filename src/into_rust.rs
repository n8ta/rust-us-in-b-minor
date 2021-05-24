use crate::float32::{RustFloat32, RustFloat32Wrap, RUST_FLOAT_32_WRAP};
use crate::float64::{RUST_FLOAT_64_WRAP, RustFloat64};
use crate::fixed_array::{RUST_FIXED_ARRAY_WRAP};
use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::BareType;
use rutie::rubysys::vm::ruby_init;
use crate::fixed_array::{BareFixedArray, RustFixedArray};


extern "C" {
    // PANICs if wrong type
    pub fn rb_check_typeddata(object: rutie::rubysys::value::Value,
                              ata_type: *const rutie::rubysys::typed_data::RbDataType) -> *mut rutie::rubysys::types::c_void;
    // 0 False 1 True if matching type
    pub fn rb_typeddata_is_kind_of(object: rutie::rubysys::value::Value,
                                   data_type: *const rutie::rubysys::typed_data::RbDataType) -> rutie::rubysys::types::c_int;
}

// Try to pull out the ptr

fn is_a(ancestor: &Class, known: &AnyObject) -> bool {
    ancestor.eq(known.class().ancestors().get(0).unwrap())
}

// macro_rules! test_type {
//     () => {
//         println!("testing...");
//     }
// }

pub fn into_rust(wrapped_rust_class: &mut AnyObject) -> Box<dyn BareType> {
    println!("Into rust");
    let f32_ruby = Class::from_existing("BareFloat32").wrap_data(RustFloat32::new(), &*RUST_FLOAT_32_WRAP);
    let f64_ruby = Class::from_existing("BareFloat64").wrap_data(RustFloat64::new(), &*RUST_FLOAT_64_WRAP);

    let rust_fixed_array = RustFixedArray::rust_new(123, Box::new(RustFloat32::new()));
    let fixed_ruby = Class::from_existing("BareFixedArray").wrap_data(rust_fixed_array, &*RUST_FIXED_ARRAY_WRAP);

    let cls = wrapped_rust_class.class();
    let mut ancestors = cls.ancestors();
    let ancestor = ancestors.get(0).unwrap();

    if is_a(ancestor, &f32_ruby) {
        let mut data = wrapped_rust_class.get_data_mut(&*RUST_FLOAT_32_WRAP);
        println!("Found a f32");
        return Box::new(data.clone());
    }
    if is_a(ancestor, &f64_ruby) {
        let mut data = wrapped_rust_class.get_data_mut(&*RUST_FLOAT_64_WRAP);
        return Box::new(data);
    }
    if is_a(ancestor, &fixed_ruby) {
        println!("Found a fixed array");
        let mut rust_fixed_arr = wrapped_rust_class.get_data_mut(&*RUST_FIXED_ARRAY_WRAP);
        let copy = rust_fixed_arr.clone();
        return Box::new(copy);
    }

    panic!("No bare type")
}