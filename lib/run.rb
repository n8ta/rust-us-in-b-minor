require 'rutie'

Rutie.new(:barers).init 'bare_init', __dir__


array = BareArray.new

array.push(1)
array.push("string")
array.push(:symbol)

array.length == 3
puts array.length


float32 = BareFloat64.new()
# [5, "\x00\x00\x00\x00\x00\x00\x14\x40".b, Bare.F64]
encoded = float32.encode(1337.1337)
puts "Encoded: ours #{encoded.b.inspect} vs correct: \\xe7\\x1d\\xa7\\xe8\\x88\\xe4\\x94\\x40"
decoded = float32.decode(encoded)
puts "num: #{decoded} 1337.1337"