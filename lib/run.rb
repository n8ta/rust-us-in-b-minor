require 'rutie'

Rutie.new(:barers).init 'bare_init', __dir__


array = BareArray.new

array.push(1)
array.push("string")
array.push(:symbol)

array.length == 3
puts array.length


float32 = BareFloat64.new(33.0)
puts float32.encode(1337.1337).inspect
num = float32.decode("\xe7\x1d\xa7\xe8\x88\xe4\x94\x40".b)
puts "num: #{num} ?= 1337.1337"