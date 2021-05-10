require 'fiddle/import'

module Fib
  module FFI
    extend Fiddle::Importer
    dlload File.expand_path('./librustbare.dylib', __dir__)
    extern 'void hello_world(uint32_t x)'
    extern 'int32_t sum_array(size_t size, const int32_t* array)'
  end
end