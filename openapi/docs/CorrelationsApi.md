# \CorrelationsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_correlation**](CorrelationsApi.md#create_correlation) | **POST** /datasources/uid/{sourceUID}/correlations | Add correlation.
[**delete_correlation**](CorrelationsApi.md#delete_correlation) | **DELETE** /datasources/uid/{uid}/correlations/{correlationUID} | Delete a correlation.
[**get_correlation**](CorrelationsApi.md#get_correlation) | **GET** /datasources/uid/{sourceUID}/correlations/{correlationUID} | Gets a correlation.
[**get_correlations**](CorrelationsApi.md#get_correlations) | **GET** /datasources/correlations | Gets all correlations.
[**get_correlations_by_source_uid**](CorrelationsApi.md#get_correlations_by_source_uid) | **GET** /datasources/uid/{sourceUID}/correlations | Gets all correlations originating from the given data source.
[**update_correlation**](CorrelationsApi.md#update_correlation) | **PATCH** /datasources/uid/{sourceUID}/correlations/{correlationUID} | Updates a correlation.



## create_correlation

> models::CreateCorrelationResponseBody create_correlation(source_uid, create_correlation_command)
Add correlation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_uid** | **String** |  | [required] |
**create_correlation_command** | [**CreateCorrelationCommand**](CreateCorrelationCommand.md) |  | [required] |

### Return type

[**models::CreateCorrelationResponseBody**](CreateCorrelationResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_correlation

> models::DeleteCorrelationResponseBody delete_correlation(uid, correlation_uid)
Delete a correlation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**correlation_uid** | **String** |  | [required] |

### Return type

[**models::DeleteCorrelationResponseBody**](DeleteCorrelationResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_correlation

> models::Correlation get_correlation(source_uid, correlation_uid)
Gets a correlation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_uid** | **String** |  | [required] |
**correlation_uid** | **String** |  | [required] |

### Return type

[**models::Correlation**](Correlation.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_correlations

> Vec<models::Correlation> get_correlations(limit, page, source_uid)
Gets all correlations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Limit the maximum number of correlations to return per page |  |[default to 100]
**page** | Option<**i64**> | Page index for starting fetching correlations |  |[default to 1]
**source_uid** | Option<[**Vec<String>**](String.md)> | Source datasource UID filter to be applied to correlations |  |

### Return type

[**Vec<models::Correlation>**](Correlation.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_correlations_by_source_uid

> Vec<models::Correlation> get_correlations_by_source_uid(source_uid)
Gets all correlations originating from the given data source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_uid** | **String** |  | [required] |

### Return type

[**Vec<models::Correlation>**](Correlation.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_correlation

> models::UpdateCorrelationResponseBody update_correlation(source_uid, correlation_uid, update_correlation_command)
Updates a correlation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_uid** | **String** |  | [required] |
**correlation_uid** | **String** |  | [required] |
**update_correlation_command** | Option<[**UpdateCorrelationCommand**](UpdateCorrelationCommand.md)> |  |  |

### Return type

[**models::UpdateCorrelationResponseBody**](UpdateCorrelationResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

