require_relative './timeit'

def test_ruby_1
  require 'bare-rb' # Gem
  puts "Testing ruby"
  timeit(1_000_000, "Ruby U8 encoding") do
    Bare.encode(65, Bare.I8)
  end
end

def test_rust_1
  require_relative '../lib/bare'
  puts "Testing rust"
  timeit(1_000_000, "Rust U8 encoding") do
    Bare.encode(65, Bare.I8)
  end
end

test_ruby_1
test_rust_1