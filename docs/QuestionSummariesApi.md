# \QuestionSummariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**question_summaries_feedback_create**](QuestionSummariesApi.md#question_summaries_feedback_create) | **POST** /api2/question-summaries/feedback/ | 
[**question_summaries_retrieve**](QuestionSummariesApi.md#question_summaries_retrieve) | **GET** /api2/question-summaries/{id}/ | 



## question_summaries_feedback_create

> crate::models::QuestionSummaryFeedback question_summaries_feedback_create(question_summary_feedback)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**question_summary_feedback** | [**QuestionSummaryFeedback**](QuestionSummaryFeedback.md) |  | [required] |

### Return type

[**crate::models::QuestionSummaryFeedback**](QuestionSummaryFeedback.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## question_summaries_retrieve

> crate::models::QuestionSummary question_summaries_retrieve(id)


Get the question summary for a given question.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::QuestionSummary**](QuestionSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

