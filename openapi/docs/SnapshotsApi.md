# \SnapshotsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dashboard_snapshot**](SnapshotsApi.md#create_dashboard_snapshot) | **POST** /snapshots | When creating a snapshot using the API, you have to provide the full dashboard payload including the snapshot data. This endpoint is designed for the Grafana UI.
[**delete_dashboard_snapshot**](SnapshotsApi.md#delete_dashboard_snapshot) | **DELETE** /snapshots/{key} | Delete Snapshot by Key.
[**delete_dashboard_snapshot_by_delete_key**](SnapshotsApi.md#delete_dashboard_snapshot_by_delete_key) | **GET** /snapshots-delete/{deleteKey} | Delete Snapshot by deleteKey.
[**get_dashboard_snapshot**](SnapshotsApi.md#get_dashboard_snapshot) | **GET** /snapshots/{key} | Get Snapshot by Key.
[**get_sharing_options**](SnapshotsApi.md#get_sharing_options) | **GET** /snapshot/shared-options | Get snapshot sharing settings.
[**search_dashboard_snapshots**](SnapshotsApi.md#search_dashboard_snapshots) | **GET** /dashboard/snapshots | List snapshots.



## create_dashboard_snapshot

> models::CreateDashboardSnapshot200Response create_dashboard_snapshot(create_dashboard_snapshot_command)
When creating a snapshot using the API, you have to provide the full dashboard payload including the snapshot data. This endpoint is designed for the Grafana UI.

Snapshot public mode should be enabled or authentication is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dashboard_snapshot_command** | [**CreateDashboardSnapshotCommand**](CreateDashboardSnapshotCommand.md) |  | [required] |

### Return type

[**models::CreateDashboardSnapshot200Response**](createDashboardSnapshot_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_snapshot

> models::SuccessResponseBody delete_dashboard_snapshot(key)
Delete Snapshot by Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_snapshot_by_delete_key

> models::SuccessResponseBody delete_dashboard_snapshot_by_delete_key(delete_key)
Delete Snapshot by deleteKey.

Snapshot public mode should be enabled or authentication is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_key** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_snapshot

> get_dashboard_snapshot(key)
Get Snapshot by Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sharing_options

> models::GetSharingOptions200Response get_sharing_options()
Get snapshot sharing settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSharingOptions200Response**](getSharingOptions_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_dashboard_snapshots

> Vec<models::DashboardSnapshotDto> search_dashboard_snapshots(query, limit)
List snapshots.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Search Query |  |
**limit** | Option<**i64**> | Limit the number of returned results |  |[default to 1000]

### Return type

[**Vec<models::DashboardSnapshotDto>**](DashboardSnapshotDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

