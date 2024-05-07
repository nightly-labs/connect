# CreateCorrelationCommand

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**models::CorrelationConfig**](CorrelationConfig.md)> |  | [optional]
**description** | Option<**String**> | Optional description of the correlation | [optional]
**label** | Option<**String**> | Optional label identifying the correlation | [optional]
**provisioned** | Option<**bool**> | True if correlation was created with provisioning. This makes it read-only. | [optional]
**target_uid** | Option<**String**> | Target data source UID to which the correlation is created. required if config.type = query | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


