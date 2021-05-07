require 'fiddle/import'

module Fib
  module FFI
    extend Fiddle::Importer
    dlload File.expand_path('./librustbare.dylib', __dir__)
    extern 'void hello_world(uint32_t x)'
  end
end