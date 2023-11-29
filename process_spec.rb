#!/usr/bin/env ruby

require 'yaml'

def process(spec)
  spec
end

# Not safe, don't pwn me Metaculus
spec = open('Metaculus API (1.0).yaml', 'r') { |f| YAML.load(f) }

open('Metaculus API (1.0) Modified.yaml', 'w') { |f| f.write(YAML.dump(process(spec))) }
