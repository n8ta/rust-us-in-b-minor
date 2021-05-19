require 'rutie'

Rutie.new(:barers).init 'bare_init', __dir__


float32 = BareFloat64.new()
floatarr = BareFixedArray.new(3)
init = [1.1,2.2,3.3]
encoded = floatarr.encode(init)
decode = floatarr.decode(encoded)
puts "#{decode} #{init}"



encoded = float32.encode(1337.1337)
puts "Encoded: ours #{encoded.b.inspect} vs correct: \\xe7\\x1d\\xa7\\xe8\\x88\\xe4\\x94\\x40"
decoded = float32.decode(encoded)
puts "num: #{decoded} 1337.1337"