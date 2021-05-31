require_relative './timeit'

chunk_size = 4098

def movie_in_chunks(chunk_size)
  file = File.open("/Users/n8ta/Desktop/Its.Such.a.Beautiful.Day.mkv", mode="rb")
  result = []
  0.upto(1000) do 
    result.append(file.read(chunk_size))
  end
  file.close
  result[0..result.length-2]
end

def test_ruby_4(data, count, chunk_size)
  require 'bare-rb' # Gem
  puts "Testing ruby"
  schema = Bare.Array(Bare.DataFixedLen(chunk_size))
  # schema = Bare.Array(Bare.DataFixedLen(chunk_size)
  binary = timeit(count, "RUBY: Array(DataFixedLen(chunk_size)) encoding") do
    Bare.encode(data, schema)
  end
  timeit(count, "RUBY: Array(DataFixedLen(chunk_size)) decoding") do
    original = Bare.decode(binary, schema)
  end
end

def test_rust_4(data, count, chunk_size)
  require_relative '../lib/bare'
  puts "Testing rust"
  schema = Bare.Array(Bare.DataFixedLen(chunk_size))
  #schema = Bare.DataFixedLen(chunk_size)
  binary = timeit(count, "RUST: Array(DataFixedLen(chunk_size)) encoding") do
    Bare.encode(data, schema)
  end
  timeit(count, "RUST: Array(DataFixedLen(chunk_size)) decoding") do
    original = Bare.decode(binary, schema)
  end
end

puts "Loading movie into memory and chunking"
movie = movie_in_chunks(chunk_size)
puts "Done"


count = 5000
test_rust_4(movie, count, chunk_size)
test_ruby_4(movie, count, chunk_size)
