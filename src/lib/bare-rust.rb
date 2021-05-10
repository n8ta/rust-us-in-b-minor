require_relative 'librustbare/ffi'

# Hello from RUST!
puts Fib::FFI.hello_world(1337)


class BareRust
  def self.encode(msg, schema, type=nil)
    # TODO:
  end

  def self.decode(msg, schema, type=nil)
    # TODO:
  end

  def self.Int
    # TODO:
  end

  def self.Array
    # TODO:
  end

end