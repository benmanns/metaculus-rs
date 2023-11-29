# Considerations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_state** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**url** | **String** |  | [readonly]
**page_url** | **String** |  | [readonly]
**id** | **i32** |  | [readonly]
**author** | **i32** |  | [readonly]
**author_name** | Option<**String**> |  | [readonly]
**title** | Option<**String**> |  | [optional]
**title_short** | Option<**String**> |  | [optional][default to ]
**group_label** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::Status3baEnum**](Status3baEnum.md)> |  | [optional]
**resolution** | Option<**f64**> |  | [readonly]
**created_time** | **String** |  | [readonly]
**publish_time** | Option<**String**> |  | [optional]
**close_time** | Option<**String**> |  | [optional]
**effected_close_time** | **String** |  | [readonly]
**resolve_time** | Option<**String**> |  | [optional]
**possibilities** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**scoring** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][default to {}]
**r#type** | Option<[**crate::models::QuestionTypes**](QuestionTypes.md)> |  | [optional]
**user_perms** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**weekly_movement** | Option<**f64**> |  | [optional]
**weekly_movement_direction** | Option<**i32**> |  | [optional]
**cp_reveal_time** | Option<**String**> |  | [optional]
**edited_time** | **String** |  | [readonly]
**metaculus_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**community_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**consideration_voters** | **Vec<i32>** |  | [readonly]
**user_voted_on_consideration** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


