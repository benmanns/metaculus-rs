# ProjectUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**name** | **String** |  | 
**url** | **String** |  | [readonly]
**type_str** | **String** |  | [readonly]
**score_type** | [**crate::models::ScoreTypeEnum**](ScoreTypeEnum.md) |  | [readonly]
**subtitle** | **String** |  | [readonly]
**description** | Option<**String**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**site_id** | **i32** |  | [readonly]
**user_perms** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**question_user_perms** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**sign_up_fields** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**tournament_close_date** | Option<**String**> |  | [optional]
**tournament_start_date** | Option<**String**> |  | [optional]
**prize_pool** | **f64** |  | [readonly]
**config** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**in_main_feed_by_default** | Option<**bool**> |  | [optional]
**public** | Option<**bool**> |  | [optional]
**is_opt_in** | **bool** |  | [readonly]
**organizations** | Option<[**Vec<crate::models::ProjectOrganization>**](ProjectOrganization.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


