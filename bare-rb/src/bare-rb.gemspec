Gem::Specification.new do |s|
  s.name        = 'bare-rb'
  s.version     = '0.1.5'
  s.date        = '2020-10-13'
  s.summary     = "Bare Message Encoding Implementation"
  s.description = "The first implementation of the BARE (Binary Application Record Encoding) in Ruby. Includes schema parsing and compatibility tests."
  s.authors     = ["Nate Tracy-Amoroso"]
  s.email       = 'n8@u.northwestern.edu'
  s.files       = %w[./lib/bare-rb.rb ./lib/types.rb ./lib/exceptions.rb ./lib/lexer.rb ./lib/parser.rb, ./ext/**/*]
  s.homepage    = 'https://github.com/n8ta/bare-rb'
  s.extensions  = %w[ext/barerbrs/extconf.rb]
  s.license     = 'MIT'
  s.required_ruby_version  = '2.5'
  s.add_development_dependency "rake-compiler"
  s.add_dependency "ffi"
end