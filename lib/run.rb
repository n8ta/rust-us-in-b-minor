require 'rutie'

Rutie.new(:barers).init 'bare_init', __dir__


array = BareArray.new

array.push(1)
array.push("string")
array.push(:symbol)

array.length == 3
puts array.length
# b = RubyBareArray.new(1,1)
# b.print
# arr = Bare::FixedArray.new(Bare::F32.new, 3)
# data = [1.0,2.0,3.0]
# arr.encode(data)
