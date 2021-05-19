mod array;
mod float64;

#[macro_use]
extern crate rutie;

#[macro_use]
extern crate lazy_static;

use array::array_init;
use float64::float64_init;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    array_init();
    float64_init();
}