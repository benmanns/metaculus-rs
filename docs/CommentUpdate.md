# CommentUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | [readonly]
**id** | **i32** |  | [readonly]
**author** | Option<**i32**> |  | [readonly]
**author_name** | Option<**String**> |  | [readonly]
**author_lvl** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**author_supporter_lvl** | **i32** |  | [readonly]
**is_deactivated** | **bool** |  | [readonly]
**is_moderator** | **bool** |  | [readonly]
**is_admin** | **bool** |  | [readonly]
**question** | **String** |  | 
**comment_html** | **String** |  | [readonly]
**comment_text** | **String** |  | 
**created_time** | **String** |  | [readonly]
**prediction_value** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**submit_type** | Option<[**crate::models::SubmitTypeEnum**](SubmitTypeEnum.md)> |  | [optional]
**preview** | Option<**bool**> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**num_children** | **i32** |  | [readonly][default to 0]
**num_likes** | Option<**i32**> |  | [readonly]
**deleted** | Option<**bool**> |  | [optional]
**parent_author** | Option<**String**> |  | [readonly]
**user_like** | **i32** |  | [readonly][default to 0]
**user_like_at** | **String** |  | [readonly]
**latest_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**include_latest_prediction** | Option<**bool**> |  | [optional]
**edits** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


