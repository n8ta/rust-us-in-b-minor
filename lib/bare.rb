class Bare

  require 'rutie'
  Rutie.new(:barers).init 'bare_init', __dir__

  def self.F32
    Rust_F32.new()
  end
  def self.F64
    Rust_F64.new()
  end
  def self.ArrayFixedLen(size, type)
    Rust_ArrayFixedLen.new(size, type)
  end

end