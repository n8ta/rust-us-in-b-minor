require_relative './timeit'

def movie_in_chunks(chunk_size)
  file = File.open("/Users/n8ta/Desktop/Its.Such.a.Beautiful.Day.mkv", mode="rb")
  result = []
  while !file.eof?
    result.append(file.read(chunk_size))
  end
  file.close
  result = result[0..result.length-2]
  result
end

def test_ruby_4(data)
  require 'bare-rb' # Gem
  puts "Testing ruby"
  schema = Bare.Array(Bare.DataFixedLen(4096))
  binary = timeit(0, "RUBY: Array(DataFixedLen(4096)) encoding") do
    Bare.encode(data, schema)
  end
  timeit(0, "RUBY: Array(DataFixedLen(4096)) decoding") do
    original = Bare.decode(binary, schema)
  end
end

def test_rust_4(data)
  require_relative '../lib/bare'
  puts "Testing rust"
  schema = Bare.Array(Bare.DataFixedLen(4096))
  binary = timeit(0, "RUST: Array(DataFixedLen(4096)) encoding") do
    Bare.encode(data, schema)
  end
  timeit(0, "RUST: Array(DataFixedLen(4096)) decoding") do
    original = Bare.decode(binary, schema)
  end
end

puts "Loading movie into memory and chunking"
movie = movie_in_chunks(4096)
puts "Done"

test_rust_4(movie)
test_ruby_4(movie)
