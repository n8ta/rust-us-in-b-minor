def timeit(count, message, &block)
  start = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  ret = nil
  (0..count).each do ||
    ret = block.call
  end
  elapsed = Process.clock_gettime(Process::CLOCK_MONOTONIC) - start
  puts "#{message} took #{elapsed}"
  ret
end
