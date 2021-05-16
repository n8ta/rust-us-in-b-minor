
# frozen_string_literal: true

require_relative './rutie_ruby_example/version'
require 'rutie'

x = Rutie.new(:rutie_ruby_example).init 'Init_hello_world', __dir__
puts HelloClass.hello_world