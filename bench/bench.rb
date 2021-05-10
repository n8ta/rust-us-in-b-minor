require_relative '../src/lib/bare-rb'
require 'benchmark'

dead_beef_1000000 = ["\xDE\xAD\xBE\xEF".b] * 1024 * 1
ruby_schema = Bare.Array(Bare.Data)
# rust_schema = BareRust.Array(BareRust.Data)

def bench(class_obj, schema, data)
  Benchmark.measure {
    encoded = class_obj.encode(data, schema)
    decoded = class_obj.decode(encoded, schema)
    if decoded != data
      raise Exception.new("DECODE DOESN'T MATCH ORIGINAL")
    end
  }
end

ruby = bench(Bare, ruby_schema, dead_beef_1000000)
# rust = bench(BareRust, rust_schema, dead_beef_1000000)
puts "Ruby: #{ruby.real.round(4)}s"
# puts "Rust: #{output.real.round(3)}s"