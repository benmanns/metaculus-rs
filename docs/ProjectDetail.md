# ProjectDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**name** | **String** |  | 
**url** | **String** |  | [readonly]
**type_str** | **String** |  | [readonly]
**score_type** | [**crate::models::ScoreTypeEnum**](ScoreTypeEnum.md) |  | [readonly]
**subtitle** | **String** |  | [readonly]
**description** | **String** |  | [readonly]
**r#type** | **String** |  | [readonly]
**site_id** | **i32** |  | [readonly]
**user_perms** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**question_user_perms** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**sign_up_fields** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**tournament_close_date** | **String** |  | [readonly]
**tournament_start_date** | **String** |  | [readonly]
**prize_pool** | **f64** |  | [readonly]
**config** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**in_main_feed_by_default** | **bool** |  | [readonly]
**public** | **bool** |  | 
**is_opt_in** | **bool** |  | [readonly]
**user_permissions** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**essay_count** | **i32** |  | [readonly]
**completion_question_count** | **i32** |  | [readonly]
**organizations** | [**Vec<crate::models::Organization>**](Organization.md) |  | 
**header_logo** | **String** |  | 
**header_image** | **String** |  | 
**default_project_permissions** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**default_question_permissions** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**default_role** | Option<[**serde_json::Value**](.md)> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


