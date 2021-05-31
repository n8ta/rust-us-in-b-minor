use crate::{init, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::{AnyException, AnyObject, Class, Encoding, Integer, Object, RString};
use std::{convert::TryInto, rc::Rc};

const NAME: &str = "Rust_U32";

#[derive(Clone, Debug)]
pub struct RustU32;

impl RustU32 {
    pub fn new() -> Self {
        RustU32
    }
}

impl BareType for RustU32 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let num_bytes = (num.try_convert_to::<Integer>()?.to_u64() as u32).to_le_bytes();
        for byte in num_bytes.iter() {
            bytes.push(*byte);
        }
        return Ok(());
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut num_bytes: [u8; 4] = bytes[..4].try_into().unwrap();
        let num = u32::from_le_bytes(num_bytes);
        let number = Integer::new(num.into());
        return (&bytes[4..], number.into());
    }
}

type RustU32Rc = Rc<RustU32>;

wrappable_struct! {
    RustU32Rc,
    RustU32Wrap,
    RUST_U32_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareU32,
    RUST_U32_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustU32::new()), &*RUST_U32_WRAP)
    }
);

init!(u32_init, NAME);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enc() {
        let mut bytes = vec![];
        RustU32::new().encode(Integer::new(9000).into(), &mut bytes);
        assert_eq!(bytes, [0x28, 0x23, 0x00, 0x00]);

        let mut bytes = vec![];
        RustU32::new().encode(Integer::new(128_000).into(), &mut bytes);
        assert_eq!(bytes, [0x00, 0xF4, 0x01, 0x00]);

        let mut bytes = vec![];
        RustU32::new().encode(Integer::new(u32::MAX.into()).into(), &mut bytes);
        assert_eq!(bytes, [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_dec() {
        let mut bytes = vec![0x28, 0x23, 0x00, 0x00];
        let (_, obj) = RustU32::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, 9000);

        let mut bytes = vec![0x00, 0xF4, 0x01, 0x00];
        let (_, obj) = RustU32::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, 128_000);

        let mut bytes = vec![0xFF, 0xFF, 0xFF, 0xFF];
        let (_, obj) = RustU32::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, u32::MAX as u64);
    }
}
