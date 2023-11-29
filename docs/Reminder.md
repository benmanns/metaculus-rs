# Reminder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**reminder** | Option<[**crate::models::ReminderEnum**](ReminderEnum.md)> |  | [optional]
**question** | **i32** |  | 
**user** | **i32** |  | 
**status** | Option<[**crate::models::ReminderStatusEnum**](ReminderStatusEnum.md)> |  | [optional]
**created_time** | **String** |  | [readonly]
**note** | Option<**String**> |  | [optional]
**question_title** | Option<**String**> |  | [readonly]
**status_label** | **String** |  | [readonly]
**reminder_label** | **String** |  | [readonly]
**remind_on_date** | Option<**String**> |  | [optional]
**remind_on_new_comments** | Option<**i32**> |  | [optional]
**remind_base_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**remind_on_percent_lifetime** | Option<**i32**> |  | [optional]
**next_trigger_date** | Option<**String**> |  | [optional]
**next_trigger_value** | Option<**i32**> |  | [optional]
**repeat_pattern** | Option<[**crate::models::RepeatPatternEnum**](RepeatPatternEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


