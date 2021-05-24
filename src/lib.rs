mod fixed_array;
mod float64;
mod float32;
mod into_rust;

#[allow(warnings)]
#[allow(dead_code)]

#[macro_use]
extern crate rutie;

extern crate lazy_static;

use float64::float64_init;
use fixed_array::fixed_array_init;
use rutie::{AnyObject, AnyException};
use float32::float32_init;



pub trait BareType: Clone {
    fn encode(&self, input: AnyObject, byte_output: &mut Vec<u8>) -> Result<(), AnyException>;
    fn decode<'a>(&self, bytes: &'a [u8]) -> (&'a [u8], AnyObject);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    fixed_array_init();
    float64_init();
    float32_init()
}