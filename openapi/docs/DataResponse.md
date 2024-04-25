# DataResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | Option<**String**> | Error is a property to be set if the corresponding DataQuery has an error. | [optional]
**error_source** | Option<**String**> | ErrorSource type defines the source of the error | [optional]
**frames** | Option<[**Vec<models::Frame>**](Frame.md)> | It is the main data container within a backend.DataResponse. There should be no `nil` entries in the Frames slice (making them pointers was a mistake). | [optional]
**status** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


