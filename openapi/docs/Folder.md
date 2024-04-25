# Folder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_control** | Option<**std::collections::HashMap<String, bool>**> | Metadata contains user accesses for a given resource Ex: map[string]bool{\"create\":true, \"delete\": true} | [optional]
**can_admin** | Option<**bool**> |  | [optional]
**can_delete** | Option<**bool**> |  | [optional]
**can_edit** | Option<**bool**> |  | [optional]
**can_save** | Option<**bool**> |  | [optional]
**created** | Option<**String**> |  | [optional]
**created_by** | Option<**String**> |  | [optional]
**has_acl** | Option<**bool**> |  | [optional]
**id** | Option<**i64**> | Deprecated: use UID instead | [optional]
**org_id** | Option<**i64**> |  | [optional]
**parent_uid** | Option<**String**> | only used if nested folders are enabled | [optional]
**parents** | Option<[**Vec<models::Folder>**](Folder.md)> | the parent folders starting from the root going down | [optional]
**title** | Option<**String**> |  | [optional]
**uid** | Option<**String**> |  | [optional]
**updated** | Option<**String**> |  | [optional]
**updated_by** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**version** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


