use crate::types::array::RUST_ARRAY_WRAP;
use crate::types::fixed_array::RUST_FIXED_ARRAY_WRAP;
use crate::types::float32::RUST_FLOAT_32_WRAP;
use crate::types::float64::RUST_FLOAT_64_WRAP;
use crate::types::int::RUST_INT_WRAP;
use crate::types::uint::RUST_UINT_WRAP;

use crate::types::data_fixed_len::RUST_FIXED_DATA_WRAP;
use crate::types::i16::RUST_I16_WRAP;
use crate::types::i32::RUST_I32_WRAP;
use crate::types::i64::RUST_I64_WRAP;
use crate::types::i8::RUST_I8_WRAP;
use crate::types::optional::RUST_OPT_WRAP;
use crate::types::u16::RUST_U16_WRAP;
use crate::types::u32::RUST_U32_WRAP;
use crate::types::u64::RUST_U64_WRAP;
use crate::types::u8::RUST_U8_WRAP;
use crate::types::union::RUST_UNION_WRAP;
use crate::types::strct::RUST_STRUCT_WRAP;
use crate::types::enm::RUST_ENUM_WRAP;

use crate::BareType;
use rutie::{AnyObject, Object, RString};

use std::rc::Rc;

pub fn wrapper_to_rust_type(wrapped_rust_class: &mut AnyObject) -> Rc<dyn BareType> {
    let val = wrapped_rust_class
        .class()
        .const_get("BTYPE")
        .try_convert_to::<RString>()
        .unwrap();
    let btype = val.to_str();
    match btype {
        "Rust_F32" => wrapped_rust_class.get_data_mut(&*RUST_FLOAT_32_WRAP).clone(),
        "Rust_F64" => wrapped_rust_class.get_data_mut(&*RUST_FLOAT_64_WRAP).clone(),
        "Rust_ArrayFixedLen" => wrapped_rust_class.get_data_mut(&*RUST_FIXED_ARRAY_WRAP).clone(),
        "Rust_Uint" => wrapped_rust_class.get_data_mut(&*RUST_UINT_WRAP).clone(),
        "Rust_Int" => wrapped_rust_class.get_data_mut(&*RUST_INT_WRAP).clone(),
        "Rust_Array" => wrapped_rust_class.get_data_mut(&*RUST_ARRAY_WRAP).clone(),
        "Rust_Union" => wrapped_rust_class.get_data_mut(&*RUST_UNION_WRAP).clone(),
        "Rust_Struct" => wrapped_rust_class.get_data_mut(&*RUST_STRUCT_WRAP).clone(),
        "Rust_Enum" => wrapped_rust_class.get_data_mut(&*RUST_ENUM_WRAP).clone(),
        "Rust_I8" => wrapped_rust_class.get_data_mut(&*RUST_I8_WRAP).clone(),
        "Rust_I16" => wrapped_rust_class.get_data_mut(&*RUST_I16_WRAP).clone(),
        "Rust_I32" => wrapped_rust_class.get_data_mut(&*RUST_I32_WRAP).clone(),
        "Rust_I64" => wrapped_rust_class.get_data_mut(&*RUST_I64_WRAP).clone(),
        "Rust_U8" => wrapped_rust_class.get_data_mut(&*RUST_U8_WRAP).clone(),
        "Rust_U16" => wrapped_rust_class.get_data_mut(&*RUST_U16_WRAP).clone(),
        "Rust_U32" => wrapped_rust_class.get_data_mut(&*RUST_U32_WRAP).clone(),
        "Rust_U64" => wrapped_rust_class.get_data_mut(&*RUST_U64_WRAP).clone(),
        "Rust_Opt" => wrapped_rust_class.get_data_mut(&*RUST_OPT_WRAP).clone(),
        "Rust_DataFixedLen" => wrapped_rust_class.get_data_mut(&*RUST_FIXED_DATA_WRAP).clone(),
        _ => panic!("That's not a bare type! {}", btype),
    }
}
