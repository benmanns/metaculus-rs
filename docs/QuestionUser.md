# QuestionUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_state** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**url** | **String** |  | [readonly]
**page_url** | **String** |  | [readonly]
**id** | **i32** |  | [readonly]
**author** | **i32** |  | [readonly]
**author_name** | **String** |  | [readonly]
**title** | Option<**String**> |  | [optional]
**title_short** | **String** |  | [readonly]
**group_label** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::Status3baEnum**](Status3baEnum.md)> |  | [optional]
**resolution** | Option<**f64**> |  | [readonly]
**created_time** | **String** |  | [readonly]
**publish_time** | Option<**String**> |  | [optional]
**close_time** | Option<**String**> |  | [optional]
**effected_close_time** | Option<**String**> |  | [readonly]
**resolve_time** | Option<**String**> |  | [optional]
**possibilities** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**scoring** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][default to {}]
**r#type** | Option<[**crate::models::QuestionTypes**](QuestionTypes.md)> |  | [optional]
**user_perms** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**weekly_movement** | Option<**f64**> |  | [optional]
**weekly_movement_direction** | Option<**i32**> |  | [optional]
**cp_reveal_time** | Option<**String**> |  | [optional]
**edited_time** | **String** |  | [readonly]
**last_activity_time** | **String** |  | [readonly]
**activity** | **f64** |  | [readonly]
**comment_count** | **i32** |  | [readonly]
**votes** | **i32** |  | [readonly]
**metaculus_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**community_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**number_of_forecasters** | Option<**i32**> |  | [readonly]
**prediction_count** | **i32** |  | 
**related_questions** | Option<[**Vec<crate::models::QuestionRelated>**](QuestionRelated.md)> |  | [readonly]
**group** | Option<**i32**> |  | [readonly]
**condition** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**sub_questions** | [**Vec<crate::models::SubQuestionUserList>**](SubQuestionUserList.md) |  | 
**options** | [**Vec<crate::models::Option>**](Option.md) |  | [readonly]
**has_fan_graph** | **bool** |  | [readonly]
**projects** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**community_absolute_log_score** | Option<**f64**> |  | [readonly]
**metaculus_absolute_log_score** | Option<**f64**> |  | [readonly]
**metaculus_relative_log_score** | Option<**f64**> |  | [readonly]
**unweighted_community_prediction_v2** | Option<**String**> |  | [readonly]
**recency_weighted_community_prediction_v2** | Option<**String**> |  | [readonly]
**comment_count_snapshot** | **i32** |  | [readonly]
**user_vote** | **i32** |  | [readonly]
**user_community_vis** | [**crate::models::UserCommunityVisEnum**](UserCommunityVisEnum.md) |  | [readonly]
**my_predictions** | Option<[**crate::models::ExtendedPredictionUsername**](ExtendedPredictionUsername.md)> |  | [optional]
**divergence** | Option<**f64**> |  | [readonly]
**peer_score** | Option<**f64**> |  | [readonly]
**baseline_score** | Option<**f64**> |  | [readonly]
**user_prediction_sequence** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


