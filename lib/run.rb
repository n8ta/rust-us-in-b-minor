require 'rutie'

Rutie.new(:barers).init 'bare_init', __dir__

float32 = BareFloat32.new()
float64 = BareFloat64.new()
puts "Creating BareFixedArray(F32)"
floatarr32 = BareFixedArray.new(3, float32)
puts "Creating BareFixedArray(F64)"
floatarr64 = BareFixedArray.new(3, float64)

init = [1.1,2.2,3.3]
encoded64 = floatarr64.encode(init)
decode64 = floatarr64.decode(encoded64)

encoded32 = floatarr32.encode(init)
decode32 = floatarr32.decode(encoded32)

puts "f64: #{decode64} #{init}"
puts "f32: #{decode32} #{init}"


encoded = float32.encode(1337.1337)
puts "Encoded: ours #{encoded} vs correct: \\xe7\\x1d\\xa7\\xe8\\x88\\xe4\\x94\\x40"
decoded = float32.decode(encoded)
puts "num: #{decoded} 1337.1337"


float_arr_arr_64 = BareFixedArray.new(3, floatarr64)
twoD = [[1.1,1.2,1.3],[2.1,2.2,2.3],[3.1,3.2,3.3]]
encoded = float_arr_arr_64.encode(twoD)
decoded = float_arr_arr_64.decode(encoded)
puts "init: #{twoD} VS \n      #{decoded}"