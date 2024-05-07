# Frame

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fields** | Option<[**Vec<models::Field>**](Field.md)> | Fields are the columns of a frame. All Fields must be of the same the length when marshalling the Frame for transmission. There should be no `nil` entries in the Fields slice (making them pointers was a mistake). | [optional]
**meta** | Option<[**models::FrameMeta**](FrameMeta.md)> |  | [optional]
**name** | Option<**String**> | Name is used in some Grafana visualizations. | [optional]
**ref_id** | Option<**String**> | RefID is a property that can be set to match a Frame to its originating query. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


