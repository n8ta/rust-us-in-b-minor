use libc;

#[no_mangle]
pub extern "C" fn hello_world(x: i32) {
    println!("Hello from rust {}", x);
}

#[no_mangle]
pub extern "C" fn sum_array(
    size: libc::size_t,
    array: *const i32
) -> i32 {
    let mut sum: i32 = 0;
    unsafe {
        for n in 0..size {
            sum += *(array.add(n))
        }
    }
    sum
}