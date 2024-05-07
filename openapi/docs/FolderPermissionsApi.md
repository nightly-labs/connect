# \FolderPermissionsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_folder_permission_list**](FolderPermissionsApi.md#get_folder_permission_list) | **GET** /folders/{folder_uid}/permissions | Gets all existing permissions for the folder with the given `uid`.
[**update_folder_permissions**](FolderPermissionsApi.md#update_folder_permissions) | **POST** /folders/{folder_uid}/permissions | Updates permissions for a folder. This operation will remove existing permissions if they’re not included in the request.



## get_folder_permission_list

> Vec<models::DashboardAclInfoDto> get_folder_permission_list(folder_uid)
Gets all existing permissions for the folder with the given `uid`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |

### Return type

[**Vec<models::DashboardAclInfoDto>**](DashboardACLInfoDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder_permissions

> models::SuccessResponseBody update_folder_permissions(folder_uid, update_dashboard_acl_command)
Updates permissions for a folder. This operation will remove existing permissions if they’re not included in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**update_dashboard_acl_command** | [**UpdateDashboardAclCommand**](UpdateDashboardAclCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

