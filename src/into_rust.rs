use crate::types::float32::{RUST_FLOAT_32_WRAP};
use crate::types::float64::{RUST_FLOAT_64_WRAP};
use crate::types::fixed_array::{RUST_FIXED_ARRAY_WRAP};

use rutie::{AnyObject, Object, RString};
use crate::BareType;

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