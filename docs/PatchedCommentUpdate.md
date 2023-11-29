# PatchedCommentUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> |  | [optional][readonly]
**id** | Option<**i32**> |  | [optional][readonly]
**author** | Option<**i32**> |  | [optional][readonly]
**author_name** | Option<**String**> |  | [optional][readonly]
**author_lvl** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**author_supporter_lvl** | Option<**i32**> |  | [optional][readonly]
**is_deactivated** | Option<**bool**> |  | [optional][readonly]
**is_moderator** | Option<**bool**> |  | [optional][readonly]
**is_admin** | Option<**bool**> |  | [optional][readonly]
**question** | Option<**String**> |  | [optional]
**comment_html** | Option<**String**> |  | [optional][readonly]
**comment_text** | Option<**String**> |  | [optional]
**created_time** | Option<**String**> |  | [optional][readonly]
**prediction_value** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]
**submit_type** | Option<[**crate::models::SubmitTypeEnum**](SubmitTypeEnum.md)> |  | [optional]
**preview** | Option<**bool**> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**num_children** | Option<**i32**> |  | [optional][readonly][default to 0]
**num_likes** | Option<**i32**> |  | [optional][readonly]
**deleted** | Option<**bool**> |  | [optional]
**parent_author** | Option<**String**> |  | [optional][readonly]
**user_like** | Option<**i32**> |  | [optional][readonly][default to 0]
**user_like_at** | Option<**String**> |  | [optional][readonly]
**latest_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**include_latest_prediction** | Option<**bool**> |  | [optional]
**edits** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


