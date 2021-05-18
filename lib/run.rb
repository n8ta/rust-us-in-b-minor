require_relative './rutie_ruby_example/version'
Rutie.new(:barers).init 'bare_init', __dir__

arr = Bare::FixedArray.new(Bare::F32.new, 3)
data = [1.0,2.0,3.0]
arr.encode(data)
