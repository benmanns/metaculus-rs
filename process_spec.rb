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
  clear_parameters(spec, '/api2/users/{id}/', 'put', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/{id}/', 'patch', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/{id}/collect-tachyons/', 'post', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/{id}/purchase-track-record/', 'post', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/{id}/unlock-power/', 'post', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/global-cp-reminder/', 'get', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as id is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])
  clear_parameters(spec, '/api2/users/global-cp-reminder/', 'post', %w[ask_when_reaffirm_question_modal date_joined default_community_visibility default_mp_visibility email first_name formerly_known_as id is_staff is_superuser last_name last_visited level levelTitle permissions powers purchasable_track_record score show_profile_comments supporter_level supporter_since tachyons url username username_change_cost])

  schemas_were_sorted = spec['components']['schemas'].keys == spec['components']['schemas'].keys.sort

  if spec['paths']['/api2/about-numbers/']['get']['responses']['200'] == { 'description' => 'No response body' }
    # Add AboutNumbers response schema based on observed response
    spec['paths']['/api2/about-numbers/']['get']['responses']['200'] = {
      'content' => {
        'application/json' => {
          'schema' => {
            '$ref' => '#/components/schemas/AboutNumbers'
          }
        }
      },
      'description' => ''
    }
    spec['components']['schemas']['AboutNumbers'] = {
      'type' => 'object',
      'properties' => {
        'predictions' => { 'type' => 'integer' },
        'questions' => { 'type' => 'integer' },
        'resolved_questions' => { 'type' => 'integer' },
        'years_of_predictions' => { 'type' => 'integer' }
      },
      'required' => ['predictions', 'questions', 'resolved_questions', 'years_of_predictions']
    }

    check_reminder_schema = false
    reminder_schema_lists = [
      spec['paths']['/api2/users/global-cp-reminder/']['get']['responses']['200']['content'],
      spec['paths']['/api2/users/global-cp-reminder/']['post']['requestBody']['content'],
      spec['paths']['/api2/users/global-cp-reminder/']['post']['responses']['200']['content']
    ]
    reminder_schema_lists.each do |list|
      list.each do |content_type, content|
        if content['schema']['$ref'] == '#/components/schemas/User'
          content['schema']['$ref'] = '#/components/schemas/GlobalCPReminder'
          check_reminder_schema = true
        end
      end
    end

    if check_reminder_schema
      spec['components']['schemas']['GlobalCPReminder'] ||= {
        'type' => 'object',
        'properties' => {
          'enabled' => { 'type' => 'boolean' },
          'delta' => { 'type' => 'integer' }
        },
        'required' => ['enabled', 'delta']
      }
    end

    if spec['paths']['/api2/questions/{id}/prediction-history/']['get']['responses']['200'] == { 'description' => 'No response body' }
      # Add PredictionHistory response schema based on observed response
      spec['paths']['/api2/questions/{id}/prediction-history/']['get']['responses']['200'] = {
        'content' => {
          'application/json' => {
            'schema' => {
              '$ref' => '#/components/schemas/PredictionHistory'
            }
          }
        },
        'description' => ''
      }

      spec['components']['schemas']['PredictionHistory'] = {
        'type' => 'object',
        'properties' => {
          'question' => { 'type' => 'integer' },
          'community_prediction' => { 'type' => 'array', 'items' => { '$ref' => '#/components/schemas/PredictionHistoryTime' } },
          'metaculus_prediction' => { 'type' => 'array', 'items' => { '$ref' => '#/components/schemas/PredictionHistoryTime' } },
        },
        'required' => ['question', 'community_prediction', 'metaculus_prediction']
      }

      spec['components']['schemas']['PredictionHistoryTime'] = {
        'type' => 'object',
        'properties' => {
          't' => { 'type' => 'number', 'format' => 'double' },
          'y' => { 'type' => 'array', 'items' => { 'type' => 'number', 'format' => 'double' } },
          'q1' => { 'type' => 'number', 'format' => 'double' },
          'q2' => { 'type' => 'number', 'format' => 'double' },
          'q3' => { 'type' => 'number', 'format' => 'double' },
          'low' => { 'type' => 'number', 'format' => 'double' },
          'high' => { 'type' => 'number', 'format' => 'double' },
        },
        'required' => ['t', 'y', 'q1', 'q2', 'q3', 'low', 'high']
      }
    end

    # /api2/questions/{id}/predictions/
    if spec['paths']['/api2/questions/{id}/predictions/']['get']['responses']['200'] == { 'description' => 'No response body' }
      # Add [ExtendedPredictionUsername] response schema based on observed response
      spec['paths']['/api2/questions/{id}/predictions/']['get']['responses']['200'] = {
        'content' => {
          'application/json' => {
            'schema' => {
              'type' => 'array',
              'items' => {
                '$ref' => '#/components/schemas/ExtendedPredictionUsername'
              }
            }
          }
        },
        'description' => ''
      }
    end

    check_predictions_schemas = ['ExtendedPredictionUsername', 'Prediction', 'PredictionUsername']
    check_predictions_schemas.each do |schema|
      if spec['components']['schemas'][schema]['properties']['predictions']['type'] == 'object'
        spec['components']['schemas'][schema]['properties']['predictions'] = {
          'type' => 'array',
          'items' => {
            'type' => 'object',
            'additionalProperties' => {}
          }
        }
      else
        raise "#{schema} predictions already fixed"
      end
    end

    # Make effected_close_time nullable based on observed responses
    # I have not been able to check the others:
    # * Considerations
    # * PatchedQuestionUpdate
    # * Question
    # * QuestionRelated
    # * QuestionUpdate
    check_effected_close_time_schemas = ['QuestionUser', 'QuestionUserDetail', 'SubQuestionList', 'SubQuestionUserDetail', 'SubQuestionUserList']
    check_effected_close_time_schemas.each do |schema|
      if spec['components']['schemas'][schema]['properties']['effected_close_time']['nullable']
        raise "#{schema} effected_close_time already nullable"
      end
      spec['components']['schemas'][schema]['properties']['effected_close_time']['nullable'] = true
    end

    if schemas_were_sorted
      spec['components']['schemas'] = spec['components']['schemas'].sort.to_h
    end
  end

  # Remove empty parameters (that we caused earlier)
  spec['paths'].each do |path, methods|
    methods.each do |method, details|
      details.delete('parameters') if details['parameters'] == []
    end
  end


  # Add servers config to aid generated code and docs
  spec['servers'] = [{ 'url' => 'https://www.metaculus.com' }]

  spec
end

# Not safe, don't pwn me Metaculus
spec = open('Metaculus API (1.0).yaml', 'r') { |f| YAML.load(f) }

open('Metaculus API (1.0) Modified.yaml', 'w') { |f| f.write(YAML.dump(process(spec))) }
