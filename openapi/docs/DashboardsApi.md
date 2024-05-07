# \DashboardsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calculate_dashboard_diff**](DashboardsApi.md#calculate_dashboard_diff) | **POST** /dashboards/calculate-diff | Perform diff on two dashboards.
[**delete_dashboard_by_uid**](DashboardsApi.md#delete_dashboard_by_uid) | **DELETE** /dashboards/uid/{uid} | Delete dashboard by uid.
[**get_dashboard_by_uid**](DashboardsApi.md#get_dashboard_by_uid) | **GET** /dashboards/uid/{uid} | Get dashboard by uid.
[**get_dashboard_tags**](DashboardsApi.md#get_dashboard_tags) | **GET** /dashboards/tags | Get all dashboards tags of an organisation.
[**get_home_dashboard**](DashboardsApi.md#get_home_dashboard) | **GET** /dashboards/home | Get home dashboard.
[**import_dashboard**](DashboardsApi.md#import_dashboard) | **POST** /dashboards/import | Import dashboard.
[**post_dashboard**](DashboardsApi.md#post_dashboard) | **POST** /dashboards/db | Create / Update dashboard



## calculate_dashboard_diff

> Vec<i32> calculate_dashboard_diff(calculate_dashboard_diff_request)
Perform diff on two dashboards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calculate_dashboard_diff_request** | [**CalculateDashboardDiffRequest**](CalculateDashboardDiffRequest.md) |  | [required] |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_by_uid

> models::DeleteDashboardByUid200Response delete_dashboard_by_uid(uid)
Delete dashboard by uid.

Will delete the dashboard given the specified unique identifier (uid).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**models::DeleteDashboardByUid200Response**](deleteDashboardByUID_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_by_uid

> models::DashboardFullWithMeta get_dashboard_by_uid(uid)
Get dashboard by uid.

Will return the dashboard given the dashboard unique identifier (uid).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**models::DashboardFullWithMeta**](DashboardFullWithMeta.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_tags

> Vec<models::DashboardTagCloudItem> get_dashboard_tags()
Get all dashboards tags of an organisation.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DashboardTagCloudItem>**](DashboardTagCloudItem.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_home_dashboard

> models::GetHomeDashboardResponse get_home_dashboard()
Get home dashboard.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetHomeDashboardResponse**](GetHomeDashboardResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_dashboard

> models::ImportDashboardResponse import_dashboard(import_dashboard_request)
Import dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_dashboard_request** | [**ImportDashboardRequest**](ImportDashboardRequest.md) |  | [required] |

### Return type

[**models::ImportDashboardResponse**](ImportDashboardResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_dashboard

> models::PostDashboard200Response post_dashboard(save_dashboard_command)
Create / Update dashboard

Creates a new dashboard or updates an existing dashboard. Note: This endpoint is not intended for creating folders, use `POST /api/folders` for that.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_dashboard_command** | [**SaveDashboardCommand**](SaveDashboardCommand.md) |  | [required] |

### Return type

[**models::PostDashboard200Response**](postDashboard_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

