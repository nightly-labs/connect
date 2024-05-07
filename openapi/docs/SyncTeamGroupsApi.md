# \SyncTeamGroupsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team_group_api**](SyncTeamGroupsApi.md#add_team_group_api) | **POST** /teams/{teamId}/groups | Add External Group.
[**get_team_groups_api**](SyncTeamGroupsApi.md#get_team_groups_api) | **GET** /teams/{teamId}/groups | Get External Groups.
[**remove_team_group_api_query**](SyncTeamGroupsApi.md#remove_team_group_api_query) | **DELETE** /teams/{teamId}/groups | Remove External Group.



## add_team_group_api

> models::SuccessResponseBody add_team_group_api(team_id, team_group_mapping)
Add External Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i64** |  | [required] |
**team_group_mapping** | [**TeamGroupMapping**](TeamGroupMapping.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_groups_api

> Vec<models::TeamGroupDto> get_team_groups_api(team_id)
Get External Groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i64** |  | [required] |

### Return type

[**Vec<models::TeamGroupDto>**](TeamGroupDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_team_group_api_query

> models::SuccessResponseBody remove_team_group_api_query(team_id, group_id)
Remove External Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i64** |  | [required] |
**group_id** | Option<**String**> |  |  |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

