# FrameMeta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel** | Option<**String**> | Channel is the path to a stream in grafana live that has real-time updates for this data. | [optional]
**custom** | Option<[**serde_json::Value**](.md)> | Custom datasource specific values. | [optional]
**data_topic** | Option<**String**> | nolint:revive | [optional]
**executed_query_string** | Option<**String**> | ExecutedQueryString is the raw query sent to the underlying system. All macros and templating have been applied.  When metadata contains this value, it will be shown in the query inspector. | [optional]
**notices** | Option<[**Vec<models::Notice>**](Notice.md)> | Notices provide additional information about the data in the Frame that Grafana can display to the user in the user interface. | [optional]
**path** | Option<**String**> | Path is a browsable path on the datasource. | [optional]
**path_separator** | Option<**String**> | PathSeparator defines the separator pattern to decode a hierarchy. The default separator is '/'. | [optional]
**preferred_visualisation_plugin_id** | Option<**String**> | PreferredVisualizationPluginId sets the panel plugin id to use to render the data when using Explore. If the plugin cannot be found will fall back to PreferredVisualization. | [optional]
**preferred_visualisation_type** | Option<**String**> |  | [optional]
**stats** | Option<[**Vec<models::QueryStat>**](QueryStat.md)> | Stats is an array of query result statistics. | [optional]
**r#type** | Option<**String**> | A FrameType string, when present in a frame's metadata, asserts that the frame's structure conforms to the FrameType's specification. This property is currently optional, so FrameType may be FrameTypeUnknown even if the properties of the Frame correspond to a defined FrameType. +enum | [optional]
**type_version** | Option<**Vec<i32>**> |  | [optional]
**unique_row_id_fields** | Option<**Vec<i64>**> | Array of field indices which values create a unique id for each row. Ideally this should be globally unique ID but that isn't guarantied. Should help with keeping track and deduplicating rows in visualizations, especially with streaming data with frequent updates. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


