# \DashboardPublicApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_public_dashboard**](DashboardPublicApi.md#create_public_dashboard) | **POST** /dashboards/uid/{dashboardUid}/public-dashboards | 
[**delete_public_dashboard**](DashboardPublicApi.md#delete_public_dashboard) | **DELETE** /dashboards/uid/{dashboardUid}/public-dashboards/{uid} | 
[**get_public_annotations**](DashboardPublicApi.md#get_public_annotations) | **GET** /public/dashboards/{accessToken}/annotations | 
[**get_public_dashboard**](DashboardPublicApi.md#get_public_dashboard) | **GET** /dashboards/uid/{dashboardUid}/public-dashboards | 
[**list_public_dashboards**](DashboardPublicApi.md#list_public_dashboards) | **GET** /dashboards/public-dashboards | 
[**query_public_dashboard**](DashboardPublicApi.md#query_public_dashboard) | **POST** /public/dashboards/{accessToken}/panels/{panelId}/query | 
[**update_public_dashboard**](DashboardPublicApi.md#update_public_dashboard) | **PATCH** /dashboards/uid/{dashboardUid}/public-dashboards/{uid} | 
[**view_public_dashboard**](DashboardPublicApi.md#view_public_dashboard) | **GET** /public/dashboards/{accessToken} | 



## create_public_dashboard

> models::PublicDashboard create_public_dashboard(dashboard_uid, public_dashboard_dto)


Create public dashboard for a dashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_uid** | **String** |  | [required] |
**public_dashboard_dto** | [**PublicDashboardDto**](PublicDashboardDto.md) |  | [required] |

### Return type

[**models::PublicDashboard**](PublicDashboard.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_public_dashboard

> models::SuccessResponseBody delete_public_dashboard(dashboard_uid, uid)


Delete public dashboard for a dashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_uid** | **String** |  | [required] |
**uid** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_annotations

> Vec<models::AnnotationEvent> get_public_annotations(access_token)


Get annotations for a public dashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token** | **String** |  | [required] |

### Return type

[**Vec<models::AnnotationEvent>**](AnnotationEvent.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_dashboard

> models::PublicDashboard get_public_dashboard(dashboard_uid)


Get public dashboard by dashboardUid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_uid** | **String** |  | [required] |

### Return type

[**models::PublicDashboard**](PublicDashboard.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_public_dashboards

> models::PublicDashboardListResponseWithPagination list_public_dashboards()


Get list of public dashboards

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicDashboardListResponseWithPagination**](PublicDashboardListResponseWithPagination.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_public_dashboard

> models::QueryDataResponse query_public_dashboard(access_token, panel_id)


Get results for a given panel on a public dashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token** | **String** |  | [required] |
**panel_id** | **i64** |  | [required] |

### Return type

[**models::QueryDataResponse**](QueryDataResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_public_dashboard

> models::PublicDashboard update_public_dashboard(dashboard_uid, uid, public_dashboard_dto)


Update public dashboard for a dashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_uid** | **String** |  | [required] |
**uid** | **String** |  | [required] |
**public_dashboard_dto** | [**PublicDashboardDto**](PublicDashboardDto.md) |  | [required] |

### Return type

[**models::PublicDashboard**](PublicDashboard.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_public_dashboard

> models::DashboardFullWithMeta view_public_dashboard(access_token)


Get public dashboard for view

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token** | **String** |  | [required] |

### Return type

[**models::DashboardFullWithMeta**](DashboardFullWithMeta.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

