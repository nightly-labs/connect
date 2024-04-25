# CreateLibraryElementCommand

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**folder_id** | Option<**i64**> | ID of the folder where the library element is stored.  Deprecated: use FolderUID instead | [optional]
**folder_uid** | Option<**String**> | UID of the folder where the library element is stored. | [optional]
**kind** | Option<**i64**> | Kind of element to create, Use 1 for library panels or 2 for c. Description: 1 - library panels 2 - library variables | [optional]
**model** | Option<[**serde_json::Value**](.md)> | The JSON model for the library element. | [optional]
**name** | Option<**String**> | Name of the library element. | [optional]
**uid** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


