# AlertingRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_at** | **String** |  | 
**alerts** | Option<[**Vec<models::Alert>**](Alert.md)> |  | [optional]
**annotations** | **std::collections::HashMap<String, String>** | The custom marshaling for labels.Labels ends up doing this anyways. | 
**duration** | Option<**f64**> |  | [optional]
**evaluation_time** | Option<**f64**> |  | [optional]
**health** | **String** |  | 
**labels** | Option<**std::collections::HashMap<String, String>**> | The custom marshaling for labels.Labels ends up doing this anyways. | [optional]
**last_error** | Option<**String**> |  | [optional]
**last_evaluation** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**query** | **String** |  | 
**state** | **String** | State can be \"pending\", \"firing\", \"inactive\". | 
**totals** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**totals_filtered** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


