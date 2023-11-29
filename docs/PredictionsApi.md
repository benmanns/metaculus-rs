# \PredictionsApi

All URIs are relative to *https://www.metaculus.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**predictions_list**](PredictionsApi.md#predictions_list) | **GET** /api2/predictions/ | 
[**predictions_retrieve**](PredictionsApi.md#predictions_retrieve) | **GET** /api2/predictions/{id}/ | 



## predictions_list

> crate::models::PaginatedPredictionUsernameList predictions_list(last_prediction_time__gt, last_prediction_time__lt, limit, offset, order_by, points_won, points_won__gt, points_won__gte, points_won__lt, points_won__lte, question, user, username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**last_prediction_time__gt** | Option<**String**> |  |  |
**last_prediction_time__lt** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order_by** | Option<**String**> | Which field to use when ordering the results. |  |
**points_won** | Option<**f32**> |  |  |
**points_won__gt** | Option<**f32**> |  |  |
**points_won__gte** | Option<**f32**> |  |  |
**points_won__lt** | Option<**f32**> |  |  |
**points_won__lte** | Option<**f32**> |  |  |
**question** | Option<**i32**> |  |  |
**user** | Option<**i32**> |  |  |
**username** | Option<**String**> |  |  |

### Return type

[**crate::models::PaginatedPredictionUsernameList**](PaginatedPredictionUsernameList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## predictions_retrieve

> crate::models::PredictionUsername predictions_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this combo prediction. | [required] |

### Return type

[**crate::models::PredictionUsername**](PredictionUsername.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

