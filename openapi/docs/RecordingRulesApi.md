# \RecordingRulesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_recording_rule**](RecordingRulesApi.md#create_recording_rule) | **POST** /recording-rules | Create a recording rule that is then registered and started.
[**create_recording_rule_write_target**](RecordingRulesApi.md#create_recording_rule_write_target) | **POST** /recording-rules/writer | Create a remote write target.
[**delete_recording_rule**](RecordingRulesApi.md#delete_recording_rule) | **DELETE** /recording-rules/{recordingRuleID} | Delete removes the rule from the registry and stops it.
[**delete_recording_rule_write_target**](RecordingRulesApi.md#delete_recording_rule_write_target) | **DELETE** /recording-rules/writer | Delete the remote write target.
[**get_recording_rule_write_target**](RecordingRulesApi.md#get_recording_rule_write_target) | **GET** /recording-rules/writer | Return the prometheus remote write target.
[**list_recording_rules**](RecordingRulesApi.md#list_recording_rules) | **GET** /recording-rules | Lists all rules in the database: active or deleted.
[**test_create_recording_rule**](RecordingRulesApi.md#test_create_recording_rule) | **POST** /recording-rules/test | Test a recording rule.
[**update_recording_rule**](RecordingRulesApi.md#update_recording_rule) | **PUT** /recording-rules | Update the active status of a rule.



## create_recording_rule

> models::RecordingRuleJson create_recording_rule(recording_rule_json)
Create a recording rule that is then registered and started.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_json** | [**RecordingRuleJson**](RecordingRuleJson.md) |  | [required] |

### Return type

[**models::RecordingRuleJson**](RecordingRuleJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_recording_rule_write_target

> models::PrometheusRemoteWriteTargetJson create_recording_rule_write_target(prometheus_remote_write_target_json)
Create a remote write target.

It returns a 422 if there is not an existing prometheus data source configured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prometheus_remote_write_target_json** | [**PrometheusRemoteWriteTargetJson**](PrometheusRemoteWriteTargetJson.md) |  | [required] |

### Return type

[**models::PrometheusRemoteWriteTargetJson**](PrometheusRemoteWriteTargetJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_rule

> models::SuccessResponseBody delete_recording_rule(recording_rule_id)
Delete removes the rule from the registry and stops it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_rule_write_target

> models::SuccessResponseBody delete_recording_rule_write_target()
Delete the remote write target.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_rule_write_target

> models::PrometheusRemoteWriteTargetJson get_recording_rule_write_target()
Return the prometheus remote write target.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PrometheusRemoteWriteTargetJson**](PrometheusRemoteWriteTargetJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recording_rules

> Vec<models::RecordingRuleJson> list_recording_rules()
Lists all rules in the database: active or deleted.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RecordingRuleJson>**](RecordingRuleJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_create_recording_rule

> models::SuccessResponseBody test_create_recording_rule(recording_rule_json)
Test a recording rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_json** | [**RecordingRuleJson**](RecordingRuleJson.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_recording_rule

> models::RecordingRuleJson update_recording_rule(recording_rule_json)
Update the active status of a rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_json** | [**RecordingRuleJson**](RecordingRuleJson.md) |  | [required] |

### Return type

[**models::RecordingRuleJson**](RecordingRuleJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

