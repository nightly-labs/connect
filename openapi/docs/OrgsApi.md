# \OrgsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_org_user**](OrgsApi.md#add_org_user) | **POST** /orgs/{org_id}/users | Add a new user to the current organization.
[**create_org**](OrgsApi.md#create_org) | **POST** /orgs | Create Organization.
[**delete_org_by_id**](OrgsApi.md#delete_org_by_id) | **DELETE** /orgs/{org_id} | Delete Organization.
[**get_org_by_id**](OrgsApi.md#get_org_by_id) | **GET** /orgs/{org_id} | Get Organization by ID.
[**get_org_by_name**](OrgsApi.md#get_org_by_name) | **GET** /orgs/name/{org_name} | Get Organization by ID.
[**get_org_quota**](OrgsApi.md#get_org_quota) | **GET** /orgs/{org_id}/quotas | Fetch Organization quota.
[**get_org_users**](OrgsApi.md#get_org_users) | **GET** /orgs/{org_id}/users | Get Users in Organization.
[**remove_org_user**](OrgsApi.md#remove_org_user) | **DELETE** /orgs/{org_id}/users/{user_id} | Delete user in current organization.
[**search_org_users**](OrgsApi.md#search_org_users) | **GET** /orgs/{org_id}/users/search | Search Users in Organization.
[**search_orgs**](OrgsApi.md#search_orgs) | **GET** /orgs | Search all Organizations.
[**update_org**](OrgsApi.md#update_org) | **PUT** /orgs/{org_id} | Update Organization.
[**update_org_address**](OrgsApi.md#update_org_address) | **PUT** /orgs/{org_id}/address | Update Organization's address.
[**update_org_quota**](OrgsApi.md#update_org_quota) | **PUT** /orgs/{org_id}/quotas/{quota_target} | Update user quota.
[**update_org_user**](OrgsApi.md#update_org_user) | **PATCH** /orgs/{org_id}/users/{user_id} | Update Users in Organization.



## add_org_user

> models::SuccessResponseBody add_org_user(org_id, add_org_user_command)
Add a new user to the current organization.

Adds a global user to the current organization.  If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users:add` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |
**add_org_user_command** | [**AddOrgUserCommand**](AddOrgUserCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_org

> models::CreateOrg200Response create_org(create_org_command)
Create Organization.

Only works if [users.allow_org_create](https://grafana.com/docs/grafana/latest/administration/configuration/#allow_org_create) is set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_org_command** | [**CreateOrgCommand**](CreateOrgCommand.md) |  | [required] |

### Return type

[**models::CreateOrg200Response**](createOrg_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_org_by_id

> models::SuccessResponseBody delete_org_by_id(org_id)
Delete Organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_by_id

> models::OrgDetailsDto get_org_by_id(org_id)
Get Organization by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |

### Return type

[**models::OrgDetailsDto**](OrgDetailsDTO.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_by_name

> models::OrgDetailsDto get_org_by_name(org_name)
Get Organization by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_name** | **String** |  | [required] |

### Return type

[**models::OrgDetailsDto**](OrgDetailsDTO.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_quota

> Vec<models::QuotaDto> get_org_quota(org_id)
Fetch Organization quota.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `orgs.quotas:read` and scope `org:id:1` (orgIDScope).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |

### Return type

[**Vec<models::QuotaDto>**](QuotaDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_users

> Vec<models::OrgUserDto> get_org_users(org_id)
Get Users in Organization.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users:read` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |

### Return type

[**Vec<models::OrgUserDto>**](OrgUserDTO.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_org_user

> models::SuccessResponseBody remove_org_user(org_id, user_id)
Delete user in current organization.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users:remove` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |
**user_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_org_users

> models::SearchOrgUsersQueryResult search_org_users(org_id)
Search Users in Organization.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users:read` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |

### Return type

[**models::SearchOrgUsersQueryResult**](SearchOrgUsersQueryResult.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_orgs

> Vec<models::OrgDto> search_orgs(page, perpage, name, query)
Search all Organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> |  |  |[default to 1]
**perpage** | Option<**i64**> | Number of items per page The totalCount field in the response can be used for pagination list E.g. if totalCount is equal to 100 teams and the perpage parameter is set to 10 then there are 10 pages of teams. |  |[default to 1000]
**name** | Option<**String**> |  |  |
**query** | Option<**String**> | If set it will return results where the query value is contained in the name field. Query values with spaces need to be URL encoded. |  |

### Return type

[**Vec<models::OrgDto>**](OrgDTO.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org

> models::SuccessResponseBody update_org(org_id, update_org_form)
Update Organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |
**update_org_form** | [**UpdateOrgForm**](UpdateOrgForm.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_address

> models::SuccessResponseBody update_org_address(org_id, update_org_address_form)
Update Organization's address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |
**update_org_address_form** | [**UpdateOrgAddressForm**](UpdateOrgAddressForm.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_quota

> models::SuccessResponseBody update_org_quota(quota_target, org_id, update_quota_cmd)
Update user quota.

If you are running Grafana Enterprise and have Fine-grained access control enabled, you need to have a permission with action `orgs.quotas:write` and scope `org:id:1` (orgIDScope).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_target** | **String** |  | [required] |
**org_id** | **i64** |  | [required] |
**update_quota_cmd** | [**UpdateQuotaCmd**](UpdateQuotaCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_user

> models::SuccessResponseBody update_org_user(org_id, user_id, update_org_user_command)
Update Users in Organization.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `org.users.role:update` with scope `users:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i64** |  | [required] |
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

