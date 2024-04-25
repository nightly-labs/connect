# AlertQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**datasource_uid** | Option<**String**> | Grafana data source unique identifier; it should be '__expr__' for a Server Side Expression operation. | [optional]
**model** | Option<[**serde_json::Value**](.md)> | JSON is the raw JSON query and includes the above properties as well as custom properties. | [optional]
**query_type** | Option<**String**> | QueryType is an optional identifier for the type of query. It can be used to distinguish different types of queries. | [optional]
**ref_id** | Option<**String**> | RefID is the unique identifier of the query, set by the frontend call. | [optional]
**relative_time_range** | Option<[**models::RelativeTimeRange**](RelativeTimeRange.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


