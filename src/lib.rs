#[macro_use]
extern crate rutie;
use rutie::{Class, Object, RString, VM, NilClass};

class!(RutieExample);
class!(HelloClass);

methods!(
    RutieExample,
    _rtself,

    fn pub_reverse(input: RString) -> RString {
        let ruby_string = input.
          map_err(|e| VM::raise_ex(e) ).
          unwrap();

        RString::new_utf8(
          &ruby_string.
          to_string().
          chars().
          rev().
          collect::<String>()
        )
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rutie_ruby_example() {
    Class::new("RutieExample", None).define(|klass| {
        klass.def_self("reverse", pub_reverse);
    });
}

methods!(
    HelloClass,
    _rtself,

    fn hello_world() -> NilClass {
        println!("Hello from rust!");
        NilClass::new()
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_hello_world() {
    Class::new("HelloClass", None).define(|klass| {
        klass.def_self("hello_world", hello_world);
    });
}