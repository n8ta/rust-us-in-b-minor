use rutie::{Class, AnyObject, Object, Float, RString, Encoding, AnyException, Fixnum};
use lazy_static::lazy_static;
use crate::{BareType, init, ruby_methods};
use std::rc::Rc;
use rutie::rubysys::value::RubySpecialFlags::FixnumFlag;
use crate::into_rust::wrapper_to_rust_type;
use crate::types::uint::RustUint;


#[derive(Clone)]
pub struct RustMap {
    to: Rc<dyn BareType>,
    from: Rc<dyn BareType>,
}


const NAME: &str = "Rust_Map";


impl RustMap {
    pub fn new(mut from: AnyObject, mut to: AnyObject) -> Self {
        RustMap {
            to: wrapper_to_rust_type(&mut to),
            from: wrapper_to_rust_type(&mut from),
        }
    }
}

impl BareType for RustMap {
    fn encode(&self, map: AnyObject, bytes: &mut Vec<u8>) -> Result<(), AnyException> {
        let map = map.try_convert_to::<rutie::Hash>()?;
        let size = map.length();
        RustUint.encode(rutie::Integer::from(size as u64).into(), bytes);
        let mut exception = Ok(());
        map.each(|key, value| {
            if let Err(e) = self.from.encode(key, bytes) {
                exception = Err(e);
            }
            if let Err(e) = self.to.encode(value, bytes) {
                exception = Err(e);
            }
        });
        exception
    }
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject) {
        let (mut bytes, count) = RustUint.decode(bytes);
        let count = count.try_convert_to::<rutie::Integer>().unwrap().to_u64() as usize;
        let mut map = rutie::Hash::new();
        for _ in 0..count {
            let (new_bytes, key) = self.from.decode(bytes);
            let (new_bytes, value) = self.to.decode(new_bytes);
            map.store(key, value);
            bytes = new_bytes;
        }
        (bytes, map.into())
    }
}

type RustMapRc = Rc<RustMap>;

wrappable_struct! {
    RustMapRc,
    RustMapRcWrap,
    RUST_MAP_WRAP,
    mark(data) {}
}

ruby_methods!(
    BareMap,
    RUST_MAP_WRAP,
    fn new(to: AnyObject, from: AnyObject) {
        Class::from_existing(NAME).wrap_data(Rc::new(RustMap::new(to.unwrap(), from.unwrap())), &*RUST_MAP_WRAP)
    }
);

init!(map_init, NAME);

#[cfg(test)]
#[test]
fn encode_test_map() {
    // [{ 8 => 16, 5 => 10 }, "\x02\x08\x10\x00\x05\x0A\x00".b, Bare.Map(Bare.U8, Bare.U16)]
    use crate::types::u8::RustU8;
    use crate::types::u16::RustU16;
    rutie::VM::init();
    let mut hash = rutie::Hash::new();
    hash.store(rutie::Integer::new(8),
               rutie::Integer::new(16));
    hash.store(rutie::Integer::new(5),
               rutie::Integer::new(10));
    let map = RustMap {
        from: Rc::new(RustU8::new()),
        to: Rc::new(RustU16::new()),
    };
    let mut bytes = vec![];
    map.encode(hash.into(), &mut bytes);
    assert_eq!(bytes, vec![0x02, 0x08, 0x10, 0x00, 0x05, 0x0A, 0x00]);

    let (bytes, result) = map.decode(bytes.as_slice());

    let from = rutie::Integer::new(8);
    let res = result.try_convert_to::<rutie::Hash>().unwrap().at(&from);
    let res = res.try_convert_to::<rutie::Integer>().unwrap();
    assert_eq!(res.to_i64(), 16);

    let from2 = rutie::Integer::new(5);
    let res2 = result.try_convert_to::<rutie::Hash>().unwrap().at(&from2);
    let res2 = res2.try_convert_to::<rutie::Integer>().unwrap();
    assert_eq!(res2.to_i64(), 10);
}
