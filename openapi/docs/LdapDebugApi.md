# \LdapDebugApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sync_status**](LdapDebugApi.md#get_sync_status) | **GET** /admin/ldap-sync-status | Returns the current state of the LDAP background sync integration.



## get_sync_status

> models::ActiveSyncStatusDto get_sync_status()
Returns the current state of the LDAP background sync integration.

You need to have a permission with action `ldap.status:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ActiveSyncStatusDto**](ActiveSyncStatusDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

