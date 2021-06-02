require_relative "../lib/bare.rb"
require 'json'

test_3_struct_inner = {
  orderId: Bare.I64,
  quantity: Bare.I32
}


test_2_enum = {0 => "ACCOUNTING",
               1 => "ADMINISTRATION",
               2 => "CUSTOMER_SERVICE",
               3 => "DEVELOPMENT",
               99 => "JSMITH" }

pkey = Bare.DataFixedLen(128)
addr = Bare.Struct({address: Bare.ArrayFixedLen(Bare.String, 4),
                    city: Bare.String,
                    state: Bare.String,
                    country: Bare.String})

dept = Bare.Enum(test_2_enum)
time = Bare.String
schema = {
  PublicKey: pkey,
  Time: time,
  Department: dept,
  Customer: Bare.Struct(
    {
      PublicKey: pkey,
      address: addr,
      name: Bare.String,
      email: Bare.String,
      orders: Bare.Array(Bare.Struct(test_3_struct_inner)),
      metadata: Bare.Map(Bare.String, Bare.Data)
    }),
  Employee: Bare.Struct(
    {
      name: Bare.String,
      email: Bare.String,
      address: addr,
      department: dept,
      hireDate: time,
      publicKey: Bare.Optional(pkey),
      metadata: Bare.Map(Bare.String, Bare.Data)
    }
  ),
  Person: Bare.Union({0 => :Customer, 1 => :Employee}),
  Address: addr,
}

address = {
  address: ["Address line 1", "", "", ""],
  city: "The big city",
  state: "Drenthe",
  country: "The Netherlands" }

customer = {
  name: "Martijn Braam",
  email: "spam@example.org",
  address: address,
  PublicKey: "1" * 128,
  orders: [
    { orderId: 5, quantity: 1 },
    { orderId: 6, quantity: 2 }
  ],
  metadata: {
    "ssh" => "jafsl8dfaf2",
    "gpg" => "jofa8f2jdlasfj8",
  }
}


address_json = {
  "address": ["Address line 1", "", "", ""],
  "city": "The big city",
  "state": "Drenthe",
  "country": "The Netherlands" }

customer_json = {
  "name": "Martijn Braam",
  "email": "spam@example.org",
  "address": address_json,
  "orders": [
    { "orderId": 5, "quantity": 1 },
    { "orderId": 6, "quantity": 2 }
  ],
  "metadata": {
    "ssh" => "jafsl8dfaf2",
    "gpg" => "jofa8f2jdlasfj8",
  }
}

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
#
acc = 0

timeit(100_000, "Benching JSON serialize/deserialize") do
  bin = JSON.dump(customer_json)
  native = JSON.parse(bin)
  acc += native["orders"][0]["orderId"]
end

timeit(100_000, "Benching bare serialize/deserialize") do
  bin = Bare.encode(customer, schema[:Customer])
  native = Bare.decode(bin, schema[:Customer])
  acc += native[:orders][0][:orderId]
end

puts acc

