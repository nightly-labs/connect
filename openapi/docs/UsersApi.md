# \UsersApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_by_id**](UsersApi.md#get_user_by_id) | **GET** /users/{user_id} | Get user by id.
[**get_user_by_login_or_email**](UsersApi.md#get_user_by_login_or_email) | **GET** /users/lookup | Get user by login or email.
[**get_user_org_list**](UsersApi.md#get_user_org_list) | **GET** /users/{user_id}/orgs | Get organizations for user.
[**get_user_teams**](UsersApi.md#get_user_teams) | **GET** /users/{user_id}/teams | Get teams for user.
[**search_users**](UsersApi.md#search_users) | **GET** /users | Get users.
[**search_users_with_paging**](UsersApi.md#search_users_with_paging) | **GET** /users/search | Get users with paging.
[**update_user**](UsersApi.md#update_user) | **PUT** /users/{user_id} | Update user.



## get_user_by_id

> models::UserProfileDto get_user_by_id(user_id)
Get user by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**models::UserProfileDto**](UserProfileDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_login_or_email

> models::UserProfileDto get_user_by_login_or_email(login_or_email)
Get user by login or email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_or_email** | **String** | loginOrEmail of the user | [required] |

### Return type

[**models::UserProfileDto**](UserProfileDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_org_list

> Vec<models::UserOrgDto> get_user_org_list(user_id)
Get organizations for user.

Get organizations for user identified by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**Vec<models::UserOrgDto>**](UserOrgDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_teams

> Vec<models::TeamDto> get_user_teams(user_id)
Get teams for user.

Get teams for user identified by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**Vec<models::TeamDto>**](TeamDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> Vec<models::UserSearchHitDto> search_users(perpage, page)
Get users.

Returns all users that the authenticated user has permission to view, admin permission required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**perpage** | Option<**i64**> | Limit the maximum number of users to return per page |  |[default to 1000]
**page** | Option<**i64**> | Page index for starting fetching users |  |[default to 1]

### Return type

[**Vec<models::UserSearchHitDto>**](UserSearchHitDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users_with_paging

> models::SearchUserQueryResult search_users_with_paging()
Get users with paging.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SearchUserQueryResult**](SearchUserQueryResult.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::SuccessResponseBody update_user(user_id, update_user_command)
Update user.

Update the user identified by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**update_user_command** | [**UpdateUserCommand**](UpdateUserCommand.md) | To change the email, name, login, theme, provide another one. | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

