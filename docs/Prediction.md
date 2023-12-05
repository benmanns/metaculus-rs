# Prediction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**predictions** | Option<[**Vec<::std::collections::HashMap<String, serde_json::Value>>**](map.md)> |  | [optional]
**points_won** | Option<**f64**> |  | [optional]
**user** | Option<**i32**> |  | 
**question** | **i32** |  | 
**active** | Option<**bool**> |  | [optional]
**log_score** | Option<**f64**> | Used in tournament scoring. Defined as log2(player prediction / community prediction), averaged over the lifetime of the question. Zero for null / void predictions. | [optional]
**absolute_log_score** | Option<**f64**> | Defined as per our scoring FAQ. Zero for null / void predictions. | [optional]
**coverage** | Option<**f64**> | Percentage of question open time that this prediction has been active. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


