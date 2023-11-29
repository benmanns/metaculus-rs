# \CategoriesApi

All URIs are relative to *https://www.metaculus.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**categories_list**](CategoriesApi.md#categories_list) | **GET** /api2/categories/ | 
[**categories_retrieve**](CategoriesApi.md#categories_retrieve) | **GET** /api2/categories/{bare_id}/ | 



## categories_list

> crate::models::PaginatedCategoryList categories_list(id, long_name, short_name, limit, offset, url)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**long_name** | **String** |  | [required] |
**short_name** | **String** |  | [required] |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**url** | Option<**String**> |  |  |

### Return type

[**crate::models::PaginatedCategoryList**](PaginatedCategoryList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## categories_retrieve

> crate::models::Category categories_retrieve(bare_id, id, long_name, short_name, url)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bare_id** | **i32** |  | [required] |
**id** | **String** |  | [required] |
**long_name** | **String** |  | [required] |
**short_name** | **String** |  | [required] |
**url** | Option<**String**> |  |  |

### Return type

[**crate::models::Category**](Category.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

