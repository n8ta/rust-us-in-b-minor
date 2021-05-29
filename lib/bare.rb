class Bare
  require 'rutie'

  Rutie.new(:barers).init 'bare_init', __dir__

  def self.encode(msg, schema)
    schema.encode(msg).force_encoding('ASCII-8BIT')
  end
  def self.decode(bin, schema)
    schema.decode(bin)
  end

  def self.F32
    Rust_F32.new
  end
  def self.F64
    Rust_F64.new
  end
  def self.ArrayFixedLen(type, size)
    Rust_ArrayFixedLen.new(type, size)
  end

  def self.Uint()
    Rust_Uint.new
  end

  def self.Int()
    Rust_Int.new
  end

  def self.Array(typ)
    Rust_Array.new typ
  end

  def self.I8
    Rust_I8.new
  end
  def self.I16
    Rust_I16.new
  end
  def self.I32
    Rust_I32.new
  end
  def self.I64
    Rust_I64.new
  end
  def self.Optional(typ)
    Rust_Opt.new(typ)
  end

  def self.DataFixedLen(len)
    Rust_DataFixedLen.new(len)
  end
end