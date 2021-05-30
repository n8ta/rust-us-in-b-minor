require 'socket'

require 'concurrent'

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

request_schema = Bare.Array(Bare.I16)
response_schema = Bare.Array(Bare.Optional(Bare.F64))

failure_occurred = Concurrent::AtomicBoolean.new
failure_occurred.make_false

requests_completed = Concurrent::AtomicFixnum.new(0)

def worker(request_schema, response_schema, failure_occurred, requests_completed)
  total = 0
  s = Socket.new Socket::AF_INET, Socket::SOCK_STREAM
  s.connect Socket.pack_sockaddr_in(8081, '0.0.0.0')
  loop do
    begin
      count = (Random.rand * 1000 + 2).to_i
      requests = []
      0.upto(count) do
        requests.append((Random.rand * (2 ** 16)).to_i)
      end
      requests_bin = Bare.encode(requests, request_schema)
      s << Bare.encode(requests_bin.size.to_i, Bare.I64)
      s << requests_bin

      response_size = s.recv(8)
      response_size = Bare.decode(response_size, Bare.I64)

      bin = s.recv(response_size)

      response = Bare.decode(bin, response_schema)
      response.each do |f|
        total += f
      end
      requests_completed.increment
    rescue Exception => e
      puts e.inspect
      puts e
      puts e.backtrace.reverse.join("\n")
      failure_occurred.make_true
      break
    end
  end
  s.close
  puts "Exiting..."
end

pool = Concurrent::FixedThreadPool.new(1_000_000)
old_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
old_value = requests_completed.value
threads = 0
while failure_occurred.false?
  time_delta = Process.clock_gettime(Process::CLOCK_MONOTONIC) - old_time
  completed = requests_completed.value - old_value
  puts "Req/s: #{(completed.to_f / time_delta.to_f).to_i}, Client Threads: #{threads}"

  threads += 1
  pool.post do
    worker(request_schema, response_schema, failure_occurred, requests_completed)
  end
  old_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  old_value = requests_completed.value
  sleep(1)
end

pool.shutdown
pool.wait_for_termination