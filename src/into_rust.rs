use crate::float32::{RustFloat32, RustFloat32Wrap, RUST_FLOAT_32_WRAP};
use crate::float64::{RUST_FLOAT_64_WRAP, RustFloat64};
use crate::fixed_array::{RUST_FIXED_ARRAY_WRAP};
use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::BareType;
use rutie::rubysys::vm::ruby_init;
use crate::fixed_array::{ArrayFixedLen, RustFixedArray};
use std::rc::Rc;

pub fn wrapper_to_rust_type(wrapped_rust_class: &mut AnyObject) -> Rc<dyn BareType> {
    let val = wrapped_rust_class.class().const_get("BTYPE").try_convert_to::<RString>().unwrap();
    let btype = val.to_str();
    match btype {
        "Rust_F32" => wrapped_rust_class.get_data_mut(&*RUST_FLOAT_32_WRAP).clone(),
        "Rust_F64" => wrapped_rust_class.get_data_mut(&*RUST_FLOAT_64_WRAP).clone(),
        "Rust_ArrayFixedLen" => wrapped_rust_class.get_data_mut(&*RUST_FIXED_ARRAY_WRAP).clone(),
        _ => panic!("That's not a bare type! {}", btype)
    }
}