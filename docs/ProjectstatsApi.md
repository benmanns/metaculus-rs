# \ProjectstatsApi

All URIs are relative to *https://www.metaculus.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projectstats_list**](ProjectstatsApi.md#projectstats_list) | **GET** /api2/projectstats/ | 
[**projectstats_retrieve**](ProjectstatsApi.md#projectstats_retrieve) | **GET** /api2/projectstats/{id}/ | 



## projectstats_list

> crate::models::PaginatedProjectUserStatsList projectstats_list(limit, offset, order_by, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order_by** | Option<**String**> | Which field to use when ordering the results. |  |
**project** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedProjectUserStatsList**](PaginatedProjectUserStatsList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projectstats_retrieve

> crate::models::ProjectUserStats projectstats_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project user stats. | [required] |

### Return type

[**crate::models::ProjectUserStats**](ProjectUserStats.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

