# \ProjectsApi

All URIs are relative to *https://www.metaculus.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_create**](ProjectsApi.md#projects_create) | **POST** /api2/projects/ | 
[**projects_follow_create**](ProjectsApi.md#projects_follow_create) | **POST** /api2/projects/{id}/follow/ | 
[**projects_invite_members_create**](ProjectsApi.md#projects_invite_members_create) | **POST** /api2/projects/{id}/invite-members/ | 
[**projects_is_following_retrieve**](ProjectsApi.md#projects_is_following_retrieve) | **GET** /api2/projects/{id}/is-following/ | 
[**projects_join_create**](ProjectsApi.md#projects_join_create) | **POST** /api2/projects/{id}/join/ | 
[**projects_leave_create**](ProjectsApi.md#projects_leave_create) | **POST** /api2/projects/{id}/leave/ | 
[**projects_list**](ProjectsApi.md#projects_list) | **GET** /api2/projects/ | 
[**projects_members_create**](ProjectsApi.md#projects_members_create) | **POST** /api2/projects/{id}/members/{user_id}/ | 
[**projects_members_destroy**](ProjectsApi.md#projects_members_destroy) | **DELETE** /api2/projects/{id}/members/{user_id}/ | 
[**projects_members_partial_update**](ProjectsApi.md#projects_members_partial_update) | **PATCH** /api2/projects/{id}/members/{user_id}/ | 
[**projects_members_update**](ProjectsApi.md#projects_members_update) | **PUT** /api2/projects/{id}/members/{user_id}/ | 
[**projects_partial_update**](ProjectsApi.md#projects_partial_update) | **PATCH** /api2/projects/{id}/ | 
[**projects_personal_stats_retrieve**](ProjectsApi.md#projects_personal_stats_retrieve) | **GET** /api2/projects/{id}/personal-stats/ | 
[**projects_register_create**](ProjectsApi.md#projects_register_create) | **POST** /api2/projects/{id}/register/ | 
[**projects_registered_retrieve**](ProjectsApi.md#projects_registered_retrieve) | **GET** /api2/projects/{id}/registered/ | 
[**projects_retrieve**](ProjectsApi.md#projects_retrieve) | **GET** /api2/projects/{id}/ | 
[**projects_unfollow_create**](ProjectsApi.md#projects_unfollow_create) | **POST** /api2/projects/{id}/unfollow/ | 
[**projects_update**](ProjectsApi.md#projects_update) | **PUT** /api2/projects/{id}/ | 



## projects_create

> crate::models::ProjectUpdate projects_create(project_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_update** | [**ProjectUpdate**](ProjectUpdate.md) |  | [required] |

### Return type

[**crate::models::ProjectUpdate**](ProjectUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_follow_create

> crate::models::Project projects_follow_create(id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_invite_members_create

> crate::models::Project projects_invite_members_create(id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_is_following_retrieve

> crate::models::Project projects_is_following_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_join_create

> crate::models::Project projects_join_create(id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_leave_create

> crate::models::Project projects_leave_create(id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list

> crate::models::PaginatedProjectList projects_list(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::PaginatedProjectList**](PaginatedProjectList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_members_create

> crate::models::Project projects_members_create(id, user_id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**user_id** | **String** |  | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_members_destroy

> projects_members_destroy(id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_members_partial_update

> crate::models::Project projects_members_partial_update(id, user_id, patched_project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**user_id** | **String** |  | [required] |
**patched_project** | Option<[**PatchedProject**](PatchedProject.md)> |  |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_members_update

> crate::models::Project projects_members_update(id, user_id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**user_id** | **String** |  | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_partial_update

> crate::models::Project projects_partial_update(id, patched_project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**patched_project** | Option<[**PatchedProject**](PatchedProject.md)> |  |  |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_personal_stats_retrieve

> crate::models::Project projects_personal_stats_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_register_create

> crate::models::Project projects_register_create(id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_registered_retrieve

> crate::models::Project projects_registered_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_retrieve

> crate::models::ProjectDetail projects_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |

### Return type

[**crate::models::ProjectDetail**](ProjectDetail.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_unfollow_create

> crate::models::Project projects_unfollow_create(id, project)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_update

> crate::models::ProjectUpdate projects_update(id, project_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this project. | [required] |
**project_update** | [**ProjectUpdate**](ProjectUpdate.md) |  | [required] |

### Return type

[**crate::models::ProjectUpdate**](ProjectUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

