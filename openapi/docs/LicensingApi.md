# \LicensingApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_license_token**](LicensingApi.md#delete_license_token) | **DELETE** /licensing/token | Remove license from database.
[**get_custom_permissions_csv**](LicensingApi.md#get_custom_permissions_csv) | **GET** /licensing/custom-permissions-csv | Get custom permissions report in CSV format.
[**get_custom_permissions_report**](LicensingApi.md#get_custom_permissions_report) | **GET** /licensing/custom-permissions | Get custom permissions report.
[**get_license_token**](LicensingApi.md#get_license_token) | **GET** /licensing/token | Get license token.
[**get_status**](LicensingApi.md#get_status) | **GET** /licensing/check | Check license availability.
[**post_license_token**](LicensingApi.md#post_license_token) | **POST** /licensing/token | Create license token.
[**post_renew_license_token**](LicensingApi.md#post_renew_license_token) | **POST** /licensing/token/renew | Manually force license refresh.
[**refresh_license_stats**](LicensingApi.md#refresh_license_stats) | **GET** /licensing/refresh-stats | Refresh license stats.



## delete_license_token

> models::ErrorResponseBody delete_license_token(delete_token_command)
Remove license from database.

Removes the license stored in the Grafana database. Available in Grafana Enterprise v7.4+.  You need to have a permission with action `licensing:delete`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_token_command** | [**DeleteTokenCommand**](DeleteTokenCommand.md) |  | [required] |

### Return type

[**models::ErrorResponseBody**](ErrorResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_permissions_csv

> get_custom_permissions_csv()
Get custom permissions report in CSV format.

You need to have a permission with action `licensing.reports:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_permissions_report

> get_custom_permissions_report()
Get custom permissions report.

You need to have a permission with action `licensing.reports:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_token

> models::Token get_license_token()
Get license token.

You need to have a permission with action `licensing:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Token**](Token.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status

> get_status()
Check license availability.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_license_token

> models::Token post_license_token(delete_token_command)
Create license token.

You need to have a permission with action `licensing:update`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_token_command** | [**DeleteTokenCommand**](DeleteTokenCommand.md) |  | [required] |

### Return type

[**models::Token**](Token.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_renew_license_token

> post_renew_license_token(body)
Manually force license refresh.

Manually ask license issuer for a new token. Available in Grafana Enterprise v7.4+.  You need to have a permission with action `licensing:update`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_license_stats

> models::ActiveUserStats refresh_license_stats()
Refresh license stats.

You need to have a permission with action `licensing:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ActiveUserStats**](ActiveUserStats.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

