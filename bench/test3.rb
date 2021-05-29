require_relative './timeit'

def test_ruby_3(data, count)
  require 'bare-rb' # Gem
  puts "Testing ruby"
  timeit(count, "RUBY: Array(Optional(Uint)))") do
    schema = Bare.Array(Bare.Optional(Bare.Uint))
    binary = Bare.encode(data, schema)
    original = Bare.decode(binary, schema)
    puts original == data
  end
end

def test_rust_3(data, count)
  require_relative '../lib/bare'
  puts "Testing rust"
  timeit(count, "RUST: Array(Optional(Uint)))") do
    schema = Bare.Array(Bare.Optional(Bare.Uint))
    binary = Bare.encode(data, schema)
    original = Bare.decode(binary, schema)
    puts original == data
  end
end

uints = []
(0..1_000_000).each do
  if Random.rand(1) == 0
    uints.append(nil)
  else
    uints.append Random.rand(2**32)
  end
end

test_ruby_3(uints, 0)
test_rust_3(uints, 0)
