# \DashboardPermissionsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dashboard_permissions_list_by_id**](DashboardPermissionsApi.md#get_dashboard_permissions_list_by_id) | **GET** /dashboards/id/{DashboardID}/permissions | Gets all existing permissions for the given dashboard.
[**get_dashboard_permissions_list_by_uid**](DashboardPermissionsApi.md#get_dashboard_permissions_list_by_uid) | **GET** /dashboards/uid/{uid}/permissions | Gets all existing permissions for the given dashboard.
[**update_dashboard_permissions_by_id**](DashboardPermissionsApi.md#update_dashboard_permissions_by_id) | **POST** /dashboards/id/{DashboardID}/permissions | Updates permissions for a dashboard.
[**update_dashboard_permissions_by_uid**](DashboardPermissionsApi.md#update_dashboard_permissions_by_uid) | **POST** /dashboards/uid/{uid}/permissions | Updates permissions for a dashboard.



## get_dashboard_permissions_list_by_id

> Vec<models::DashboardAclInfoDto> get_dashboard_permissions_list_by_id(dashboard_id)
Gets all existing permissions for the given dashboard.

Please refer to [updated API](#/dashboard_permissions/getDashboardPermissionsListByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** |  | [required] |

### Return type

[**Vec<models::DashboardAclInfoDto>**](DashboardACLInfoDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_permissions_list_by_uid

> Vec<models::DashboardAclInfoDto> get_dashboard_permissions_list_by_uid(uid)
Gets all existing permissions for the given dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**Vec<models::DashboardAclInfoDto>**](DashboardACLInfoDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_permissions_by_id

> models::SuccessResponseBody update_dashboard_permissions_by_id(dashboard_id, update_dashboard_acl_command)
Updates permissions for a dashboard.

Please refer to [updated API](#/dashboard_permissions/updateDashboardPermissionsByUID) instead  This operation will remove existing permissions if they’re not included in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** |  | [required] |
**update_dashboard_acl_command** | [**UpdateDashboardAclCommand**](UpdateDashboardAclCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_permissions_by_uid

> models::SuccessResponseBody update_dashboard_permissions_by_uid(uid, update_dashboard_acl_command)
Updates permissions for a dashboard.

This operation will remove existing permissions if they’re not included in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**update_dashboard_acl_command** | [**UpdateDashboardAclCommand**](UpdateDashboardAclCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

