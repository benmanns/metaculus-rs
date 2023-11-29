# \UserProfilesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_profiles_list**](UserProfilesApi.md#user_profiles_list) | **GET** /api2/user-profiles/ | 
[**user_profiles_partial_update**](UserProfilesApi.md#user_profiles_partial_update) | **PATCH** /api2/user-profiles/{id}/ | 
[**user_profiles_retrieve**](UserProfilesApi.md#user_profiles_retrieve) | **GET** /api2/user-profiles/{id}/ | 
[**user_profiles_update**](UserProfilesApi.md#user_profiles_update) | **PUT** /api2/user-profiles/{id}/ | 



## user_profiles_list

> crate::models::PaginatedUserProfileList user_profiles_list(first_name, id2, last_name, ask_when_reaffirm_question_modal, date_joined, default_community_visibility, default_mp_visibility, email, formerly_known_as, id, is_staff, is_superuser, last_visited, level, level_title, page, permissions, powers, purchasable_track_record, score, show_profile_comments, supporter_level, supporter_since, tachyons, url, username, username_change_cost)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**first_name** | **String** |  | [required] |
**id2** | **i32** |  | [required] |
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
**page** | Option<**i32**> | A page number within the paginated result set. |  |
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

### Return type

[**crate::models::PaginatedUserProfileList**](PaginatedUserProfileList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profiles_partial_update

> crate::models::UserProfile user_profiles_partial_update(first_name, id2, last_name, ask_when_reaffirm_question_modal, date_joined, default_community_visibility, default_mp_visibility, email, formerly_known_as, id, is_staff, is_superuser, last_visited, level, level_title, permissions, powers, purchasable_track_record, score, show_profile_comments, supporter_level, supporter_since, tachyons, url, username, username_change_cost, patched_user_profile)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**first_name** | **String** |  | [required] |
**id2** | **i32** |  | [required] |
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

> crate::models::UserProfile user_profiles_retrieve(first_name, id2, last_name, ask_when_reaffirm_question_modal, date_joined, default_community_visibility, default_mp_visibility, email, formerly_known_as, id, is_staff, is_superuser, last_visited, level, level_title, permissions, powers, purchasable_track_record, score, show_profile_comments, supporter_level, supporter_since, tachyons, url, username, username_change_cost)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**first_name** | **String** |  | [required] |
**id2** | **i32** |  | [required] |
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

### Return type

[**crate::models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profiles_update

> crate::models::UserProfile user_profiles_update(first_name, id2, last_name, ask_when_reaffirm_question_modal, date_joined, default_community_visibility, default_mp_visibility, email, formerly_known_as, id, is_staff, is_superuser, last_visited, level, level_title, permissions, powers, purchasable_track_record, score, show_profile_comments, supporter_level, supporter_since, tachyons, url, username, username_change_cost, user_profile)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**first_name** | **String** |  | [required] |
**id2** | **i32** |  | [required] |
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
**user_profile** | Option<[**UserProfile**](UserProfile.md)> |  |  |

### Return type

[**crate::models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

