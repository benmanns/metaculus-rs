# PatchedReminder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**reminder** | Option<[**crate::models::ReminderEnum**](ReminderEnum.md)> |  | [optional]
**question** | Option<**i32**> |  | [optional]
**user** | Option<**i32**> |  | [optional]
**status** | Option<[**crate::models::ReminderStatusEnum**](ReminderStatusEnum.md)> |  | [optional]
**created_time** | Option<**String**> |  | [optional][readonly]
**note** | Option<**String**> |  | [optional]
**question_title** | Option<**String**> |  | [optional][readonly]
**status_label** | Option<**String**> |  | [optional][readonly]
**reminder_label** | Option<**String**> |  | [optional][readonly]
**remind_on_date** | Option<**String**> |  | [optional]
**remind_on_new_comments** | Option<**i32**> |  | [optional]
**remind_base_prediction** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**remind_on_percent_lifetime** | Option<**i32**> |  | [optional]
**next_trigger_date** | Option<**String**> |  | [optional]
**next_trigger_value** | Option<**i32**> |  | [optional]
**repeat_pattern** | Option<[**crate::models::RepeatPatternEnum**](RepeatPatternEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


