# RouteExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#continue** | Option<**bool**> |  | [optional]
**group_by** | Option<**Vec<String>**> |  | [optional]
**group_interval** | Option<**String**> |  | [optional]
**group_wait** | Option<**String**> |  | [optional]
**r#match** | Option<**std::collections::HashMap<String, String>**> | Deprecated. Remove before v1.0 release. | [optional]
**match_re** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**matchers** | Option<[**Vec<models::Matcher>**](Matcher.md)> | Matchers is a slice of Matchers that is sortable, implements Stringer, and provides a Matches method to match a LabelSet against all Matchers in the slice. Note that some users of Matchers might require it to be sorted. | [optional]
**mute_time_intervals** | Option<**Vec<String>**> |  | [optional]
**object_matchers** | Option<[**Vec<Vec<String>>**](Vec.md)> |  | [optional]
**receiver** | Option<**String**> |  | [optional]
**repeat_interval** | Option<**String**> |  | [optional]
**routes** | Option<[**Vec<models::RouteExport>**](RouteExport.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


