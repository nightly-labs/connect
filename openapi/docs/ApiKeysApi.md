# \ApiKeysApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_ap_ikey**](ApiKeysApi.md#add_ap_ikey) | **POST** /auth/keys | Creates an API key.
[**delete_ap_ikey**](ApiKeysApi.md#delete_ap_ikey) | **DELETE** /auth/keys/{id} | Delete API key.
[**get_ap_ikeys**](ApiKeysApi.md#get_ap_ikeys) | **GET** /auth/keys | Get auth keys.



## add_ap_ikey

> models::NewApiKeyResult add_ap_ikey(add_api_key_command)
Creates an API key.

Will return details of the created API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_api_key_command** | [**AddApiKeyCommand**](AddApiKeyCommand.md) |  | [required] |

### Return type

[**models::NewApiKeyResult**](NewApiKeyResult.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ap_ikey

> models::SuccessResponseBody delete_ap_ikey(id)
Delete API key.

Deletes an API key. Deprecated. See: https://grafana.com/docs/grafana/next/administration/api-keys/#migrate-api-keys-to-grafana-service-accounts-using-the-api.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ap_ikeys

> Vec<models::ApiKeyDto> get_ap_ikeys(include_expired)
Get auth keys.

Will return auth keys.  Deprecated: true.  Deprecated. Please use GET /api/serviceaccounts and GET /api/serviceaccounts/{id}/tokens instead see https://grafana.com/docs/grafana/next/administration/api-keys/#migrate-api-keys-to-grafana-service-accounts-using-the-api.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_expired** | Option<**bool**> | Show expired keys |  |[default to false]

### Return type

[**Vec<models::ApiKeyDto>**](ApiKeyDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

