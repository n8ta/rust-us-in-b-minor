require 'rutie'

module Bare
  # VERSION = "0.1.0"

  class FixedArray
    TYPE = 1
    def initialize(type, len)
      @type = type
      @len = len
    end

    def encode(arr)
      BareRustInterface.encode(@type, @len)
    end
    def decode(arr)
    end

  end

  class F32
    TYPE = 2
    def initialize
    end
    def encode(num)
    end
    def decode(msg)
    end
  end

end
