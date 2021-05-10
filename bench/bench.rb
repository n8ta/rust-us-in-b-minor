require 'bare-rb'
require 'benchmark'

file = File.open("/Users/jack/Downloads/Midterm.zip", "rb").read

list = []
(0..(file.size / 1024)).each do |idx|
    list.append(file[idx * 1024..((idx + 1) * 1024)])
end

schema = Bare.Array(Bare.Data)

puts Benchmark.measure {
    output = Bare.encode(list, schema)
    output = Bare.decode(output, schema)
}
