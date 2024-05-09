# ProvisionedAlertRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**annotations** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**condition** | **String** |  | 
**data** | [**Vec<models::AlertQuery>**](AlertQuery.md) |  | 
**exec_err_state** | **String** |  | 
**folder_uid** | **String** |  | 
**r#for** | **i64** | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | 
**id** | Option<**i64**> |  | [optional]
**is_paused** | Option<**bool**> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**no_data_state** | **String** |  | 
**notification_settings** | Option<[**models::AlertRuleNotificationSettings**](AlertRuleNotificationSettings.md)> |  | [optional]
**org_id** | **i64** |  | 
**provenance** | Option<**String**> |  | [optional]
**rule_group** | **String** |  | 
**title** | **String** |  | 
**uid** | Option<**String**> |  | [optional]
**updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


