# \AdminApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_get_settings**](AdminApi.md#admin_get_settings) | **GET** /admin/settings | Fetch settings.
[**admin_get_stats**](AdminApi.md#admin_get_stats) | **GET** /admin/stats | Fetch Grafana Stats.



## admin_get_settings

> std::collections::HashMap<String, std::collections::HashMap<String, String>> admin_get_settings()
Fetch settings.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `settings:read` and scopes: `settings:*`, `settings:auth.saml:` and `settings:auth.saml:enabled` (property level).

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, std::collections::HashMap<String, String>>**](std::collections::HashMap.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_get_stats

> models::AdminStats admin_get_stats()
Fetch Grafana Stats.

Only works with Basic Authentication (username and password). See introduction for an explanation. If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `server:stats:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AdminStats**](AdminStats.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

