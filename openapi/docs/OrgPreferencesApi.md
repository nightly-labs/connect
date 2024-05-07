# \OrgPreferencesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_org_preferences**](OrgPreferencesApi.md#get_org_preferences) | **GET** /org/preferences | Get Current Org Prefs.
[**patch_org_preferences**](OrgPreferencesApi.md#patch_org_preferences) | **PATCH** /org/preferences | Patch Current Org Prefs.
[**update_org_preferences**](OrgPreferencesApi.md#update_org_preferences) | **PUT** /org/preferences | Update Current Org Prefs.



## get_org_preferences

> models::Preferences get_org_preferences()
Get Current Org Prefs.

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


## patch_org_preferences

> models::SuccessResponseBody patch_org_preferences(patch_prefs_cmd)
Patch Current Org Prefs.

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


## update_org_preferences

> models::SuccessResponseBody update_org_preferences(update_prefs_cmd)
Update Current Org Prefs.

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

