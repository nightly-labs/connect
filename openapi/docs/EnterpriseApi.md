# \EnterpriseApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team_group_api**](EnterpriseApi.md#add_team_group_api) | **POST** /teams/{teamId}/groups | Add External Group.
[**add_team_role**](EnterpriseApi.md#add_team_role) | **POST** /access-control/teams/{teamId}/roles | Add team role.
[**add_user_role**](EnterpriseApi.md#add_user_role) | **POST** /access-control/users/{userId}/roles | Add a user role assignment.
[**admin_provisioning_reload_access_control**](EnterpriseApi.md#admin_provisioning_reload_access_control) | **POST** /admin/provisioning/access-control/reload | You need to have a permission with action `provisioning:reload` with scope `provisioners:accesscontrol`.
[**create_recording_rule**](EnterpriseApi.md#create_recording_rule) | **POST** /recording-rules | Create a recording rule that is then registered and started.
[**create_recording_rule_write_target**](EnterpriseApi.md#create_recording_rule_write_target) | **POST** /recording-rules/writer | Create a remote write target.
[**create_report**](EnterpriseApi.md#create_report) | **POST** /reports | Create a report.
[**create_role**](EnterpriseApi.md#create_role) | **POST** /access-control/roles | Create a new custom role.
[**delete_license_token**](EnterpriseApi.md#delete_license_token) | **DELETE** /licensing/token | Remove license from database.
[**delete_recording_rule**](EnterpriseApi.md#delete_recording_rule) | **DELETE** /recording-rules/{recordingRuleID} | Delete removes the rule from the registry and stops it.
[**delete_recording_rule_write_target**](EnterpriseApi.md#delete_recording_rule_write_target) | **DELETE** /recording-rules/writer | Delete the remote write target.
[**delete_report**](EnterpriseApi.md#delete_report) | **DELETE** /reports/{id} | Delete a report.
[**delete_role**](EnterpriseApi.md#delete_role) | **DELETE** /access-control/roles/{roleUID} | Delete a custom role.
[**get_access_control_status**](EnterpriseApi.md#get_access_control_status) | **GET** /access-control/status | Get status.
[**get_custom_permissions_csv**](EnterpriseApi.md#get_custom_permissions_csv) | **GET** /licensing/custom-permissions-csv | Get custom permissions report in CSV format.
[**get_custom_permissions_report**](EnterpriseApi.md#get_custom_permissions_report) | **GET** /licensing/custom-permissions | Get custom permissions report.
[**get_license_token**](EnterpriseApi.md#get_license_token) | **GET** /licensing/token | Get license token.
[**get_metadata**](EnterpriseApi.md#get_metadata) | **GET** /saml/metadata | It exposes the SP (Grafana's) metadata for the IdP's consumption.
[**get_recording_rule_write_target**](EnterpriseApi.md#get_recording_rule_write_target) | **GET** /recording-rules/writer | Return the prometheus remote write target.
[**get_report**](EnterpriseApi.md#get_report) | **GET** /reports/{id} | Get a report.
[**get_report_settings**](EnterpriseApi.md#get_report_settings) | **GET** /reports/settings | Get settings.
[**get_reports**](EnterpriseApi.md#get_reports) | **GET** /reports | List reports.
[**get_role**](EnterpriseApi.md#get_role) | **GET** /access-control/roles/{roleUID} | Get a role.
[**get_role_assignments**](EnterpriseApi.md#get_role_assignments) | **GET** /access-control/roles/{roleUID}/assignments | Get role assignments.
[**get_saml_logout**](EnterpriseApi.md#get_saml_logout) | **GET** /logout/saml | GetLogout initiates single logout process.
[**get_slo**](EnterpriseApi.md#get_slo) | **GET** /saml/slo | It performs Single Logout (SLO) callback.
[**get_status**](EnterpriseApi.md#get_status) | **GET** /licensing/check | Check license availability.
[**get_sync_status**](EnterpriseApi.md#get_sync_status) | **GET** /admin/ldap-sync-status | Returns the current state of the LDAP background sync integration.
[**get_team_groups_api**](EnterpriseApi.md#get_team_groups_api) | **GET** /teams/{teamId}/groups | Get External Groups.
[**list_recording_rules**](EnterpriseApi.md#list_recording_rules) | **GET** /recording-rules | Lists all rules in the database: active or deleted.
[**list_roles**](EnterpriseApi.md#list_roles) | **GET** /access-control/roles | Get all roles.
[**list_team_roles**](EnterpriseApi.md#list_team_roles) | **GET** /access-control/teams/{teamId}/roles | Get team roles.
[**list_teams_roles**](EnterpriseApi.md#list_teams_roles) | **POST** /access-control/teams/roles/search | List roles assigned to multiple teams.
[**list_user_roles**](EnterpriseApi.md#list_user_roles) | **GET** /access-control/users/{userId}/roles | List roles assigned to a user.
[**list_users_roles**](EnterpriseApi.md#list_users_roles) | **POST** /access-control/users/roles/search | List roles assigned to multiple users.
[**post_acs**](EnterpriseApi.md#post_acs) | **POST** /saml/acs | It performs Assertion Consumer Service (ACS).
[**post_license_token**](EnterpriseApi.md#post_license_token) | **POST** /licensing/token | Create license token.
[**post_renew_license_token**](EnterpriseApi.md#post_renew_license_token) | **POST** /licensing/token/renew | Manually force license refresh.
[**post_slo**](EnterpriseApi.md#post_slo) | **POST** /saml/slo | It performs Single Logout (SLO) callback.
[**refresh_license_stats**](EnterpriseApi.md#refresh_license_stats) | **GET** /licensing/refresh-stats | Refresh license stats.
[**remove_team_group_api_query**](EnterpriseApi.md#remove_team_group_api_query) | **DELETE** /teams/{teamId}/groups | Remove External Group.
[**remove_team_role**](EnterpriseApi.md#remove_team_role) | **DELETE** /access-control/teams/{teamId}/roles/{roleUID} | Remove team role.
[**remove_user_role**](EnterpriseApi.md#remove_user_role) | **DELETE** /access-control/users/{userId}/roles/{roleUID} | Remove a user role assignment.
[**render_report_pdfs**](EnterpriseApi.md#render_report_pdfs) | **GET** /reports/render/pdfs | Render report for multiple dashboards.
[**save_report_settings**](EnterpriseApi.md#save_report_settings) | **POST** /reports/settings | Save settings.
[**search_result**](EnterpriseApi.md#search_result) | **POST** /access-control/assignments/search | Debug permissions.
[**send_report**](EnterpriseApi.md#send_report) | **POST** /reports/email | Send a report.
[**send_test_email**](EnterpriseApi.md#send_test_email) | **POST** /reports/test-email | Send test report via email.
[**set_role_assignments**](EnterpriseApi.md#set_role_assignments) | **PUT** /access-control/roles/{roleUID}/assignments | Set role assignments.
[**set_team_roles**](EnterpriseApi.md#set_team_roles) | **PUT** /access-control/teams/{teamId}/roles | Update team role.
[**set_user_roles**](EnterpriseApi.md#set_user_roles) | **PUT** /access-control/users/{userId}/roles | Set user role assignments.
[**test_create_recording_rule**](EnterpriseApi.md#test_create_recording_rule) | **POST** /recording-rules/test | Test a recording rule.
[**update_recording_rule**](EnterpriseApi.md#update_recording_rule) | **PUT** /recording-rules | Update the active status of a rule.
[**update_report**](EnterpriseApi.md#update_report) | **PUT** /reports/{id} | Update a report.
[**update_role**](EnterpriseApi.md#update_role) | **PUT** /access-control/roles/{roleUID} | Update a custom role.



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


## admin_provisioning_reload_access_control

> models::ErrorResponseBody admin_provisioning_reload_access_control()
You need to have a permission with action `provisioning:reload` with scope `provisioners:accesscontrol`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ErrorResponseBody**](ErrorResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_recording_rule

> models::RecordingRuleJson create_recording_rule(recording_rule_json)
Create a recording rule that is then registered and started.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_json** | [**RecordingRuleJson**](RecordingRuleJson.md) |  | [required] |

### Return type

[**models::RecordingRuleJson**](RecordingRuleJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_recording_rule_write_target

> models::PrometheusRemoteWriteTargetJson create_recording_rule_write_target(prometheus_remote_write_target_json)
Create a remote write target.

It returns a 422 if there is not an existing prometheus data source configured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prometheus_remote_write_target_json** | [**PrometheusRemoteWriteTargetJson**](PrometheusRemoteWriteTargetJson.md) |  | [required] |

### Return type

[**models::PrometheusRemoteWriteTargetJson**](PrometheusRemoteWriteTargetJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_report

> models::CreateReport200Response create_report(create_or_update_report_config)
Create a report.

Available to org admins only and with a valid license.  You need to have a permission with action `reports.admin:create`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_or_update_report_config** | [**CreateOrUpdateReportConfig**](CreateOrUpdateReportConfig.md) |  | [required] |

### Return type

[**models::CreateReport200Response**](createReport_200_response.md)

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


## delete_license_token

> models::ErrorResponseBody delete_license_token(delete_token_command)
Remove license from database.

Removes the license stored in the Grafana database. Available in Grafana Enterprise v7.4+.  You need to have a permission with action `licensing:delete`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_token_command** | [**DeleteTokenCommand**](DeleteTokenCommand.md) |  | [required] |

### Return type

[**models::ErrorResponseBody**](ErrorResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_rule

> models::SuccessResponseBody delete_recording_rule(recording_rule_id)
Delete removes the rule from the registry and stops it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_rule_write_target

> models::SuccessResponseBody delete_recording_rule_write_target()
Delete the remote write target.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_report

> models::SuccessResponseBody delete_report(id)
Delete a report.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.delete` with scope `reports:id:<report ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
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


## get_custom_permissions_csv

> get_custom_permissions_csv()
Get custom permissions report in CSV format.

You need to have a permission with action `licensing.reports:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_permissions_report

> get_custom_permissions_report()
Get custom permissions report.

You need to have a permission with action `licensing.reports:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_token

> models::Token get_license_token()
Get license token.

You need to have a permission with action `licensing:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Token**](Token.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata

> Vec<i32> get_metadata()
It exposes the SP (Grafana's) metadata for the IdP's consumption.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_rule_write_target

> models::PrometheusRemoteWriteTargetJson get_recording_rule_write_target()
Return the prometheus remote write target.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PrometheusRemoteWriteTargetJson**](PrometheusRemoteWriteTargetJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report

> models::Report get_report(id)
Get a report.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports:read` with scope `reports:id:<report ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::Report**](Report.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report_settings

> models::ReportSettings get_report_settings()
Get settings.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.settings:read`x.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReportSettings**](ReportSettings.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reports

> Vec<models::Report> get_reports()
List reports.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports:read` with scope `reports:*`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Report>**](Report.md)

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


## get_saml_logout

> get_saml_logout()
GetLogout initiates single logout process.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slo

> get_slo()
It performs Single Logout (SLO) callback.

There might be two possible requests: 1. Logout response (callback) when Grafana initiates single logout and IdP returns response to logout request. 2. Logout request when another SP initiates single logout and IdP sends logout request to the Grafana, or in case of IdP-initiated logout.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status

> get_status()
Check license availability.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## list_recording_rules

> Vec<models::RecordingRuleJson> list_recording_rules()
Lists all rules in the database: active or deleted.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RecordingRuleJson>**](RecordingRuleJSON.md)

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


## post_acs

> post_acs(relay_state)
It performs Assertion Consumer Service (ACS).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**relay_state** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_license_token

> models::Token post_license_token(delete_token_command)
Create license token.

You need to have a permission with action `licensing:update`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_token_command** | [**DeleteTokenCommand**](DeleteTokenCommand.md) |  | [required] |

### Return type

[**models::Token**](Token.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_renew_license_token

> post_renew_license_token(body)
Manually force license refresh.

Manually ask license issuer for a new token. Available in Grafana Enterprise v7.4+.  You need to have a permission with action `licensing:update`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_slo

> post_slo(saml_request, saml_response)
It performs Single Logout (SLO) callback.

There might be two possible requests: 1. Logout response (callback) when Grafana initiates single logout and IdP returns response to logout request. 2. Logout request when another SP initiates single logout and IdP sends logout request to the Grafana, or in case of IdP-initiated logout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**saml_request** | Option<**String**> |  |  |
**saml_response** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_license_stats

> models::ActiveUserStats refresh_license_stats()
Refresh license stats.

You need to have a permission with action `licensing:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ActiveUserStats**](ActiveUserStats.md)

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


## render_report_pdfs

> Vec<i32> render_report_pdfs(dashboard_id, orientation, layout, title, scale_factor, include_tables)
Render report for multiple dashboards.

Available to all users and with a valid license.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | Option<**String**> |  |  |
**orientation** | Option<**String**> |  |  |
**layout** | Option<**String**> |  |  |
**title** | Option<**String**> |  |  |
**scale_factor** | Option<**String**> |  |  |
**include_tables** | Option<**String**> |  |  |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_report_settings

> models::SuccessResponseBody save_report_settings(report_settings)
Save settings.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.settings:write`xx.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_settings** | [**ReportSettings**](ReportSettings.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_result

> models::SearchResult search_result()
Debug permissions.

Returns the result of the search through access-control role assignments.  You need to have a permission with action `teams.roles:read` on scope `teams:*` and a permission with action `users.roles:read` on scope `users:*`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_report

> models::SuccessResponseBody send_report(report_email)
Send a report.

Generate and send a report. This API waits for the report to be generated before returning. We recommend that you set the client’s timeout to at least 60 seconds. Available to org admins only and with a valid license.  Only available in Grafana Enterprise v7.0+. This API endpoint is experimental and may be deprecated in a future release. On deprecation, a migration strategy will be provided and the endpoint will remain functional until the next major release of Grafana.  You need to have a permission with action `reports:send`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_email** | [**ReportEmail**](ReportEmail.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_email

> models::SuccessResponseBody send_test_email(create_or_update_report_config)
Send test report via email.

Available to org admins only and with a valid license.  You need to have a permission with action `reports:send`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_or_update_report_config** | [**CreateOrUpdateReportConfig**](CreateOrUpdateReportConfig.md) |  | [required] |

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


## test_create_recording_rule

> models::SuccessResponseBody test_create_recording_rule(recording_rule_json)
Test a recording rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_json** | [**RecordingRuleJson**](RecordingRuleJson.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_recording_rule

> models::RecordingRuleJson update_recording_rule(recording_rule_json)
Update the active status of a rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_rule_json** | [**RecordingRuleJson**](RecordingRuleJson.md) |  | [required] |

### Return type

[**models::RecordingRuleJson**](RecordingRuleJSON.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_report

> models::SuccessResponseBody update_report(id, create_or_update_report_config)
Update a report.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.admin:write` with scope `reports:id:<report ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**create_or_update_report_config** | [**CreateOrUpdateReportConfig**](CreateOrUpdateReportConfig.md) |  | [required] |

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

