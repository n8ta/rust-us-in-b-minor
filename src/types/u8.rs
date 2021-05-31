use crate::{init, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::{AnyException, AnyObject, Class, Encoding, Integer, Object, RString};
use std::rc::Rc;

const NAME: &str = "Rust_U8";

#[derive(Clone, Debug)]
pub struct RustU8;

impl RustU8 {
    pub fn new() -> Self {
        RustU8
    }
}

impl BareType for RustU8 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let num = num.try_convert_to::<Integer>()?.to_u64() as u8;
        bytes.push(num);
        return Ok(());
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let val = bytes[0] as u8;
        let num = Integer::new(val.into());
        return (&bytes[1..], num.into());
    }
}

type RustU8Rc = Rc<RustU8>;

wrappable_struct! {
    RustU8Rc,
    RustU8Wrap,
    RUST_U8_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareU8,
    RUST_U8_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustU8::new()), &*RUST_U8_WRAP)
    }
);

init!(u8_init, NAME);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enc() {
        let mut bytes = vec![];
        RustU8::new().encode(Integer::from(2).into(), &mut bytes);
        assert_eq!(bytes, [0x02]);

        let mut bytes = vec![];
        RustU8::new().encode(Integer::new(u8::MAX.into()).into(), &mut bytes);
        assert_eq!(bytes, [0xFF]);
    }

    #[test]
    fn test_dec() {
        let mut bytes = vec![0x02];
        let (_, obj) = RustU8::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, 2);

        let mut bytes = vec![0xFF];
        let (_, obj) = RustU8::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, u8::MAX as u64);
    }
}
