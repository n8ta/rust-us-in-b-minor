use crate::{init, ruby_methods, BareType};
use lazy_static::lazy_static;
use rutie::rubysys::fixnum::rb_ull2inum;
use rutie::{AnyException, AnyObject, Class, Encoding, Integer, Object, RString};
use std::{
    convert::{TryFrom, TryInto},
    rc::Rc,
};

const NAME: &str = "Rust_U64";

#[derive(Clone, Debug)]
pub struct RustU64;

impl RustU64 {
    pub fn new() -> Self {
        RustU64
    }
}

impl BareType for RustU64 {
    fn encode(&self, num: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let num_bytes = (num.try_convert_to::<Integer>()?.to_u64()).to_le_bytes();
        for byte in num_bytes.iter() {
            bytes.push(*byte);
        }
        return Ok(());
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let mut num_bytes: [u8; 8] = bytes[..8].try_into().unwrap();
        let num = i64::try_from(u64::from_le_bytes(num_bytes))
            .expect("Full u64 bit numbers are currently not supported due to a dependency issue.");
        let number = Integer::new(num);
        return (&bytes[8..], number.into());
    }
}

type RustU64Rc = Rc<RustU64>;

wrappable_struct! {
    RustU64Rc,
    RustU64Wrap,
    RUST_U64_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareU64,
    RUST_U64_WRAP,
    fn new() {
        Class::from_existing(NAME).wrap_data(Rc::new(RustU64::new()), &*RUST_U64_WRAP)
    }
);

init!(u64_init, NAME);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enc() {
        let mut bytes = vec![];
        RustU64::new().encode(Integer::new(9000).into(), &mut bytes);
        assert_eq!(bytes, [0x28, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let mut bytes = vec![];
        RustU64::new().encode(Integer::new(9876543210).into(), &mut bytes);
        assert_eq!(bytes, [0xEA, 0x16, 0xB0, 0x4C, 0x02, 0x00, 0x00, 0x00]);

        // Rutie can't decode u64s that use the full 64 bits.
        // let mut bytes = vec![];
        // let num = Integer::from(u64::MAX);
        // RustU64::new().encode(num.into(), &mut bytes);
        // assert_eq!(bytes, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_dec() {
        let mut bytes = vec![0x28, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let (_, obj) = RustU64::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, 9000);

        let mut bytes = vec![0xEA, 0x16, 0xB0, 0x4C, 0x02, 0x00, 0x00, 0x00];
        let (_, obj) = RustU64::new().decode(&mut bytes);
        let obj = obj.try_convert_to::<Integer>().unwrap().to_u64();
        assert_eq!(obj, 9876543210);
    }
}
