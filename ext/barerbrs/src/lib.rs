#[no_mangle]
pub extern "C" fn hello_world(x: i32) {
    println!("Hello from rust {}", x);
}