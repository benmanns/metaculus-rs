# SubQuestionUserDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_state** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**id** | **i32** |  | [readonly]
**resolution** | Option<**f64**> |  | [readonly]
**publish_time** | Option<**String**> |  | [optional]
**close_time** | Option<**String**> |  | [optional]
**effected_close_time** | Option<**String**> |  | [readonly]
**resolve_time** | Option<**String**> |  | [optional]
**possibilities** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**sub_question_label** | Option<**String**> |  | [optional]
**metaculus_prediction** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**community_prediction** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**conditioned_on_resolution** | **f64** |  | 
**cp_reveal_time** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**title_short** | Option<**String**> |  | [optional][default to ]
**prediction_count** | **i32** |  | 
**created_time** | **String** |  | [readonly]
**scoring** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][default to {}]
**closing_bonus** | **f64** |  | [readonly]
**user_perms** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**url** | **String** |  | [readonly]
**my_predictions** | [**crate::models::Prediction**](Prediction.md) |  | 
**user_community_vis** | [**crate::models::UserCommunityVisEnum**](UserCommunityVisEnum.md) |  | [readonly]
**peer_score** | Option<**f64**> |  | [readonly]
**baseline_score** | Option<**f64**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


