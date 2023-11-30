#!/usr/bin/env ruby

require 'set'
require 'yaml'

# clear_parameters clears the parameters from the spec that are in the
# parameters set. It expects to clear the parameters, and if not found,
# it will raise an error, to ensure that we're updating the parameter
# modifications as the original spec changes.
# To clear by name, use the name as a string.
# To clear by name and in, use an array of [name, in].
def clear_parameters(spec, path, method, parameters)
  found = Set.new
  parameters = Set.new(parameters)
  spec['paths'][path][method]['parameters'].each do |parameter|
    if parameters.include?(parameter['name'])
      found.add(parameter['name'])
      parameters.delete(parameter['name'])
    elsif parameters.include?([parameter['name'], parameter['in']])
      found.add([parameter['name'], parameter['in']])
      parameters.delete([parameter['name'], parameter['in']])
    end
  end
  raise "Parameter#{parameters.length > 1 ? 's' : ''} not found: #{parameters.to_a.sort.map(&:inspect).join(', ')}" unless parameters.empty?
  spec['paths'][path][method]['parameters'].reject! do |parameter|
    found.include?(parameter['name']) || found.include?([parameter['name'], parameter['in']])
  end
end

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

        # in: path but not in path, so we should remove it
        details['parameters'].reject! { |p| p['in'] == 'path' && !path.include?("{#{p['name']}}") }

        # in: query, but also in: path, so we should remove query
        path_parameters = details['parameters'].select { |p| p['in'] == 'path' }.map { |p| p['name'] }
        details['parameters'].reject! { |p| p['in'] == 'query' && path_parameters.include?(p['name']) }
      end
    end
  end

  # Seemingly unused parameters
  clear_parameters(spec, '/api2/categories/', 'get', %w[id long_name short_name url])
  clear_parameters(spec, '/api2/categories/{bare_id}/', 'get', %w[id long_name short_name url])
  clear_parameters(spec, '/api2/user-profiles/', 'get', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as id is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/user-profiles/{id}/', 'get', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/user-profiles/{id}/', 'put', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/user-profiles/{id}/', 'patch', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/', 'get', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as id is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/{id}/', 'get', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])

  # Add servers config to aid generated code and docs
  spec['servers'] = [{ 'url' => 'https://www.metaculus.com' }]

  spec
end

# Not safe, don't pwn me Metaculus
spec = open('Metaculus API (1.0).yaml', 'r') { |f| YAML.load(f) }

open('Metaculus API (1.0) Modified.yaml', 'w') { |f| f.write(YAML.dump(process(spec))) }
