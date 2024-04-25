# \SamlApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_metadata**](SamlApi.md#get_metadata) | **GET** /saml/metadata | It exposes the SP (Grafana's) metadata for the IdP's consumption.
[**get_saml_logout**](SamlApi.md#get_saml_logout) | **GET** /logout/saml | GetLogout initiates single logout process.
[**get_slo**](SamlApi.md#get_slo) | **GET** /saml/slo | It performs Single Logout (SLO) callback.
[**post_acs**](SamlApi.md#post_acs) | **POST** /saml/acs | It performs Assertion Consumer Service (ACS).
[**post_slo**](SamlApi.md#post_slo) | **POST** /saml/slo | It performs Single Logout (SLO) callback.



## get_metadata

> Vec<i32> get_metadata()
It exposes the SP (Grafana's) metadata for the IdP's consumption.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saml_logout

> get_saml_logout()
GetLogout initiates single logout process.

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


## get_slo

> get_slo()
It performs Single Logout (SLO) callback.

There might be two possible requests: 1. Logout response (callback) when Grafana initiates single logout and IdP returns response to logout request. 2. Logout request when another SP initiates single logout and IdP sends logout request to the Grafana, or in case of IdP-initiated logout.

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


## post_acs

> post_acs(relay_state)
It performs Assertion Consumer Service (ACS).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**relay_state** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_slo

> post_slo(saml_request, saml_response)
It performs Single Logout (SLO) callback.

There might be two possible requests: 1. Logout response (callback) when Grafana initiates single logout and IdP returns response to logout request. 2. Logout request when another SP initiates single logout and IdP sends logout request to the Grafana, or in case of IdP-initiated logout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_request** | Option<**String**> |  |  |
**saml_response** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

