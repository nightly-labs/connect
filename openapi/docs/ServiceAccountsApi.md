# \ServiceAccountsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service_account**](ServiceAccountsApi.md#create_service_account) | **POST** /serviceaccounts | Create service account
[**create_token**](ServiceAccountsApi.md#create_token) | **POST** /serviceaccounts/{serviceAccountId}/tokens | CreateNewToken adds a token to a service account
[**delete_service_account**](ServiceAccountsApi.md#delete_service_account) | **DELETE** /serviceaccounts/{serviceAccountId} | Delete service account
[**delete_token**](ServiceAccountsApi.md#delete_token) | **DELETE** /serviceaccounts/{serviceAccountId}/tokens/{tokenId} | DeleteToken deletes service account tokens
[**list_tokens**](ServiceAccountsApi.md#list_tokens) | **GET** /serviceaccounts/{serviceAccountId}/tokens | Get service account tokens
[**retrieve_service_account**](ServiceAccountsApi.md#retrieve_service_account) | **GET** /serviceaccounts/{serviceAccountId} | Get single serviceaccount by Id
[**search_org_service_accounts_with_paging**](ServiceAccountsApi.md#search_org_service_accounts_with_paging) | **GET** /serviceaccounts/search | Search service accounts with paging
[**update_service_account**](ServiceAccountsApi.md#update_service_account) | **PATCH** /serviceaccounts/{serviceAccountId} | Update service account



## create_service_account

> models::ServiceAccountDto create_service_account(create_service_account_form)
Create service account

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:write` scope: `serviceaccounts:*`  Requires basic authentication and that the authenticated user is a Grafana Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_service_account_form** | Option<[**CreateServiceAccountForm**](CreateServiceAccountForm.md)> |  |  |

### Return type

[**models::ServiceAccountDto**](ServiceAccountDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_token

> models::NewApiKeyResult create_token(service_account_id, add_service_account_token_command)
CreateNewToken adds a token to a service account

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:write` scope: `serviceaccounts:id:1` (single service account)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** |  | [required] |
**add_service_account_token_command** | Option<[**AddServiceAccountTokenCommand**](AddServiceAccountTokenCommand.md)> |  |  |

### Return type

[**models::NewApiKeyResult**](NewApiKeyResult.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_account

> models::SuccessResponseBody delete_service_account(service_account_id)
Delete service account

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:delete` scope: `serviceaccounts:id:1` (single service account)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_token

> models::SuccessResponseBody delete_token(token_id, service_account_id)
DeleteToken deletes service account tokens

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:write` scope: `serviceaccounts:id:1` (single service account)  Requires basic authentication and that the authenticated user is a Grafana Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **i64** |  | [required] |
**service_account_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tokens

> Vec<models::TokenDto> list_tokens(service_account_id)
Get service account tokens

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:read` scope: `global:serviceaccounts:id:1` (single service account)  Requires basic authentication and that the authenticated user is a Grafana Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** |  | [required] |

### Return type

[**Vec<models::TokenDto>**](TokenDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_service_account

> models::ServiceAccountDto retrieve_service_account(service_account_id)
Get single serviceaccount by Id

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:read` scope: `serviceaccounts:id:1` (single service account)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** |  | [required] |

### Return type

[**models::ServiceAccountDto**](ServiceAccountDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_org_service_accounts_with_paging

> models::SearchOrgServiceAccountsResult search_org_service_accounts_with_paging(disabled, expired_tokens, query, perpage, page)
Search service accounts with paging

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:read` scope: `serviceaccounts:*`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disabled** | Option<**bool**> |  |  |
**expired_tokens** | Option<**bool**> |  |  |
**query** | Option<**String**> | It will return results where the query value is contained in one of the name. Query values with spaces need to be URL encoded. |  |
**perpage** | Option<**i64**> | The default value is 1000. |  |
**page** | Option<**i64**> | The default value is 1. |  |

### Return type

[**models::SearchOrgServiceAccountsResult**](SearchOrgServiceAccountsResult.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_account

> models::UpdateServiceAccount200Response update_service_account(service_account_id, update_service_account_form)
Update service account

Required permissions (See note in the [introduction](https://grafana.com/docs/grafana/latest/developers/http_api/serviceaccount/#service-account-api) for an explanation): action: `serviceaccounts:write` scope: `serviceaccounts:id:1` (single service account)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account_id** | **i64** |  | [required] |
**update_service_account_form** | Option<[**UpdateServiceAccountForm**](UpdateServiceAccountForm.md)> |  |  |

### Return type

[**models::UpdateServiceAccount200Response**](updateServiceAccount_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

