# \OrganizationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organizations_list**](OrganizationsApi.md#organizations_list) | **GET** /api2/organizations/ | 
[**organizations_members_create**](OrganizationsApi.md#organizations_members_create) | **POST** /api2/organizations/{id}/members/ | 
[**organizations_members_create2**](OrganizationsApi.md#organizations_members_create2) | **POST** /api2/organizations/{id}/members/{user_id}/ | 
[**organizations_members_destroy**](OrganizationsApi.md#organizations_members_destroy) | **DELETE** /api2/organizations/{id}/members/{user_id}/ | 
[**organizations_members_partial_update**](OrganizationsApi.md#organizations_members_partial_update) | **PATCH** /api2/organizations/{id}/members/{user_id}/ | 
[**organizations_members_update**](OrganizationsApi.md#organizations_members_update) | **PUT** /api2/organizations/{id}/members/{user_id}/ | 
[**organizations_retrieve**](OrganizationsApi.md#organizations_retrieve) | **GET** /api2/organizations/{id}/ | 



## organizations_list

> crate::models::PaginatedOrganizationList organizations_list(page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |

### Return type

[**crate::models::PaginatedOrganizationList**](PaginatedOrganizationList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_members_create

> crate::models::Organization organizations_members_create(id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this organization. | [required] |
**organization** | [**Organization**](Organization.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_members_create2

> crate::models::Organization organizations_members_create2(id, user_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this organization. | [required] |
**user_id** | **String** |  | [required] |
**organization** | [**Organization**](Organization.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_members_destroy

> organizations_members_destroy(id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this organization. | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_members_partial_update

> crate::models::Organization organizations_members_partial_update(id, user_id, patched_organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this organization. | [required] |
**user_id** | **String** |  | [required] |
**patched_organization** | Option<[**PatchedOrganization**](PatchedOrganization.md)> |  |  |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_members_update

> crate::models::Organization organizations_members_update(id, user_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this organization. | [required] |
**user_id** | **String** |  | [required] |
**organization** | [**Organization**](Organization.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_retrieve

> crate::models::OrganizationDetail organizations_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this organization. | [required] |

### Return type

[**crate::models::OrganizationDetail**](OrganizationDetail.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

