#!/usr/bin/env ruby

require 'yaml'

def process(spec)
  spec['paths'].each do |path, methods|
    methods.each do |method, details|
      if details['parameters']
        details['parameters'].each do |parameter|
          if parameter['in'] == 'query' && parameter['required'] == false
            # in: query implies required: false, so we can remove it
            parameter.delete('required')
          end
        end
      end
    end
  end

  spec
end

# Not safe, don't pwn me Metaculus
spec = open('Metaculus API (1.0).yaml', 'r') { |f| YAML.load(f) }

open('Metaculus API (1.0) Modified.yaml', 'w') { |f| f.write(YAML.dump(process(spec))) }
