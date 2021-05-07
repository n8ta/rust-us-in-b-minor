require "rake/extensiontask"


task :compile do
  `cd ext/barerbrs && cargo build`
  `mv ext/barerbrs/target/debug/librustbare.dylib ./src/lib/librustbare`
end

task :clean do
  `cd ext/barerbrs && cargo clean`
  `rm ./src/librustbare.dylib`
end
