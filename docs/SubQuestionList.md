# SubQuestionList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_state** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**id** | **i32** |  | [readonly]
**resolution** | Option<**f64**> |  | [readonly]
**publish_time** | Option<**String**> |  | [optional]
**close_time** | Option<**String**> |  | [optional]
**effected_close_time** | **String** |  | [readonly]
**resolve_time** | Option<**String**> |  | [optional]
**possibilities** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**sub_question_label** | Option<**String**> |  | [optional]
**metaculus_prediction** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**community_prediction** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**conditioned_on_resolution** | **f64** |  | 
**cp_reveal_time** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


