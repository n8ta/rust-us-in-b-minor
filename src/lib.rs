mod float64;
mod fixed_array;

#[macro_use]
extern crate rutie;

#[macro_use]
extern crate lazy_static;

use float64::float64_init;
use fixed_array::fixed_array_init;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn bare_init() {
    fixed_array_init();
    float64_init();
}