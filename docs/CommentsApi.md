# \CommentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**comments_create**](CommentsApi.md#comments_create) | **POST** /api2/comments/ | 
[**comments_destroy**](CommentsApi.md#comments_destroy) | **DELETE** /api2/comments/{id}/ | 
[**comments_like_create**](CommentsApi.md#comments_like_create) | **POST** /api2/comments/{id}/like/ | 
[**comments_list**](CommentsApi.md#comments_list) | **GET** /api2/comments/ | 
[**comments_partial_update**](CommentsApi.md#comments_partial_update) | **PATCH** /api2/comments/{id}/ | 
[**comments_report_create**](CommentsApi.md#comments_report_create) | **POST** /api2/comments/{id}/report/ | 
[**comments_retrieve**](CommentsApi.md#comments_retrieve) | **GET** /api2/comments/{id}/ | 
[**comments_update**](CommentsApi.md#comments_update) | **PUT** /api2/comments/{id}/ | 



## comments_create

> crate::models::Comment comments_create(comment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment** | [**Comment**](Comment.md) |  | [required] |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_destroy

> comments_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this comment. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_like_create

> crate::models::Comment comments_like_create(id, comment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this comment. | [required] |
**comment** | [**Comment**](Comment.md) |  | [required] |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_list

> crate::models::PaginatedCommentList comments_list(author, created_time__gt, created_time__lt, cursor, id, limit, order_by, question)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**author** | Option<**i32**> |  |  |
**created_time__gt** | Option<**String**> |  |  |
**created_time__lt** | Option<**String**> |  |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**id** | Option<**i32**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**order_by** | Option<**String**> | Which field to use when ordering the results. |  |
**question** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedCommentList**](PaginatedCommentList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_partial_update

> crate::models::CommentUpdate comments_partial_update(id, patched_comment_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this comment. | [required] |
**patched_comment_update** | Option<[**PatchedCommentUpdate**](PatchedCommentUpdate.md)> |  |  |

### Return type

[**crate::models::CommentUpdate**](CommentUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_report_create

> crate::models::Comment comments_report_create(id, comment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this comment. | [required] |
**comment** | [**Comment**](Comment.md) |  | [required] |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_retrieve

> crate::models::CommentChildren comments_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this comment. | [required] |

### Return type

[**crate::models::CommentChildren**](CommentChildren.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_update

> crate::models::CommentUpdate comments_update(id, comment_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this comment. | [required] |
**comment_update** | [**CommentUpdate**](CommentUpdate.md) |  | [required] |

### Return type

[**crate::models::CommentUpdate**](CommentUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

