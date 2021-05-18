use rutie::{Class, NilClass, AnyObject, Object, Fixnum, GC, VM};
use lazy_static::lazy_static;
use std::ops::{Deref, DerefMut};

pub struct BareArray {
    inner: Vec<AnyObject>,
}

impl BareArray {
    fn new() -> Self {
        BareArray {
            inner: Vec::new(),
        }
    }
}

impl Deref for BareArray {
    type Target = Vec<AnyObject>;

    fn deref(&self) -> &Vec<AnyObject> {
        &self.inner
    }
}

impl DerefMut for BareArray {
fn deref_mut(&mut self) -> &mut Vec<AnyObject> {
    &mut self.inner
}
}

wrappable_struct! {
    BareArray,
    BareArrayWrapper,
    BARE_ARRAY_WRAPPER,

    // Mark each `AnyObject` element of the `inner` vector to prevent garbage collection.
    // `data` is a mutable reference to the wrapped data (`&mut VectorOfObjects`).
    mark(data) {
        for object in &data.inner {
            GC::mark(object);
        }
    }
}

class!(RustyArray);

methods! {
    RustyArray,
    rtself,

    fn new() -> AnyObject {
        let vec = BareArray::new();

        Class::from_existing("BareArray").wrap_data(vec, &*BARE_ARRAY_WRAPPER)
    }

    fn push(object: AnyObject) -> NilClass {
        rtself.get_data_mut(&*BARE_ARRAY_WRAPPER).push(object.unwrap());

        NilClass::new()
    }

    fn length() -> Fixnum {
        let length = rtself.get_data(&*BARE_ARRAY_WRAPPER).len() as i64;

        Fixnum::new(length)
    }
}
pub fn array_init() {
    let data_class = Class::from_existing("Object");
    Class::new("BareArray", Some(&data_class)).define(|klass| {
        klass.def_self("new", new);

        klass.def("push", push);
        klass.def("length", length);
    });
}