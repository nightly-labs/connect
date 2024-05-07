# InternalDataLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**datasource_name** | Option<**String**> |  | [optional]
**datasource_uid** | Option<**String**> |  | [optional]
**panels_state** | Option<[**serde_json::Value**](.md)> | This is an object constructed with the keys as the values of the enum VisType and the value being a bag of properties | [optional]
**query** | Option<[**serde_json::Value**](.md)> |  | [optional]
**time_range** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**transformations** | Option<[**Vec<models::LinkTransformationConfig>**](LinkTransformationConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


