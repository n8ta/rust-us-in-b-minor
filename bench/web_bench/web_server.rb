require 'socket'


puts ARGV[0]
if ARGV[0] == "RUBY"
  puts "Using RUBY bare"
  require 'bare-rb'
elsif ARGV[0] == "RUST"
  puts "Using RUST bare"
  require_relative '../../lib/bare'
else
  puts "RUBY or RUST bare impl"
  exit(-1)
end


port_to_listen_to = 8081

request_schema = Bare.Array(Bare.I16)
response_schema = Bare.Array(Bare.Optional(Bare.F64))

database = []
(0..(2 ** 17)).each do ||
  database.append(rand)
end

server = TCPServer.open(port_to_listen_to)

# def bin_to_hex(binary_string)
#   binary_string.unpack('H*').first
# end

loop {
  client = server.accept
  Thread.new do
    # db = database.dup
    loop do

      eight_bytes = client.recv(8)
      request_size = Bare.decode(eight_bytes, Bare.I64)
      request = client.recv(request_size)
      requests = Bare.decode(request, request_schema)

      response = []
      requests.each do |i16|
        response.push(database[i16.abs])
      end

      bin_response = Bare.encode(response, response_schema)

      client << Bare.encode(bin_response.size, Bare.I64)
      client << bin_response
    end
  end
}