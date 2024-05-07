# \DashboardVersionsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dashboard_version_by_id**](DashboardVersionsApi.md#get_dashboard_version_by_id) | **GET** /dashboards/id/{DashboardID}/versions/{DashboardVersionID} | Get a specific dashboard version.
[**get_dashboard_version_by_uid**](DashboardVersionsApi.md#get_dashboard_version_by_uid) | **GET** /dashboards/uid/{uid}/versions/{DashboardVersionID} | Get a specific dashboard version using UID.
[**get_dashboard_versions_by_id**](DashboardVersionsApi.md#get_dashboard_versions_by_id) | **GET** /dashboards/id/{DashboardID}/versions | Gets all existing versions for the dashboard.
[**get_dashboard_versions_by_uid**](DashboardVersionsApi.md#get_dashboard_versions_by_uid) | **GET** /dashboards/uid/{uid}/versions | Gets all existing versions for the dashboard using UID.
[**restore_dashboard_version_by_id**](DashboardVersionsApi.md#restore_dashboard_version_by_id) | **POST** /dashboards/id/{DashboardID}/restore | Restore a dashboard to a given dashboard version.
[**restore_dashboard_version_by_uid**](DashboardVersionsApi.md#restore_dashboard_version_by_uid) | **POST** /dashboards/uid/{uid}/restore | Restore a dashboard to a given dashboard version using UID.



## get_dashboard_version_by_id

> models::DashboardVersionMeta get_dashboard_version_by_id(dashboard_id, dashboard_version_id)
Get a specific dashboard version.

Please refer to [updated API](#/dashboard_versions/getDashboardVersionByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** |  | [required] |
**dashboard_version_id** | **i64** |  | [required] |

### Return type

[**models::DashboardVersionMeta**](DashboardVersionMeta.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_version_by_uid

> models::DashboardVersionMeta get_dashboard_version_by_uid(dashboard_version_id, uid)
Get a specific dashboard version using UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_version_id** | **i64** |  | [required] |
**uid** | **String** |  | [required] |

### Return type

[**models::DashboardVersionMeta**](DashboardVersionMeta.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_versions_by_id

> Vec<models::DashboardVersionMeta> get_dashboard_versions_by_id(dashboard_id)
Gets all existing versions for the dashboard.

Please refer to [updated API](#/dashboard_versions/getDashboardVersionsByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** |  | [required] |

### Return type

[**Vec<models::DashboardVersionMeta>**](DashboardVersionMeta.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_versions_by_uid

> Vec<models::DashboardVersionMeta> get_dashboard_versions_by_uid(uid, limit, start)
Gets all existing versions for the dashboard using UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**limit** | Option<**i64**> | Maximum number of results to return |  |[default to 0]
**start** | Option<**i64**> | Version to start from when returning queries |  |[default to 0]

### Return type

[**Vec<models::DashboardVersionMeta>**](DashboardVersionMeta.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_dashboard_version_by_id

> models::PostDashboard200Response restore_dashboard_version_by_id(dashboard_id, restore_dashboard_version_command)
Restore a dashboard to a given dashboard version.

Please refer to [updated API](#/dashboard_versions/restoreDashboardVersionByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** |  | [required] |
**restore_dashboard_version_command** | [**RestoreDashboardVersionCommand**](RestoreDashboardVersionCommand.md) |  | [required] |

### Return type

[**models::PostDashboard200Response**](postDashboard_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_dashboard_version_by_uid

> models::PostDashboard200Response restore_dashboard_version_by_uid(uid, restore_dashboard_version_command)
Restore a dashboard to a given dashboard version using UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**restore_dashboard_version_command** | [**RestoreDashboardVersionCommand**](RestoreDashboardVersionCommand.md) |  | [required] |

### Return type

[**models::PostDashboard200Response**](postDashboard_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

