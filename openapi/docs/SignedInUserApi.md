# \SignedInUserApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_user_password**](SignedInUserApi.md#change_user_password) | **PUT** /user/password | Change Password.
[**clear_help_flags**](SignedInUserApi.md#clear_help_flags) | **GET** /user/helpflags/clear | Clear user help flag.
[**get_signed_in_user**](SignedInUserApi.md#get_signed_in_user) | **GET** /user | 
[**get_signed_in_user_org_list**](SignedInUserApi.md#get_signed_in_user_org_list) | **GET** /user/orgs | Organizations of the actual User.
[**get_signed_in_user_team_list**](SignedInUserApi.md#get_signed_in_user_team_list) | **GET** /user/teams | Teams that the actual User is member of.
[**get_user_auth_tokens**](SignedInUserApi.md#get_user_auth_tokens) | **GET** /user/auth-tokens | Auth tokens of the actual User.
[**get_user_quotas**](SignedInUserApi.md#get_user_quotas) | **GET** /user/quotas | Fetch user quota.
[**revoke_user_auth_token**](SignedInUserApi.md#revoke_user_auth_token) | **POST** /user/revoke-auth-token | Revoke an auth token of the actual User.
[**set_help_flag**](SignedInUserApi.md#set_help_flag) | **PUT** /user/helpflags/{flag_id} | Set user help flag.
[**star_dashboard**](SignedInUserApi.md#star_dashboard) | **POST** /user/stars/dashboard/{dashboard_id} | Star a dashboard.
[**star_dashboard_by_uid**](SignedInUserApi.md#star_dashboard_by_uid) | **POST** /user/stars/dashboard/uid/{dashboard_uid} | Star a dashboard.
[**unstar_dashboard**](SignedInUserApi.md#unstar_dashboard) | **DELETE** /user/stars/dashboard/{dashboard_id} | Unstar a dashboard.
[**unstar_dashboard_by_uid**](SignedInUserApi.md#unstar_dashboard_by_uid) | **DELETE** /user/stars/dashboard/uid/{dashboard_uid} | Unstar a dashboard.
[**update_signed_in_user**](SignedInUserApi.md#update_signed_in_user) | **PUT** /user | Update signed in User.
[**user_set_using_org**](SignedInUserApi.md#user_set_using_org) | **POST** /user/using/{org_id} | Switch user context for signed in user.



## change_user_password

> models::SuccessResponseBody change_user_password(change_user_password_command)
Change Password.

Changes the password for the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_user_password_command** | [**ChangeUserPasswordCommand**](ChangeUserPasswordCommand.md) | To change the email, name, login, theme, provide another one. | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_help_flags

> models::ClearHelpFlags200Response clear_help_flags()
Clear user help flag.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClearHelpFlags200Response**](clearHelpFlags_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_signed_in_user

> models::UserProfileDto get_signed_in_user()


Get (current authenticated user)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserProfileDto**](UserProfileDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_signed_in_user_org_list

> Vec<models::UserOrgDto> get_signed_in_user_org_list()
Organizations of the actual User.

Return a list of all organizations of the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserOrgDto>**](UserOrgDTO.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_signed_in_user_team_list

> Vec<models::TeamDto> get_signed_in_user_team_list()
Teams that the actual User is member of.

Return a list of all teams that the current user is member of.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TeamDto>**](TeamDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_auth_tokens

> Vec<models::UserToken> get_user_auth_tokens()
Auth tokens of the actual User.

Return a list of all auth tokens (devices) that the actual user currently have logged in from.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserToken>**](UserToken.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_quotas

> Vec<models::QuotaDto> get_user_quotas()
Fetch user quota.

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


## revoke_user_auth_token

> models::SuccessResponseBody revoke_user_auth_token(revoke_auth_token_cmd)
Revoke an auth token of the actual User.

Revokes the given auth token (device) for the actual user. User of issued auth token (device) will no longer be logged in and will be required to authenticate again upon next activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_auth_token_cmd** | [**RevokeAuthTokenCmd**](RevokeAuthTokenCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_help_flag

> models::ClearHelpFlags200Response set_help_flag(flag_id)
Set user help flag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flag_id** | **String** |  | [required] |

### Return type

[**models::ClearHelpFlags200Response**](clearHelpFlags_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## star_dashboard

> models::SuccessResponseBody star_dashboard(dashboard_id)
Star a dashboard.

Stars the given Dashboard for the actual user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## star_dashboard_by_uid

> models::SuccessResponseBody star_dashboard_by_uid(dashboard_uid)
Star a dashboard.

Stars the given Dashboard for the actual user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_uid** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unstar_dashboard

> models::SuccessResponseBody unstar_dashboard(dashboard_id)
Unstar a dashboard.

Deletes the starring of the given Dashboard for the actual user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unstar_dashboard_by_uid

> models::SuccessResponseBody unstar_dashboard_by_uid(dashboard_uid)
Unstar a dashboard.

Deletes the starring of the given Dashboard for the actual user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_uid** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_signed_in_user

> models::SuccessResponseBody update_signed_in_user(update_user_command)
Update signed in User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_command** | [**UpdateUserCommand**](UpdateUserCommand.md) | To change the email, name, login, theme, provide another one. | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_set_using_org

> models::SuccessResponseBody user_set_using_org(org_id)
Switch user context for signed in user.

Switch user context to the given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

