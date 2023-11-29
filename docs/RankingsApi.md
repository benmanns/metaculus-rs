# \RankingsApi

All URIs are relative to *https://www.metaculus.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rankings_list**](RankingsApi.md#rankings_list) | **GET** /api2/rankings/ | 
[**rankings_retrieve**](RankingsApi.md#rankings_retrieve) | **GET** /api2/rankings/{id}/ | 



## rankings_list

> crate::models::PaginatedRankingList rankings_list(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::PaginatedRankingList**](PaginatedRankingList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rankings_retrieve

> crate::models::Ranking rankings_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this site-specific user data. | [required] |

### Return type

[**crate::models::Ranking**](Ranking.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

