# \TeamsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team_member**](TeamsApi.md#add_team_member) | **POST** /teams/{team_id}/members | Add Team Member.
[**create_team**](TeamsApi.md#create_team) | **POST** /teams | Add Team.
[**delete_team_by_id**](TeamsApi.md#delete_team_by_id) | **DELETE** /teams/{team_id} | Delete Team By ID.
[**get_team_by_id**](TeamsApi.md#get_team_by_id) | **GET** /teams/{team_id} | Get Team By ID.
[**get_team_members**](TeamsApi.md#get_team_members) | **GET** /teams/{team_id}/members | Get Team Members.
[**get_team_preferences**](TeamsApi.md#get_team_preferences) | **GET** /teams/{team_id}/preferences | Get Team Preferences.
[**remove_team_member**](TeamsApi.md#remove_team_member) | **DELETE** /teams/{team_id}/members/{user_id} | Remove Member From Team.
[**search_teams**](TeamsApi.md#search_teams) | **GET** /teams/search | Team Search With Paging.
[**update_team**](TeamsApi.md#update_team) | **PUT** /teams/{team_id} | Update Team.
[**update_team_member**](TeamsApi.md#update_team_member) | **PUT** /teams/{team_id}/members/{user_id} | Update Team Member.
[**update_team_preferences**](TeamsApi.md#update_team_preferences) | **PUT** /teams/{team_id}/preferences | Update Team Preferences.



## add_team_member

> models::SuccessResponseBody add_team_member(team_id, add_team_member_command)
Add Team Member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**add_team_member_command** | [**AddTeamMemberCommand**](AddTeamMemberCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_team

> models::CreateTeam200Response create_team(create_team_command)
Add Team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_team_command** | [**CreateTeamCommand**](CreateTeamCommand.md) |  | [required] |

### Return type

[**models::CreateTeam200Response**](createTeam_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_team_by_id

> models::SuccessResponseBody delete_team_by_id(team_id)
Delete Team By ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_by_id

> models::TeamDto get_team_by_id(team_id)
Get Team By ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |

### Return type

[**models::TeamDto**](TeamDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_members

> Vec<models::TeamMemberDto> get_team_members(team_id)
Get Team Members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |

### Return type

[**Vec<models::TeamMemberDto>**](TeamMemberDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_preferences

> models::Preferences get_team_preferences(team_id)
Get Team Preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |

### Return type

[**models::Preferences**](Preferences.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_team_member

> models::SuccessResponseBody remove_team_member(team_id, user_id)
Remove Member From Team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**user_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_teams

> models::SearchTeamQueryResult search_teams(page, perpage, name, query)
Team Search With Paging.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> |  |  |[default to 1]
**perpage** | Option<**i64**> | Number of items per page The totalCount field in the response can be used for pagination list E.g. if totalCount is equal to 100 teams and the perpage parameter is set to 10 then there are 10 pages of teams. |  |[default to 1000]
**name** | Option<**String**> |  |  |
**query** | Option<**String**> | If set it will return results where the query value is contained in the name field. Query values with spaces need to be URL encoded. |  |

### Return type

[**models::SearchTeamQueryResult**](SearchTeamQueryResult.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team

> models::SuccessResponseBody update_team(team_id, update_team_command)
Update Team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**update_team_command** | [**UpdateTeamCommand**](UpdateTeamCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team_member

> models::SuccessResponseBody update_team_member(team_id, user_id, update_team_member_command)
Update Team Member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**user_id** | **i64** |  | [required] |
**update_team_member_command** | [**UpdateTeamMemberCommand**](UpdateTeamMemberCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team_preferences

> models::SuccessResponseBody update_team_preferences(team_id, update_prefs_cmd)
Update Team Preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** |  | [required] |
**update_prefs_cmd** | [**UpdatePrefsCmd**](UpdatePrefsCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

