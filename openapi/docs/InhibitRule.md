# InhibitRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**equal** | Option<**Vec<String>**> |  | [optional]
**source_match** | Option<**std::collections::HashMap<String, String>**> | SourceMatch defines a set of labels that have to equal the given value for source alerts. Deprecated. Remove before v1.0 release. | [optional]
**source_match_re** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**source_matchers** | Option<[**Vec<models::Matcher>**](Matcher.md)> | Matchers is a slice of Matchers that is sortable, implements Stringer, and provides a Matches method to match a LabelSet against all Matchers in the slice. Note that some users of Matchers might require it to be sorted. | [optional]
**target_match** | Option<**std::collections::HashMap<String, String>**> | TargetMatch defines a set of labels that have to equal the given value for target alerts. Deprecated. Remove before v1.0 release. | [optional]
**target_match_re** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**target_matchers** | Option<[**Vec<models::Matcher>**](Matcher.md)> | Matchers is a slice of Matchers that is sortable, implements Stringer, and provides a Matches method to match a LabelSet against all Matchers in the slice. Note that some users of Matchers might require it to be sorted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


