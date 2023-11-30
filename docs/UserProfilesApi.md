# \UserProfilesApi

All URIs are relative to *https://www.metaculus.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_profiles_list**](UserProfilesApi.md#user_profiles_list) | **GET** /api2/user-profiles/ | 
[**user_profiles_partial_update**](UserProfilesApi.md#user_profiles_partial_update) | **PATCH** /api2/user-profiles/{id}/ | 
[**user_profiles_retrieve**](UserProfilesApi.md#user_profiles_retrieve) | **GET** /api2/user-profiles/{id}/ | 
[**user_profiles_update**](UserProfilesApi.md#user_profiles_update) | **PUT** /api2/user-profiles/{id}/ | 



## user_profiles_list

> crate::models::PaginatedUserProfileList user_profiles_list(page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**crate::models::PaginatedUserProfileList**](PaginatedUserProfileList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profiles_partial_update

> crate::models::UserProfile user_profiles_partial_update(id, patched_user_profile)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_user_profile** | Option<[**PatchedUserProfile**](PatchedUserProfile.md)> |  |  |

### Return type

[**crate::models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profiles_retrieve

> crate::models::UserProfile user_profiles_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profiles_update

> crate::models::UserProfile user_profiles_update(id, user_profile)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**user_profile** | Option<[**UserProfile**](UserProfile.md)> |  |  |

### Return type

[**crate::models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

