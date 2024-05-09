# \SigningKeysApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_jwks**](SigningKeysApi.md#retrieve_jwks) | **GET** /signing-keys/keys | Get JSON Web Key Set (JWKS) with all the keys that can be used to verify tokens (public keys)



## retrieve_jwks

> models::RetrieveJwks200Response retrieve_jwks()
Get JSON Web Key Set (JWKS) with all the keys that can be used to verify tokens (public keys)

Required permissions None

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RetrieveJwks200Response**](retrieveJWKS_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

