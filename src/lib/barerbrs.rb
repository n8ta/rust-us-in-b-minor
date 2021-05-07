require_relative 'librustbare/ffi'

# Hello from RUST!
puts Fib::FFI.hello_world(1337)