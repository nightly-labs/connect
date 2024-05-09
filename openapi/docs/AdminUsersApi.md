# \AdminUsersApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_create_user**](AdminUsersApi.md#admin_create_user) | **POST** /admin/users | Create new user.
[**admin_delete_user**](AdminUsersApi.md#admin_delete_user) | **DELETE** /admin/users/{user_id} | Delete global User.
[**admin_disable_user**](AdminUsersApi.md#admin_disable_user) | **POST** /admin/users/{user_id}/disable | Disable user.
[**admin_enable_user**](AdminUsersApi.md#admin_enable_user) | **POST** /admin/users/{user_id}/enable | Enable user.
[**admin_get_user_auth_tokens**](AdminUsersApi.md#admin_get_user_auth_tokens) | **GET** /admin/users/{user_id}/auth-tokens | Return a list of all auth tokens (devices) that the user currently have logged in from.
[**admin_logout_user**](AdminUsersApi.md#admin_logout_user) | **POST** /admin/users/{user_id}/logout | Logout user revokes all auth tokens (devices) for the user. User of issued auth tokens (devices) will no longer be logged in and will be required to authenticate again upon next activity.
[**admin_revoke_user_auth_token**](AdminUsersApi.md#admin_revoke_user_auth_token) | **POST** /admin/users/{user_id}/revoke-auth-token | Revoke auth token for user.
[**admin_update_user_password**](AdminUsersApi.md#admin_update_user_password) | **PUT** /admin/users/{user_id}/password | Set password for user.
[**admin_update_user_permissions**](AdminUsersApi.md#admin_update_user_permissions) | **PUT** /admin/users/{user_id}/permissions | Set permissions for user.
[**get_user_quota**](AdminUsersApi.md#get_user_quota) | **GET** /admin/users/{user_id}/quotas | Fetch user quota.
[**update_user_quota**](AdminUsersApi.md#update_user_quota) | **PUT** /admin/users/{user_id}/quotas/{quota_target} | Update user quota.



## admin_create_user

> models::AdminCreateUserResponse admin_create_user(admin_create_user_form)
Create new user.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users:create`. Note that OrgId is an optional parameter that can be used to assign a new user to a different organization when `auto_assign_org` is set to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin_create_user_form** | [**AdminCreateUserForm**](AdminCreateUserForm.md) |  | [required] |

### Return type

[**models::AdminCreateUserResponse**](AdminCreateUserResponse.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_delete_user

> models::SuccessResponseBody admin_delete_user(user_id)
Delete global User.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users:delete` and scope `global.users:*`.

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


## admin_disable_user

> models::SuccessResponseBody admin_disable_user(user_id)
Disable user.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users:disable` and scope `global.users:1` (userIDScope).

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


## admin_enable_user

> models::SuccessResponseBody admin_enable_user(user_id)
Enable user.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users:enable` and scope `global.users:1` (userIDScope).

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


## admin_get_user_auth_tokens

> Vec<models::UserToken> admin_get_user_auth_tokens(user_id)
Return a list of all auth tokens (devices) that the user currently have logged in from.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users.authtoken:list` and scope `global.users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**Vec<models::UserToken>**](UserToken.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_logout_user

> models::SuccessResponseBody admin_logout_user(user_id)
Logout user revokes all auth tokens (devices) for the user. User of issued auth tokens (devices) will no longer be logged in and will be required to authenticate again upon next activity.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users.logout` and scope `global.users:*`.

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


## admin_revoke_user_auth_token

> models::SuccessResponseBody admin_revoke_user_auth_token(user_id, revoke_auth_token_cmd)
Revoke auth token for user.

Revokes the given auth token (device) for the user. User of issued auth token (device) will no longer be logged in and will be required to authenticate again upon next activity. If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users.authtoken:update` and scope `global.users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**revoke_auth_token_cmd** | [**RevokeAuthTokenCmd**](RevokeAuthTokenCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_update_user_password

> models::SuccessResponseBody admin_update_user_password(user_id, admin_update_user_password_form)
Set password for user.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users.password:update` and scope `global.users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**admin_update_user_password_form** | [**AdminUpdateUserPasswordForm**](AdminUpdateUserPasswordForm.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_update_user_permissions

> models::SuccessResponseBody admin_update_user_permissions(user_id, admin_update_user_permissions_form)
Set permissions for user.

Only works with Basic Authentication (username and password). See introduction for an explanation. If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users.permissions:update` and scope `global.users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**admin_update_user_permissions_form** | [**AdminUpdateUserPermissionsForm**](AdminUpdateUserPermissionsForm.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_quota

> Vec<models::QuotaDto> get_user_quota(user_id)
Fetch user quota.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users.quotas:list` and scope `global.users:1` (userIDScope).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**Vec<models::QuotaDto>**](QuotaDTO.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_quota

> models::SuccessResponseBody update_user_quota(quota_target, user_id, update_quota_cmd)
Update user quota.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `users.quotas:update` and scope `global.users:1` (userIDScope).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_target** | **String** |  | [required] |
**user_id** | **i64** |  | [required] |
**update_quota_cmd** | [**UpdateQuotaCmd**](UpdateQuotaCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

