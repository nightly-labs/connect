# \AdminLdapApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ldap_status**](AdminLdapApi.md#get_ldap_status) | **GET** /admin/ldap/status | Attempts to connect to all the configured LDAP servers and returns information on whenever they're available or not.
[**get_user_from_ldap**](AdminLdapApi.md#get_user_from_ldap) | **GET** /admin/ldap/{user_name} | Finds an user based on a username in LDAP. This helps illustrate how would the particular user be mapped in Grafana when synced.
[**post_sync_user_with_ldap**](AdminLdapApi.md#post_sync_user_with_ldap) | **POST** /admin/ldap/sync/{user_id} | Enables a single Grafana user to be synchronized against LDAP.
[**reload_ldap_cfg**](AdminLdapApi.md#reload_ldap_cfg) | **POST** /admin/ldap/reload | Reloads the LDAP configuration.



## get_ldap_status

> models::SuccessResponseBody get_ldap_status()
Attempts to connect to all the configured LDAP servers and returns information on whenever they're available or not.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `ldap.status:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_from_ldap

> models::SuccessResponseBody get_user_from_ldap(user_name)
Finds an user based on a username in LDAP. This helps illustrate how would the particular user be mapped in Grafana when synced.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `ldap.user:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sync_user_with_ldap

> models::SuccessResponseBody post_sync_user_with_ldap(user_id)
Enables a single Grafana user to be synchronized against LDAP.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `ldap.user:sync`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_ldap_cfg

> models::SuccessResponseBody reload_ldap_cfg()
Reloads the LDAP configuration.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `ldap.config:reload`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

