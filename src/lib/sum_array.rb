require_relative 'librustbare/ffi'
require 'ffi'

array = [1,2,3]
ptr = FFI::MemoryPointer.new(:int32, array.size)
ptr.write_array_of_type(FFI::Type::INT32, :put_int32, array)

puts Fib::FFI.sum_array(3, ptr.address)

