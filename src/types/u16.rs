use crate::{init, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::{AnyException, AnyObject, Class, Encoding, Integer, Object, RString};
use std::{convert::TryInto, rc::Rc};

const NAME: &str = "Rust_U16";

#[derive(Clone, Debug)]
pub struct RustU16;

impl RustU16 {
    pub fn new() -> Self {
        RustU16
    }
}

impl BareType for RustU16 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let num_bytes = (num.try_convert_to::<Integer>()?.to_u64() as u16).to_le_bytes();
        for byte in num_bytes.iter() {
            bytes.push(*byte);
        }
        return Ok(());
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut num_bytes: [u8; 2] = bytes[..2].try_into().unwrap();
        let num = u16::from_le_bytes(num_bytes);
        let number = Integer::new(num.into());
        return (&bytes[2..], number.into());
    }
}

type RustU16Rc = Rc<RustU16>;

wrappable_struct! {
    RustU16Rc,
    RustU16Wrap,
    RUST_U16_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareU16,
    RUST_U16_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustU16::new()), &*RUST_U16_WRAP)
    }
);

init!(u16_init, NAME);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enc() {
        let mut bytes = vec![];
        RustU16::new().encode(Integer::new(9000).into(), &mut bytes);
        assert_eq!(bytes, [0x28, 0x23]);

        let mut bytes = vec![];
        RustU16::new().encode(Integer::new(u16::MAX.into()).into(), &mut bytes);
        assert_eq!(bytes, [0xFF, 0xFF]);
    }

    #[test]
    fn test_dec() {
        let mut bytes = vec![0x28, 0x23];
        let (_, obj) = RustU16::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, 9000);

        let mut bytes = vec![0xFF, 0xFF];
        let (_, obj) = RustU16::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, u16::MAX as u64);
    }
}
