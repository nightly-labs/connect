# \OrgApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_org_user_to_current_org**](OrgApi.md#add_org_user_to_current_org) | **POST** /org/users | Add a new user to the current organization.
[**get_current_org**](OrgApi.md#get_current_org) | **GET** /org | Get current Organization.
[**get_org_users_for_current_org**](OrgApi.md#get_org_users_for_current_org) | **GET** /org/users | Get all users within the current organization.
[**get_org_users_for_current_org_lookup**](OrgApi.md#get_org_users_for_current_org_lookup) | **GET** /org/users/lookup | Get all users within the current organization (lookup)
[**remove_org_user_for_current_org**](OrgApi.md#remove_org_user_for_current_org) | **DELETE** /org/users/{user_id} | Delete user in current organization.
[**update_current_org**](OrgApi.md#update_current_org) | **PUT** /org | Update current Organization.
[**update_current_org_address**](OrgApi.md#update_current_org_address) | **PUT** /org/address | Update current Organization's address.
[**update_org_user_for_current_org**](OrgApi.md#update_org_user_for_current_org) | **PATCH** /org/users/{user_id} | Updates the given user.



## add_org_user_to_current_org

> models::SuccessResponseBody add_org_user_to_current_org(add_org_user_command)
Add a new user to the current organization.

Adds a global user to the current organization.  If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users:add` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_org_user_command** | [**AddOrgUserCommand**](AddOrgUserCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_org

> models::OrgDetailsDto get_current_org()
Get current Organization.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrgDetailsDto**](OrgDetailsDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_users_for_current_org

> Vec<models::OrgUserDto> get_org_users_for_current_org()
Get all users within the current organization.

Returns all org users within the current organization. Accessible to users with org admin role. If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users:read` with scope `users:*`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::OrgUserDto>**](OrgUserDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_users_for_current_org_lookup

> Vec<models::UserLookupDto> get_org_users_for_current_org_lookup(query, limit)
Get all users within the current organization (lookup)

Returns all org users within the current organization, but with less detailed information. Accessible to users with org admin role, admin in any folder or admin of any team. Mainly used by Grafana UI for providing list of users when adding team members and when editing folder/dashboard permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |
**limit** | Option<**i64**> |  |  |

### Return type

[**Vec<models::UserLookupDto>**](UserLookupDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_org_user_for_current_org

> models::SuccessResponseBody remove_org_user_for_current_org(user_id)
Delete user in current organization.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users:remove` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_org

> models::SuccessResponseBody update_current_org(update_org_form)
Update current Organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_org_form** | [**UpdateOrgForm**](UpdateOrgForm.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_org_address

> models::SuccessResponseBody update_current_org_address(update_org_address_form)
Update current Organization's address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_org_address_form** | [**UpdateOrgAddressForm**](UpdateOrgAddressForm.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_user_for_current_org

> models::SuccessResponseBody update_org_user_for_current_org(user_id, update_org_user_command)
Updates the given user.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users.role:update` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**update_org_user_command** | [**UpdateOrgUserCommand**](UpdateOrgUserCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

