# \QuestionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**questions_add_consideration_vote_create**](QuestionsApi.md#questions_add_consideration_vote_create) | **POST** /api2/questions/{id}/add_consideration_vote/ | 
[**questions_boost_create**](QuestionsApi.md#questions_boost_create) | **POST** /api2/questions/{id}/boost/ | 
[**questions_bulk_predict_create**](QuestionsApi.md#questions_bulk_predict_create) | **POST** /api2/questions/bulk-predict/ | 
[**questions_create**](QuestionsApi.md#questions_create) | **POST** /api2/questions/ | 
[**questions_destroy**](QuestionsApi.md#questions_destroy) | **DELETE** /api2/questions/{id}/ | 
[**questions_list**](QuestionsApi.md#questions_list) | **GET** /api2/questions/ | 
[**questions_partial_update**](QuestionsApi.md#questions_partial_update) | **PATCH** /api2/questions/{id}/ | 
[**questions_predict_create**](QuestionsApi.md#questions_predict_create) | **POST** /api2/questions/{id}/predict/ | 
[**questions_prediction_for_date_retrieve**](QuestionsApi.md#questions_prediction_for_date_retrieve) | **GET** /api2/questions/{id}/prediction-for-date/ | 
[**questions_prediction_history_retrieve**](QuestionsApi.md#questions_prediction_history_retrieve) | **GET** /api2/questions/{id}/prediction-history/ | 
[**questions_predictions_retrieve**](QuestionsApi.md#questions_predictions_retrieve) | **GET** /api2/questions/{id}/predictions/ | 
[**questions_question_sharing_create**](QuestionsApi.md#questions_question_sharing_create) | **POST** /api2/questions/{id}/question-sharing/{username}/ | 
[**questions_question_sharing_destroy**](QuestionsApi.md#questions_question_sharing_destroy) | **DELETE** /api2/questions/{id}/question-sharing/{username}/ | 
[**questions_remove_consideration_vote_create**](QuestionsApi.md#questions_remove_consideration_vote_create) | **POST** /api2/questions/{id}/remove_consideration_vote/ | 
[**questions_resolve_create**](QuestionsApi.md#questions_resolve_create) | **POST** /api2/questions/{id}/resolve/ | 
[**questions_retrieve**](QuestionsApi.md#questions_retrieve) | **GET** /api2/questions/{id}/ | 
[**questions_show_community_create**](QuestionsApi.md#questions_show_community_create) | **POST** /api2/questions/{id}/show-community/ | 
[**questions_update**](QuestionsApi.md#questions_update) | **PUT** /api2/questions/{id}/ | 
[**questions_view_metaculus_prediction_create**](QuestionsApi.md#questions_view_metaculus_prediction_create) | **POST** /api2/questions/{id}/view-metaculus-prediction/ | 
[**questions_vote_create**](QuestionsApi.md#questions_vote_create) | **POST** /api2/questions/{id}/vote/ | 



## questions_add_consideration_vote_create

> questions_add_consideration_vote_create(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_boost_create

> crate::models::Boost questions_boost_create(id, boost)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**boost** | [**Boost**](Boost.md) |  | [required] |

### Return type

[**crate::models::Boost**](Boost.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_bulk_predict_create

> crate::models::BulkPredictionInput questions_bulk_predict_create(bulk_prediction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_prediction_input** | [**BulkPredictionInput**](BulkPredictionInput.md) |  | [required] |

### Return type

[**crate::models::BulkPredictionInput**](BulkPredictionInput.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_create

> crate::models::QuestionUpdate questions_create(question_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**question_update** | Option<[**QuestionUpdate**](QuestionUpdate.md)> |  |  |

### Return type

[**crate::models::QuestionUpdate**](QuestionUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_destroy

> questions_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_list

> crate::models::PaginatedQuestionUserList questions_list(access, author, categories, close_time__gt, close_time__lt, commented_by, contest, forecast_type, group, guessed_by, has_group, include_description, limit, not_guessed_by, offset, order_by, project, publish_time__gt, publish_time__lt, resolve_time__gt, resolve_time__lt, reversed_related, search, status, r#type, unconditional, upvoted_by, username, visible_from_project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access** | Option<**String**> |  |  |
**author** | Option<**i32**> |  |  |
**categories** | Option<**String**> |  |  |
**close_time__gt** | Option<**String**> |  |  |
**close_time__lt** | Option<**String**> |  |  |
**commented_by** | Option<**f32**> |  |  |
**contest** | Option<**String**> |  |  |
**forecast_type** | Option<**String**> |  |  |
**group** | Option<**i32**> |  |  |
**guessed_by** | Option<**f32**> |  |  |
**has_group** | Option<**bool**> |  |  |
**include_description** | Option<**String**> | Set to 'true' to include the description (and categories) in responses |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**not_guessed_by** | Option<**f32**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order_by** | Option<**String**> | Which field to use when ordering the results. |  |
**project** | Option<**String**> |  |  |
**publish_time__gt** | Option<**String**> |  |  |
**publish_time__lt** | Option<**String**> |  |  |
**resolve_time__gt** | Option<**String**> |  |  |
**resolve_time__lt** | Option<**String**> |  |  |
**reversed_related** | Option<**f32**> |  |  |
**search** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |
**unconditional** | Option<**bool**> |  |  |
**upvoted_by** | Option<**f32**> |  |  |
**username** | Option<**String**> |  |  |
**visible_from_project** | Option<**String**> |  |  |

### Return type

[**crate::models::PaginatedQuestionUserList**](PaginatedQuestionUserList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_partial_update

> crate::models::QuestionUpdate questions_partial_update(id, patched_question_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**patched_question_update** | Option<[**PatchedQuestionUpdate**](PatchedQuestionUpdate.md)> |  |  |

### Return type

[**crate::models::QuestionUpdate**](QuestionUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_predict_create

> crate::models::PredictionInput questions_predict_create(id, prediction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**prediction_input** | [**PredictionInput**](PredictionInput.md) |  | [required] |

### Return type

[**crate::models::PredictionInput**](PredictionInput.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_prediction_for_date_retrieve

> crate::models::PredictionForDate questions_prediction_for_date_retrieve(id)


This endpoint is used only for Tezos verification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |

### Return type

[**crate::models::PredictionForDate**](PredictionForDate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_prediction_history_retrieve

> questions_prediction_history_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_predictions_retrieve

> questions_predictions_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_question_sharing_create

> crate::models::ShareQuestion questions_question_sharing_create(id, username, share_question)


This endpoint is used for sharing Private and Draft questions with other users. If sharing private question - invited user will receive \"predictor\" permissions. If sharing draft question - invited user will receive \"coauthor\" permissions\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**username** | **String** |  | [required] |
**share_question** | [**ShareQuestion**](ShareQuestion.md) |  | [required] |

### Return type

[**crate::models::ShareQuestion**](ShareQuestion.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_question_sharing_destroy

> questions_question_sharing_destroy(id, username)


This endpoint is used for sharing Private and Draft questions with other users. If sharing private question - invited user will receive \"predictor\" permissions. If sharing draft question - invited user will receive \"coauthor\" permissions\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**username** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_remove_consideration_vote_create

> questions_remove_consideration_vote_create(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_resolve_create

> crate::models::QuestionResolve questions_resolve_create(id, question_resolve)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**question_resolve** | [**QuestionResolve**](QuestionResolve.md) |  | [required] |

### Return type

[**crate::models::QuestionResolve**](QuestionResolve.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_retrieve

> crate::models::QuestionUserDetail questions_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |

### Return type

[**crate::models::QuestionUserDetail**](QuestionUserDetail.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_show_community_create

> crate::models::ShowCommunity questions_show_community_create(id, show_community)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**show_community** | [**ShowCommunity**](ShowCommunity.md) |  | [required] |

### Return type

[**crate::models::ShowCommunity**](ShowCommunity.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_update

> crate::models::QuestionUpdate questions_update(id, question_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**question_update** | Option<[**QuestionUpdate**](QuestionUpdate.md)> |  |  |

### Return type

[**crate::models::QuestionUpdate**](QuestionUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_view_metaculus_prediction_create

> crate::models::Question questions_view_metaculus_prediction_create(id, question)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**question** | [**Question**](Question.md) |  | [required] |

### Return type

[**crate::models::Question**](Question.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## questions_vote_create

> crate::models::QuestionVote questions_vote_create(id, question_vote)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this question. | [required] |
**question_vote** | [**QuestionVote**](QuestionVote.md) |  | [required] |

### Return type

[**crate::models::QuestionVote**](QuestionVote.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

