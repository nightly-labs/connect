# PlaylistItemDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | Title is an unused property -- it will be removed in the future | [optional]
**r#type** | Option<**String**> | Type of the item. | [optional]
**value** | Option<**String**> | Value depends on type and describes the playlist item.  dashboard_by_id: The value is an internal numerical identifier set by Grafana. This is not portable as the numerical identifier is non-deterministic between different instances. Will be replaced by dashboard_by_uid in the future. (deprecated) dashboard_by_tag: The value is a tag which is set on any number of dashboards. All dashboards behind the tag will be added to the playlist. dashboard_by_uid: The value is the dashboard UID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


