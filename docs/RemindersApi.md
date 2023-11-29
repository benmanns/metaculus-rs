# \RemindersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reminders_create**](RemindersApi.md#reminders_create) | **POST** /api2/reminders/ | 
[**reminders_list**](RemindersApi.md#reminders_list) | **GET** /api2/reminders/ | 
[**reminders_partial_update**](RemindersApi.md#reminders_partial_update) | **PATCH** /api2/reminders/{id}/ | 
[**reminders_retrieve**](RemindersApi.md#reminders_retrieve) | **GET** /api2/reminders/{id}/ | 
[**reminders_update**](RemindersApi.md#reminders_update) | **PUT** /api2/reminders/{id}/ | 



## reminders_create

> crate::models::Reminder reminders_create(reminder)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reminder** | [**Reminder**](Reminder.md) |  | [required] |

### Return type

[**crate::models::Reminder**](Reminder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_list

> crate::models::PaginatedReminderList reminders_list(page, question, status)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**question** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |

### Return type

[**crate::models::PaginatedReminderList**](PaginatedReminderList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_partial_update

> crate::models::Reminder reminders_partial_update(id, patched_reminder)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this reminder. | [required] |
**patched_reminder** | Option<[**PatchedReminder**](PatchedReminder.md)> |  |  |

### Return type

[**crate::models::Reminder**](Reminder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_retrieve

> crate::models::Reminder reminders_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this reminder. | [required] |

### Return type

[**crate::models::Reminder**](Reminder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_update

> crate::models::Reminder reminders_update(id, reminder)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this reminder. | [required] |
**reminder** | [**Reminder**](Reminder.md) |  | [required] |

### Return type

[**crate::models::Reminder**](Reminder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

