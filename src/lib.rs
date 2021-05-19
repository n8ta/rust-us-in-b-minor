mod float64;
mod fixed_array;

#[macro_use]
extern crate rutie;

#[macro_use]
extern crate lazy_static;

use float64::float64_init;
use fixed_array::fixed_array_init;
use rutie::{AnyObject, AnyException};

pub trait BareType {
    type RubyType;
    fn encode(&self, input: AnyObject, byte_output: &mut Vec<u8>) -> Result<(), AnyException>;
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], Self::RubyType);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    fixed_array_init();
    float64_init();
}