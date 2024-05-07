# \UserPreferencesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_preferences**](UserPreferencesApi.md#get_user_preferences) | **GET** /user/preferences | Get user preferences.
[**patch_user_preferences**](UserPreferencesApi.md#patch_user_preferences) | **PATCH** /user/preferences | Patch user preferences.
[**update_user_preferences**](UserPreferencesApi.md#update_user_preferences) | **PUT** /user/preferences | Update user preferences.



## get_user_preferences

> models::Preferences get_user_preferences()
Get user preferences.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Preferences**](Preferences.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_preferences

> models::SuccessResponseBody patch_user_preferences(patch_prefs_cmd)
Patch user preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_prefs_cmd** | [**PatchPrefsCmd**](PatchPrefsCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_preferences

> models::SuccessResponseBody update_user_preferences(update_prefs_cmd)
Update user preferences.

Omitting a key (`theme`, `homeDashboardId`, `timezone`) will cause the current value to be replaced with the system default value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_prefs_cmd** | [**UpdatePrefsCmd**](UpdatePrefsCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

