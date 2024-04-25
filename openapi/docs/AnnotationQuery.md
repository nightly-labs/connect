# AnnotationQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**built_in** | Option<**f32**> | Set to 1 for the standard annotation query all dashboards have by default. | [optional]
**datasource** | Option<[**models::DataSourceRef**](DataSourceRef.md)> |  | [optional]
**enable** | Option<**bool**> | When enabled the annotation query is issued with every dashboard refresh | [optional]
**filter** | Option<[**models::AnnotationPanelFilter**](AnnotationPanelFilter.md)> |  | [optional]
**hide** | Option<**bool**> | Annotation queries can be toggled on or off at the top of the dashboard. When hide is true, the toggle is not shown in the dashboard. | [optional]
**icon_color** | Option<**String**> | Color to use for the annotation event markers | [optional]
**name** | Option<**String**> | Name of annotation. | [optional]
**target** | Option<[**models::AnnotationTarget**](AnnotationTarget.md)> |  | [optional]
**r#type** | Option<**String**> | TODO -- this should not exist here, it is based on the --grafana-- datasource | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


