require_relative './timeit'

def test_ruby_2(data, count)
  require 'bare-rb' # Gem
  puts "Testing ruby"
  timeit(count, "Ruby Array(variable sized uint) encoding") do
    schema = Bare.Array(Bare.Uint)
    binary = Bare.encode(data, schema)
    original = Bare.decode(binary, schema)
  end
end

def test_rust_2(data, count)
  require_relative '../lib/bare'
  puts "Testing rust"
  timeit(count, "Rust Array(variable sized uint) encoding") do
    schema = Bare.Array(Bare.Uint)
    binary = Bare.encode(data, schema)
    original = Bare.decode(binary, schema)
  end
end

uints = []
(0..1_000_00).each do
  uints.append Random.rand(2**32)
end

test_ruby_2(uints, 1)
test_rust_2(uints, 1)
