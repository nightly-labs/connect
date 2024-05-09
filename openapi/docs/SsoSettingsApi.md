# \SsoSettingsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_provider_settings**](SsoSettingsApi.md#get_provider_settings) | **GET** /v1/sso-settings/{key} | Get an SSO Settings entry by Key
[**list_all_providers_settings**](SsoSettingsApi.md#list_all_providers_settings) | **GET** /v1/sso-settings | List all SSO Settings entries
[**remove_provider_settings**](SsoSettingsApi.md#remove_provider_settings) | **DELETE** /v1/sso-settings/{key} | Remove SSO Settings
[**update_provider_settings**](SsoSettingsApi.md#update_provider_settings) | **PUT** /v1/sso-settings/{key} | Update SSO Settings



## get_provider_settings

> models::ListAllProvidersSettings200ResponseInner get_provider_settings(key)
Get an SSO Settings entry by Key

You need to have a permission with action `settings:read` with scope `settings:auth.<provider>:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**models::ListAllProvidersSettings200ResponseInner**](listAllProvidersSettings_200_response_inner.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_providers_settings

> Vec<models::ListAllProvidersSettings200ResponseInner> list_all_providers_settings()
List all SSO Settings entries

You need to have a permission with action `settings:read` with scope `settings:auth.<provider>:*`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ListAllProvidersSettings200ResponseInner>**](listAllProvidersSettings_200_response_inner.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_provider_settings

> models::SuccessResponseBody remove_provider_settings(key)
Remove SSO Settings

Removes the SSO Settings for a provider.  You need to have a permission with action `settings:write` and scope `settings:auth.<provider>:*`.

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


## update_provider_settings

> models::SuccessResponseBody update_provider_settings(key, update_provider_settings_request)
Update SSO Settings

Inserts or updates the SSO Settings for a provider.  You need to have a permission with action `settings:write` and scope `settings:auth.<provider>:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**update_provider_settings_request** | [**UpdateProviderSettingsRequest**](UpdateProviderSettingsRequest.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

