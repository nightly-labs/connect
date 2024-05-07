# \AccessControlApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team_role**](AccessControlApi.md#add_team_role) | **POST** /access-control/teams/{teamId}/roles | Add team role.
[**add_user_role**](AccessControlApi.md#add_user_role) | **POST** /access-control/users/{userId}/roles | Add a user role assignment.
[**create_role**](AccessControlApi.md#create_role) | **POST** /access-control/roles | Create a new custom role.
[**delete_role**](AccessControlApi.md#delete_role) | **DELETE** /access-control/roles/{roleUID} | Delete a custom role.
[**get_access_control_status**](AccessControlApi.md#get_access_control_status) | **GET** /access-control/status | Get status.
[**get_resource_description**](AccessControlApi.md#get_resource_description) | **GET** /access-control/{resource}/description | Get a description of a resource's access control properties.
[**get_resource_permissions**](AccessControlApi.md#get_resource_permissions) | **GET** /access-control/{resource}/{resourceID} | Get permissions for a resource.
[**get_role**](AccessControlApi.md#get_role) | **GET** /access-control/roles/{roleUID} | Get a role.
[**get_role_assignments**](AccessControlApi.md#get_role_assignments) | **GET** /access-control/roles/{roleUID}/assignments | Get role assignments.
[**list_roles**](AccessControlApi.md#list_roles) | **GET** /access-control/roles | Get all roles.
[**list_team_roles**](AccessControlApi.md#list_team_roles) | **GET** /access-control/teams/{teamId}/roles | Get team roles.
[**list_teams_roles**](AccessControlApi.md#list_teams_roles) | **POST** /access-control/teams/roles/search | List roles assigned to multiple teams.
[**list_user_roles**](AccessControlApi.md#list_user_roles) | **GET** /access-control/users/{userId}/roles | List roles assigned to a user.
[**list_users_roles**](AccessControlApi.md#list_users_roles) | **POST** /access-control/users/roles/search | List roles assigned to multiple users.
[**remove_team_role**](AccessControlApi.md#remove_team_role) | **DELETE** /access-control/teams/{teamId}/roles/{roleUID} | Remove team role.
[**remove_user_role**](AccessControlApi.md#remove_user_role) | **DELETE** /access-control/users/{userId}/roles/{roleUID} | Remove a user role assignment.
[**set_resource_permissions**](AccessControlApi.md#set_resource_permissions) | **POST** /access-control/{resource}/{resourceID} | Set resource permissions.
[**set_resource_permissions_for_built_in_role**](AccessControlApi.md#set_resource_permissions_for_built_in_role) | **POST** /access-control/{resource}/{resourceID}/builtInRoles/{builtInRole} | Set resource permissions for a built-in role.
[**set_resource_permissions_for_team**](AccessControlApi.md#set_resource_permissions_for_team) | **POST** /access-control/{resource}/{resourceID}/teams/{teamID} | Set resource permissions for a team.
[**set_resource_permissions_for_user**](AccessControlApi.md#set_resource_permissions_for_user) | **POST** /access-control/{resource}/{resourceID}/users/{userID} | Set resource permissions for a user.
[**set_role_assignments**](AccessControlApi.md#set_role_assignments) | **PUT** /access-control/roles/{roleUID}/assignments | Set role assignments.
[**set_team_roles**](AccessControlApi.md#set_team_roles) | **PUT** /access-control/teams/{teamId}/roles | Update team role.
[**set_user_roles**](AccessControlApi.md#set_user_roles) | **PUT** /access-control/users/{userId}/roles | Set user role assignments.
[**update_role**](AccessControlApi.md#update_role) | **PUT** /access-control/roles/{roleUID} | Update a custom role.



## add_team_role

> models::SuccessResponseBody add_team_role(team_id, add_team_role_command)
Add team role.

You need to have a permission with action `teams.roles:add` and scope `permissions:type:delegate`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i64** |  | [required] |
**add_team_role_command** | [**AddTeamRoleCommand**](AddTeamRoleCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_role

> models::SuccessResponseBody add_user_role(user_id, add_user_role_command)
Add a user role assignment.

Assign a role to a specific user. For bulk updates consider Set user role assignments.  You need to have a permission with action `users.roles:add` and scope `permissions:type:delegate`. `permissions:type:delegate` scope ensures that users can only assign roles which have same, or a subset of permissions which the user has. For example, if a user does not have required permissions for creating users, they won’t be able to assign a role which will allow to do that. This is done to prevent escalation of privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**add_user_role_command** | [**AddUserRoleCommand**](AddUserRoleCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> models::RoleDto create_role(create_role_form)
Create a new custom role.

Creates a new custom role and maps given permissions to that role. Note that roles with the same prefix as Fixed Roles can’t be created.  You need to have a permission with action `roles:write` and scope `permissions:type:delegate`. `permissions:type:delegate` scope ensures that users can only create custom roles with the same, or a subset of permissions which the user has. For example, if a user does not have required permissions for creating users, they won’t be able to create a custom role which allows to do that. This is done to prevent escalation of privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_role_form** | [**CreateRoleForm**](CreateRoleForm.md) |  | [required] |

### Return type

[**models::RoleDto**](RoleDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> models::SuccessResponseBody delete_role(role_uid, force, global)
Delete a custom role.

Delete a role with the given UID, and it’s permissions. If the role is assigned to a built-in role, the deletion operation will fail, unless force query param is set to true, and in that case all assignments will also be deleted.  You need to have a permission with action `roles:delete` and scope `permissions:type:delegate`. `permissions:type:delegate` scope ensures that users can only delete a custom role with the same, or a subset of permissions which the user has. For example, if a user does not have required permissions for creating users, they won’t be able to delete a custom role which allows to do that.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_uid** | **String** |  | [required] |
**force** | Option<**bool**> |  |  |
**global** | Option<**bool**> |  |  |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_access_control_status

> i64 get_access_control_status()
Get status.

Returns an indicator to check if fine-grained access control is enabled or not.  You need to have a permission with action `status:accesscontrol` and scope `services:accesscontrol`.

### Parameters

This endpoint does not need any parameter.

### Return type

**i64**

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_description

> models::Description get_resource_description(resource)
Get a description of a resource's access control properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | **String** |  | [required] |

### Return type

[**models::Description**](Description.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource_permissions

> Vec<models::ResourcePermissionDto> get_resource_permissions(resource, resource_id)
Get permissions for a resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | **String** |  | [required] |
**resource_id** | **String** |  | [required] |

### Return type

[**Vec<models::ResourcePermissionDto>**](resourcePermissionDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> models::RoleDto get_role(role_uid)
Get a role.

Get a role for the given UID.  You need to have a permission with action `roles:read` and scope `roles:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_uid** | **String** |  | [required] |

### Return type

[**models::RoleDto**](RoleDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_assignments

> models::RoleAssignmentsDto get_role_assignments(role_uid)
Get role assignments.

Get role assignments for the role with the given UID.  You need to have a permission with action `teams.roles:list` and scope `teams:id:*` and `users.roles:list` and scope `users:id:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_uid** | **String** |  | [required] |

### Return type

[**models::RoleAssignmentsDto**](RoleAssignmentsDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_roles

> Vec<models::RoleDto> list_roles(delegatable, include_hidden)
Get all roles.

Gets all existing roles. The response contains all global and organization local roles, for the organization which user is signed in.  You need to have a permission with action `roles:read` and scope `roles:*`.  The `delegatable` flag reduces the set of roles to only those for which the signed-in user has permissions to assign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegatable** | Option<**bool**> |  |  |
**include_hidden** | Option<**bool**> |  |  |

### Return type

[**Vec<models::RoleDto>**](RoleDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_team_roles

> models::SuccessResponseBody list_team_roles(team_id)
Get team roles.

You need to have a permission with action `teams.roles:read` and scope `teams:id:<team ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_teams_roles

> std::collections::HashMap<String, Vec<models::RoleDto>> list_teams_roles(roles_search_query)
List roles assigned to multiple teams.

Lists the roles that have been directly assigned to the given teams.  You need to have a permission with action `teams.roles:read` and scope `teams:id:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roles_search_query** | [**RolesSearchQuery**](RolesSearchQuery.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, Vec<models::RoleDto>>**](Vec.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_roles

> Vec<models::RoleDto> list_user_roles(user_id)
List roles assigned to a user.

Lists the roles that have been directly assigned to a given user. The list does not include built-in roles (Viewer, Editor, Admin or Grafana Admin), and it does not include roles that have been inherited from a team.  You need to have a permission with action `users.roles:read` and scope `users:id:<user ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |

### Return type

[**Vec<models::RoleDto>**](RoleDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_roles

> std::collections::HashMap<String, Vec<models::RoleDto>> list_users_roles(roles_search_query)
List roles assigned to multiple users.

Lists the roles that have been directly assigned to the given users. The list does not include built-in roles (Viewer, Editor, Admin or Grafana Admin), and it does not include roles that have been inherited from a team.  You need to have a permission with action `users.roles:read` and scope `users:id:*`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**roles_search_query** | [**RolesSearchQuery**](RolesSearchQuery.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, Vec<models::RoleDto>>**](Vec.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_team_role

> models::SuccessResponseBody remove_team_role(role_uid, team_id)
Remove team role.

You need to have a permission with action `teams.roles:remove` and scope `permissions:type:delegate`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_uid** | **String** |  | [required] |
**team_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_role

> models::SuccessResponseBody remove_user_role(role_uid, user_id, global)
Remove a user role assignment.

Revoke a role from a user. For bulk updates consider Set user role assignments.  You need to have a permission with action `users.roles:remove` and scope `permissions:type:delegate`. `permissions:type:delegate` scope ensures that users can only unassign roles which have same, or a subset of permissions which the user has. For example, if a user does not have required permissions for creating users, they won’t be able to unassign a role which will allow to do that. This is done to prevent escalation of privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_uid** | **String** |  | [required] |
**user_id** | **i64** |  | [required] |
**global** | Option<**bool**> | A flag indicating if the assignment is global or not. If set to false, the default org ID of the authenticated user will be used from the request to remove assignment. |  |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_resource_permissions

> models::SuccessResponseBody set_resource_permissions(resource, resource_id, set_permissions_command)
Set resource permissions.

Assigns permissions for a resource by a given type (`:resource`) and `:resourceID` to one or many assignment types. Allowed resources are `datasources`, `teams`, `dashboards`, `folders`, and `serviceaccounts`. Refer to the `/access-control/{resource}/description` endpoint for allowed Permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | **String** |  | [required] |
**resource_id** | **String** |  | [required] |
**set_permissions_command** | [**SetPermissionsCommand**](SetPermissionsCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_resource_permissions_for_built_in_role

> models::SuccessResponseBody set_resource_permissions_for_built_in_role(resource, resource_id, built_in_role, set_permission_command)
Set resource permissions for a built-in role.

Assigns permissions for a resource by a given type (`:resource`) and `:resourceID` to a built-in role. Allowed resources are `datasources`, `teams`, `dashboards`, `folders`, and `serviceaccounts`. Refer to the `/access-control/{resource}/description` endpoint for allowed Permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | **String** |  | [required] |
**resource_id** | **String** |  | [required] |
**built_in_role** | **String** |  | [required] |
**set_permission_command** | [**SetPermissionCommand**](SetPermissionCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_resource_permissions_for_team

> models::SuccessResponseBody set_resource_permissions_for_team(resource, resource_id, team_id, set_permission_command)
Set resource permissions for a team.

Assigns permissions for a resource by a given type (`:resource`) and `:resourceID` to a team. Allowed resources are `datasources`, `teams`, `dashboards`, `folders`, and `serviceaccounts`. Refer to the `/access-control/{resource}/description` endpoint for allowed Permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | **String** |  | [required] |
**resource_id** | **String** |  | [required] |
**team_id** | **i64** |  | [required] |
**set_permission_command** | [**SetPermissionCommand**](SetPermissionCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_resource_permissions_for_user

> models::SuccessResponseBody set_resource_permissions_for_user(resource, resource_id, user_id, set_permission_command)
Set resource permissions for a user.

Assigns permissions for a resource by a given type (`:resource`) and `:resourceID` to a user or a service account. Allowed resources are `datasources`, `teams`, `dashboards`, `folders`, and `serviceaccounts`. Refer to the `/access-control/{resource}/description` endpoint for allowed Permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | **String** |  | [required] |
**resource_id** | **String** |  | [required] |
**user_id** | **i64** |  | [required] |
**set_permission_command** | [**SetPermissionCommand**](SetPermissionCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_role_assignments

> models::RoleAssignmentsDto set_role_assignments(role_uid, set_role_assignments_command)
Set role assignments.

Set role assignments for the role with the given UID.  You need to have a permission with action `teams.roles:add` and `teams.roles:remove` and scope `permissions:type:delegate`, and `users.roles:add` and `users.roles:remove` and scope `permissions:type:delegate`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_uid** | **String** |  | [required] |
**set_role_assignments_command** | [**SetRoleAssignmentsCommand**](SetRoleAssignmentsCommand.md) |  | [required] |

### Return type

[**models::RoleAssignmentsDto**](RoleAssignmentsDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_team_roles

> models::SuccessResponseBody set_team_roles(team_id)
Update team role.

You need to have a permission with action `teams.roles:add` and `teams.roles:remove` and scope `permissions:type:delegate` for each.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_roles

> models::SuccessResponseBody set_user_roles(user_id, set_user_roles_command)
Set user role assignments.

Update the user’s role assignments to match the provided set of UIDs. This will remove any assigned roles that aren’t in the request and add roles that are in the set but are not already assigned to the user. If you want to add or remove a single role, consider using Add a user role assignment or Remove a user role assignment instead.  You need to have a permission with action `users.roles:add` and `users.roles:remove` and scope `permissions:type:delegate` for each. `permissions:type:delegate`  scope ensures that users can only assign or unassign roles which have same, or a subset of permissions which the user has. For example, if a user does not have required permissions for creating users, they won’t be able to assign or unassign a role which will allow to do that. This is done to prevent escalation of privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** |  | [required] |
**set_user_roles_command** | [**SetUserRolesCommand**](SetUserRolesCommand.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> models::RoleDto update_role(role_uid, update_role_command)
Update a custom role.

You need to have a permission with action `roles:write` and scope `permissions:type:delegate`. `permissions:type:delegate` scope ensures that users can only create custom roles with the same, or a subset of permissions which the user has.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_uid** | **String** |  | [required] |
**update_role_command** | [**UpdateRoleCommand**](UpdateRoleCommand.md) |  | [required] |

### Return type

[**models::RoleDto**](RoleDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

