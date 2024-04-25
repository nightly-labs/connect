# \DevicesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_devices**](DevicesApi.md#list_devices) | **GET** /stats | Lists all devices within the last 30 days
[**search_devices**](DevicesApi.md#search_devices) | **POST** /search | Lists all devices within the last 30 days



## list_devices

> Vec<models::DeviceDto> list_devices()
Lists all devices within the last 30 days

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DeviceDto>**](deviceDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_devices

> models::SearchDeviceQueryResult search_devices()
Lists all devices within the last 30 days

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SearchDeviceQueryResult**](SearchDeviceQueryResult.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

