# PatchedQuestionUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_state** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**page_url** | Option<**String**> |  | [optional][readonly]
**id** | Option<**i32**> |  | [optional][readonly]
**author** | Option<**i32**> |  | [optional][readonly]
**author_name** | Option<**String**> |  | [optional][readonly]
**title** | Option<**String**> |  | [optional]
**title_short** | Option<**String**> |  | [optional][default to ]
**group_label** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::QuestionUpdateStatusEnum**](QuestionUpdateStatusEnum.md)> |  | [optional]
**resolution** | Option<**f64**> |  | [optional][readonly]
**created_time** | Option<**String**> |  | [optional][readonly]
**publish_time** | Option<**String**> |  | [optional]
**close_time** | Option<**String**> |  | [optional]
**effected_close_time** | Option<**String**> |  | [optional][readonly]
**resolve_time** | Option<**String**> |  | [optional]
**possibilities** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**scoring** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][default to {}]
**r#type** | Option<[**crate::models::QuestionTypes**](QuestionTypes.md)> |  | [optional]
**user_perms** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**weekly_movement** | Option<**f64**> |  | [optional]
**weekly_movement_direction** | Option<**i32**> |  | [optional]
**cp_reveal_time** | Option<**String**> |  | [optional]
**edited_time** | Option<**String**> |  | [optional][readonly]
**categories** | Option<**Vec<String>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**description_html** | Option<**String**> |  | [optional]
**resolution_criteria** | Option<**String**> |  | [optional]
**resolution_criteria_html** | Option<**String**> |  | [optional]
**fine_print** | Option<**String**> |  | [optional]
**fine_print_html** | Option<**String**> |  | [optional]
**prediction_count** | Option<**i32**> |  | [optional][readonly]
**parent_question_id** | Option<**i32**> |  | [optional]
**related_questions** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**sub_questions** | Option<[**Vec<crate::models::SubQuestionUpdate>**](SubQuestionUpdate.md)> |  | [optional]
**projects** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**edit_form_data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**projects_input** | Option<[**Vec<crate::models::QuestionProject>**](QuestionProject.md)> |  | [optional]
**conditioned_on** | Option<**i32**> |  | [optional]
**unconditional_question** | Option<**i32**> |  | [optional]
**options** | Option<[**Vec<crate::models::Option>**](Option.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


