// use rutie::{Class, AnyObject, Object, RString, Encoding, Fixnum, Array, AnyException};
// use lazy_static::lazy_static;
// use crate::{BareType, init, ruby_methods};
// use Box;
//
// use crate::into_rust::wrapper_to_rust_type;
// use std::rc::Rc;
//
// pub struct RustArray {
//     array_type: Rc<dyn BareType>,
// }
//
// const NAME: &str = "Rust_Array";
//
// type RustArrayRC = Rc<RustArray>;
//
// wrappable_struct! {
//     RustArrayRC,
//     RustArrayWrap,
//     RUST_ARRAY_WRAP,
//     mark(data) {}
// }
//
// impl RustArray {
//     pub fn new(typ: AnyObject) -> Self {
//         let mut typ = typ.clone();
//         let ret = RustArray {
//             array_type: wrapper_to_rust_type(&mut typ)
//         };
//         ret
//     }
// }
//
// impl BareType for RustArray {
//     fn encode(&self, input: AnyObject, bytes: &mut Vec<u8>) -> std::result::Result<(),AnyException> {
//         let array = input.try_convert_to::<Array>().unwrap();
//         for idx in 0..self.len {
//             self.array_type.encode(array.at(idx), bytes);
//         }
//         Result::Ok(())
//     }
//     fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
//         let mut array = Array::new();
//         let mut bytes = bytes;
//         for _ in 0..self.len {
//             let (remaining_bytes, decoded) = self.array_type.decode(bytes);
//             bytes = remaining_bytes;
//             array.push(decoded);
//         }
//         (bytes, array.into())
//     }
// }
//
// ruby_methods!(
//     ArrayFixedLen,
//     RUST_ARRAY_WRAP,
//     fn new(typ: AnyObject,) {
//         let fixed_array = Rc::new(typ.unwrap());
//         let ret = Class::from_existing(NAME).wrap_data(fixed_array, &*RUST_ARRAY_WRAP);
//         ret
//     }
// );
//
//
// init!(array_init, NAME);