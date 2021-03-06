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

  def self.Union(types)
    Rust_Union.new types
  end

  def self.Struct(hash)
    Rust_Struct.new hash
  end

  def self.Enum(hash)
    Rust_Enum.new hash
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

  def self.U8
    Rust_U8.new
  end

  def self.U16
    Rust_U16.new
  end

  def self.U32
    Rust_U32.new
  end

  def self.U64
    Rust_U64.new
  end

  def self.Bool
    Rust_Bool.new
  end

  def self.Void
    Rust_Void.new
  end

  def self.Data
    Rust_Data.new
  end

  def self.String
    Rust_String.new
  end

  def self.Map(from, to)
    Rust_Map.new from, to
  end

  def self.DataFixedLen(len)
    Rust_DataFixedLen.new(len)
  end

  def self.Map(from, to)
    Rust_Map.new(from, to)
  end
end