# \GetCurrentOrgApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_current_org_quota**](GetCurrentOrgApi.md#get_current_org_quota) | **GET** /org/quotas | Fetch Organization quota.



## get_current_org_quota

> Vec<models::QuotaDto> get_current_org_quota()
Fetch Organization quota.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `orgs.quotas:read` and scope `org:id:1` (orgIDScope).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::QuotaDto>**](QuotaDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

