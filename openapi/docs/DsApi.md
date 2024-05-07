# \DsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**query_metrics_with_expressions**](DsApi.md#query_metrics_with_expressions) | **POST** /ds/query | DataSource query metrics with expressions.



## query_metrics_with_expressions

> models::QueryDataResponse query_metrics_with_expressions(metric_request)
DataSource query metrics with expressions.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:query`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_request** | [**MetricRequest**](MetricRequest.md) |  | [required] |

### Return type

[**models::QueryDataResponse**](QueryDataResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

