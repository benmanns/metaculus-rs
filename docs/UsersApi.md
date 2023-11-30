# \UsersApi

All URIs are relative to *https://www.metaculus.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_collect_tachyons_create**](UsersApi.md#users_collect_tachyons_create) | **POST** /api2/users/{id}/collect-tachyons/ | 
[**users_global_cp_reminder_create**](UsersApi.md#users_global_cp_reminder_create) | **POST** /api2/users/global-cp-reminder/ | 
[**users_global_cp_reminder_retrieve**](UsersApi.md#users_global_cp_reminder_retrieve) | **GET** /api2/users/global-cp-reminder/ | 
[**users_list**](UsersApi.md#users_list) | **GET** /api2/users/ | 
[**users_partial_update**](UsersApi.md#users_partial_update) | **PATCH** /api2/users/{id}/ | 
[**users_purchase_track_record_create**](UsersApi.md#users_purchase_track_record_create) | **POST** /api2/users/{id}/purchase-track-record/ | 
[**users_retrieve**](UsersApi.md#users_retrieve) | **GET** /api2/users/{id}/ | 
[**users_unlock_power_create**](UsersApi.md#users_unlock_power_create) | **POST** /api2/users/{id}/unlock-power/ | 
[**users_update**](UsersApi.md#users_update) | **PUT** /api2/users/{id}/ | 



## users_collect_tachyons_create

> crate::models::User users_collect_tachyons_create(id, user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_global_cp_reminder_create

> crate::models::User users_global_cp_reminder_create(first_name, last_name, ask_when_reaffirm_question_modal, date_joined, default_community_visibility, default_mp_visibility, email, formerly_known_as, id, is_staff, is_superuser, last_visited, level, level_title, permissions, powers, purchasable_track_record, score, show_profile_comments, supporter_level, supporter_since, tachyons, url, username, username_change_cost, user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**first_name** | **String** |  | [required] |
**last_name** | **String** |  | [required] |
**ask_when_reaffirm_question_modal** | Option<**bool**> |  |  |
**date_joined** | Option<**String**> |  |  |
**default_community_visibility** | Option<**i32**> |  |  |
**default_mp_visibility** | Option<**i32**> |  |  |
**email** | Option<**String**> |  |  |
**formerly_known_as** | Option<**String**> |  |  |
**id** | Option<**i32**> |  |  |
**is_staff** | Option<**bool**> |  |  |
**is_superuser** | Option<**bool**> |  |  |
**last_visited** | Option<**String**> |  |  |
**level** | Option<**i32**> |  |  |
**level_title** | Option<**String**> |  |  |
**permissions** | Option<[**::std::collections::HashMap<String, bool>**](bool.md)> |  |  |
**powers** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  |  |
**purchasable_track_record** | Option<**bool**> |  |  |
**score** | Option<**i32**> |  |  |
**show_profile_comments** | Option<**bool**> |  |  |
**supporter_level** | Option<**i32**> |  |  |
**supporter_since** | Option<**String**> |  |  |
**tachyons** | Option<**i32**> |  |  |
**url** | Option<**String**> |  |  |
**username** | Option<**String**> |  |  |
**username_change_cost** | Option<**i32**> |  |  |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_global_cp_reminder_retrieve

> crate::models::User users_global_cp_reminder_retrieve()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> crate::models::PaginatedUserList users_list(page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**crate::models::PaginatedUserList**](PaginatedUserList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_partial_update

> crate::models::User users_partial_update(id, patched_user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_user** | Option<[**PatchedUser**](PatchedUser.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_purchase_track_record_create

> crate::models::User users_purchase_track_record_create(id, user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_retrieve

> crate::models::User users_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_unlock_power_create

> crate::models::User users_unlock_power_create(id, user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update

> crate::models::User users_update(id, user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

