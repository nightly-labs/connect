# RuleGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**evaluation_time** | Option<**f64**> |  | [optional]
**file** | **String** |  | 
**interval** | **f64** |  | 
**last_evaluation** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**rules** | [**Vec<models::AlertingRule>**](AlertingRule.md) | In order to preserve rule ordering, while exposing type (alerting or recording) specific properties, both alerting and recording rules are exposed in the same array. | 
**totals** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


