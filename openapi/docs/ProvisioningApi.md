# \ProvisioningApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**route_delete_alert_rule**](ProvisioningApi.md#route_delete_alert_rule) | **DELETE** /v1/provisioning/alert-rules/{UID} | Delete a specific alert rule by UID.
[**route_delete_alert_rule_group**](ProvisioningApi.md#route_delete_alert_rule_group) | **DELETE** /v1/provisioning/folder/{FolderUID}/rule-groups/{Group} | 
[**route_delete_contactpoints**](ProvisioningApi.md#route_delete_contactpoints) | **DELETE** /v1/provisioning/contact-points/{UID} | Delete a contact point.
[**route_delete_mute_timing**](ProvisioningApi.md#route_delete_mute_timing) | **DELETE** /v1/provisioning/mute-timings/{name} | Delete a mute timing.
[**route_delete_template**](ProvisioningApi.md#route_delete_template) | **DELETE** /v1/provisioning/templates/{name} | Delete a template.
[**route_export_mute_timing**](ProvisioningApi.md#route_export_mute_timing) | **GET** /v1/provisioning/mute-timings/{name}/export | Export a mute timing in provisioning format.
[**route_export_mute_timings**](ProvisioningApi.md#route_export_mute_timings) | **GET** /v1/provisioning/mute-timings/export | Export all mute timings in provisioning format.
[**route_get_alert_rule**](ProvisioningApi.md#route_get_alert_rule) | **GET** /v1/provisioning/alert-rules/{UID} | Get a specific alert rule by UID.
[**route_get_alert_rule_export**](ProvisioningApi.md#route_get_alert_rule_export) | **GET** /v1/provisioning/alert-rules/{UID}/export | Export an alert rule in provisioning file format.
[**route_get_alert_rule_group**](ProvisioningApi.md#route_get_alert_rule_group) | **GET** /v1/provisioning/folder/{FolderUID}/rule-groups/{Group} | Get a rule group.
[**route_get_alert_rule_group_export**](ProvisioningApi.md#route_get_alert_rule_group_export) | **GET** /v1/provisioning/folder/{FolderUID}/rule-groups/{Group}/export | Export an alert rule group in provisioning file format.
[**route_get_alert_rules**](ProvisioningApi.md#route_get_alert_rules) | **GET** /v1/provisioning/alert-rules | Get all the alert rules.
[**route_get_alert_rules_export**](ProvisioningApi.md#route_get_alert_rules_export) | **GET** /v1/provisioning/alert-rules/export | Export all alert rules in provisioning file format.
[**route_get_contactpoints**](ProvisioningApi.md#route_get_contactpoints) | **GET** /v1/provisioning/contact-points | Get all the contact points.
[**route_get_contactpoints_export**](ProvisioningApi.md#route_get_contactpoints_export) | **GET** /v1/provisioning/contact-points/export | Export all contact points in provisioning file format.
[**route_get_mute_timing**](ProvisioningApi.md#route_get_mute_timing) | **GET** /v1/provisioning/mute-timings/{name} | Get a mute timing.
[**route_get_mute_timings**](ProvisioningApi.md#route_get_mute_timings) | **GET** /v1/provisioning/mute-timings | Get all the mute timings.
[**route_get_policy_tree**](ProvisioningApi.md#route_get_policy_tree) | **GET** /v1/provisioning/policies | Get the notification policy tree.
[**route_get_policy_tree_export**](ProvisioningApi.md#route_get_policy_tree_export) | **GET** /v1/provisioning/policies/export | Export the notification policy tree in provisioning file format.
[**route_get_template**](ProvisioningApi.md#route_get_template) | **GET** /v1/provisioning/templates/{name} | Get a notification template.
[**route_get_templates**](ProvisioningApi.md#route_get_templates) | **GET** /v1/provisioning/templates | Get all notification templates.
[**route_post_alert_rule**](ProvisioningApi.md#route_post_alert_rule) | **POST** /v1/provisioning/alert-rules | Create a new alert rule.
[**route_post_contactpoints**](ProvisioningApi.md#route_post_contactpoints) | **POST** /v1/provisioning/contact-points | Create a contact point.
[**route_post_mute_timing**](ProvisioningApi.md#route_post_mute_timing) | **POST** /v1/provisioning/mute-timings | Create a new mute timing.
[**route_put_alert_rule**](ProvisioningApi.md#route_put_alert_rule) | **PUT** /v1/provisioning/alert-rules/{UID} | Update an existing alert rule.
[**route_put_alert_rule_group**](ProvisioningApi.md#route_put_alert_rule_group) | **PUT** /v1/provisioning/folder/{FolderUID}/rule-groups/{Group} | Create or update alert rule group.
[**route_put_contactpoint**](ProvisioningApi.md#route_put_contactpoint) | **PUT** /v1/provisioning/contact-points/{UID} | Update an existing contact point.
[**route_put_mute_timing**](ProvisioningApi.md#route_put_mute_timing) | **PUT** /v1/provisioning/mute-timings/{name} | Replace an existing mute timing.
[**route_put_policy_tree**](ProvisioningApi.md#route_put_policy_tree) | **PUT** /v1/provisioning/policies | Sets the notification policy tree.
[**route_put_template**](ProvisioningApi.md#route_put_template) | **PUT** /v1/provisioning/templates/{name} | Updates an existing notification template.
[**route_reset_policy_tree**](ProvisioningApi.md#route_reset_policy_tree) | **DELETE** /v1/provisioning/policies | Clears the notification policy tree.



## route_delete_alert_rule

> route_delete_alert_rule(uid, x_disable_provenance)
Delete a specific alert rule by UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Alert rule UID | [required] |
**x_disable_provenance** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_delete_alert_rule_group

> route_delete_alert_rule_group(folder_uid, group)


Delete rule group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**group** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_delete_contactpoints

> route_delete_contactpoints(uid)
Delete a contact point.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | UID is the contact point unique identifier | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_delete_mute_timing

> route_delete_mute_timing(name)
Delete a mute timing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Mute timing name | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_delete_template

> route_delete_template(name)
Delete a template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Template Name | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_export_mute_timing

> models::AlertingFileExport route_export_mute_timing(name, download, format)
Export a mute timing in provisioning format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Mute timing name | [required] |
**download** | Option<**bool**> | Whether to initiate a download of the file or not. |  |[default to false]
**format** | Option<**String**> | Format of the downloaded file. Supported yaml, json or hcl. Accept header can also be used, but the query parameter will take precedence. |  |[default to yaml]

### Return type

[**models::AlertingFileExport**](AlertingFileExport.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/terraform+hcl, application/yaml, text/hcl, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_export_mute_timings

> models::AlertingFileExport route_export_mute_timings(download, format)
Export all mute timings in provisioning format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download** | Option<**bool**> | Whether to initiate a download of the file or not. |  |[default to false]
**format** | Option<**String**> | Format of the downloaded file. Supported yaml, json or hcl. Accept header can also be used, but the query parameter will take precedence. |  |[default to yaml]

### Return type

[**models::AlertingFileExport**](AlertingFileExport.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/terraform+hcl, application/yaml, text/hcl, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_alert_rule

> models::ProvisionedAlertRule route_get_alert_rule(uid)
Get a specific alert rule by UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Alert rule UID | [required] |

### Return type

[**models::ProvisionedAlertRule**](ProvisionedAlertRule.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_alert_rule_export

> models::AlertingFileExport route_get_alert_rule_export(uid, download, format)
Export an alert rule in provisioning file format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Alert rule UID | [required] |
**download** | Option<**bool**> | Whether to initiate a download of the file or not. |  |[default to false]
**format** | Option<**String**> | Format of the downloaded file. Supported yaml, json or hcl. Accept header can also be used, but the query parameter will take precedence. |  |[default to yaml]

### Return type

[**models::AlertingFileExport**](AlertingFileExport.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/terraform+hcl, application/yaml, text/hcl, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_alert_rule_group

> models::AlertRuleGroup route_get_alert_rule_group(folder_uid, group)
Get a rule group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**group** | **String** |  | [required] |

### Return type

[**models::AlertRuleGroup**](AlertRuleGroup.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_alert_rule_group_export

> models::AlertingFileExport route_get_alert_rule_group_export(folder_uid, group, download, format)
Export an alert rule group in provisioning file format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**group** | **String** |  | [required] |
**download** | Option<**bool**> | Whether to initiate a download of the file or not. |  |[default to false]
**format** | Option<**String**> | Format of the downloaded file. Supported yaml, json or hcl. Accept header can also be used, but the query parameter will take precedence. |  |[default to yaml]

### Return type

[**models::AlertingFileExport**](AlertingFileExport.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/terraform+hcl, application/yaml, text/hcl, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_alert_rules

> Vec<models::ProvisionedAlertRule> route_get_alert_rules()
Get all the alert rules.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProvisionedAlertRule>**](ProvisionedAlertRule.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_alert_rules_export

> models::AlertingFileExport route_get_alert_rules_export(download, format, folder_uid, group, rule_uid)
Export all alert rules in provisioning file format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download** | Option<**bool**> | Whether to initiate a download of the file or not. |  |[default to false]
**format** | Option<**String**> | Format of the downloaded file. Supported yaml, json or hcl. Accept header can also be used, but the query parameter will take precedence. |  |[default to yaml]
**folder_uid** | Option<[**Vec<String>**](String.md)> | UIDs of folders from which to export rules |  |
**group** | Option<**String**> | Name of group of rules to export. Must be specified only together with a single folder UID |  |
**rule_uid** | Option<**String**> | UID of alert rule to export. If specified, parameters folderUid and group must be empty. |  |

### Return type

[**models::AlertingFileExport**](AlertingFileExport.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/terraform+hcl, application/yaml, text/hcl, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_contactpoints

> Vec<models::EmbeddedContactPoint> route_get_contactpoints(name)
Get all the contact points.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Filter by name |  |

### Return type

[**Vec<models::EmbeddedContactPoint>**](EmbeddedContactPoint.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_contactpoints_export

> models::AlertingFileExport route_get_contactpoints_export(download, format, decrypt, name)
Export all contact points in provisioning file format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download** | Option<**bool**> | Whether to initiate a download of the file or not. |  |[default to false]
**format** | Option<**String**> | Format of the downloaded file. Supported yaml, json or hcl. Accept header can also be used, but the query parameter will take precedence. |  |[default to yaml]
**decrypt** | Option<**bool**> | Whether any contained secure settings should be decrypted or left redacted. Redacted settings will contain RedactedValue instead. Currently, only org admin can view decrypted secure settings. |  |[default to false]
**name** | Option<**String**> | Filter by name |  |

### Return type

[**models::AlertingFileExport**](AlertingFileExport.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/terraform+hcl, application/yaml, text/hcl, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_mute_timing

> models::MuteTimeInterval route_get_mute_timing(name)
Get a mute timing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Mute timing name | [required] |

### Return type

[**models::MuteTimeInterval**](MuteTimeInterval.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_mute_timings

> Vec<models::MuteTimeInterval> route_get_mute_timings()
Get all the mute timings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MuteTimeInterval>**](MuteTimeInterval.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_policy_tree

> models::Route route_get_policy_tree()
Get the notification policy tree.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Route**](Route.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_policy_tree_export

> models::AlertingFileExport route_get_policy_tree_export()
Export the notification policy tree in provisioning file format.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AlertingFileExport**](AlertingFileExport.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/terraform+hcl, application/yaml, text/hcl, text/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_template

> models::NotificationTemplate route_get_template(name)
Get a notification template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Template Name | [required] |

### Return type

[**models::NotificationTemplate**](NotificationTemplate.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_get_templates

> Vec<models::NotificationTemplate> route_get_templates()
Get all notification templates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NotificationTemplate>**](NotificationTemplate.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_post_alert_rule

> models::ProvisionedAlertRule route_post_alert_rule(x_disable_provenance, provisioned_alert_rule)
Create a new alert rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_disable_provenance** | Option<**String**> |  |  |
**provisioned_alert_rule** | Option<[**ProvisionedAlertRule**](ProvisionedAlertRule.md)> |  |  |

### Return type

[**models::ProvisionedAlertRule**](ProvisionedAlertRule.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_post_contactpoints

> models::EmbeddedContactPoint route_post_contactpoints(x_disable_provenance, embedded_contact_point)
Create a contact point.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_disable_provenance** | Option<**String**> |  |  |
**embedded_contact_point** | Option<[**EmbeddedContactPoint**](EmbeddedContactPoint.md)> |  |  |

### Return type

[**models::EmbeddedContactPoint**](EmbeddedContactPoint.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_post_mute_timing

> models::MuteTimeInterval route_post_mute_timing(x_disable_provenance, mute_time_interval)
Create a new mute timing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_disable_provenance** | Option<**String**> |  |  |
**mute_time_interval** | Option<[**MuteTimeInterval**](MuteTimeInterval.md)> |  |  |

### Return type

[**models::MuteTimeInterval**](MuteTimeInterval.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_put_alert_rule

> models::ProvisionedAlertRule route_put_alert_rule(uid, x_disable_provenance, provisioned_alert_rule)
Update an existing alert rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | Alert rule UID | [required] |
**x_disable_provenance** | Option<**String**> |  |  |
**provisioned_alert_rule** | Option<[**ProvisionedAlertRule**](ProvisionedAlertRule.md)> |  |  |

### Return type

[**models::ProvisionedAlertRule**](ProvisionedAlertRule.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_put_alert_rule_group

> models::AlertRuleGroup route_put_alert_rule_group(folder_uid, group, x_disable_provenance, alert_rule_group)
Create or update alert rule group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**group** | **String** |  | [required] |
**x_disable_provenance** | Option<**String**> |  |  |
**alert_rule_group** | Option<[**AlertRuleGroup**](AlertRuleGroup.md)> |  |  |

### Return type

[**models::AlertRuleGroup**](AlertRuleGroup.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_put_contactpoint

> serde_json::Value route_put_contactpoint(uid, x_disable_provenance, embedded_contact_point)
Update an existing contact point.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** | UID is the contact point unique identifier | [required] |
**x_disable_provenance** | Option<**String**> |  |  |
**embedded_contact_point** | Option<[**EmbeddedContactPoint**](EmbeddedContactPoint.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_put_mute_timing

> models::MuteTimeInterval route_put_mute_timing(name, x_disable_provenance, mute_time_interval)
Replace an existing mute timing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Mute timing name | [required] |
**x_disable_provenance** | Option<**String**> |  |  |
**mute_time_interval** | Option<[**MuteTimeInterval**](MuteTimeInterval.md)> |  |  |

### Return type

[**models::MuteTimeInterval**](MuteTimeInterval.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_put_policy_tree

> serde_json::Value route_put_policy_tree(x_disable_provenance, route)
Sets the notification policy tree.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_disable_provenance** | Option<**String**> |  |  |
**route** | Option<[**Route**](Route.md)> | The new notification routing tree to use |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_put_template

> models::NotificationTemplate route_put_template(name, x_disable_provenance, notification_template_content)
Updates an existing notification template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Template Name | [required] |
**x_disable_provenance** | Option<**String**> |  |  |
**notification_template_content** | Option<[**NotificationTemplateContent**](NotificationTemplateContent.md)> |  |  |

### Return type

[**models::NotificationTemplate**](NotificationTemplate.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## route_reset_policy_tree

> serde_json::Value route_reset_policy_tree()
Clears the notification policy tree.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

