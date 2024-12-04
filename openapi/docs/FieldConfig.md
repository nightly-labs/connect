# FieldConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**color** | Option<[**serde_json::Value**](.md)> | Map values to a display color NOTE: this interface is under development in the frontend... so simple map for now | [optional]
**custom** | Option<[**serde_json::Value**](.md)> | Panel Specific Values | [optional]
**decimals** | Option<**i32**> |  | [optional]
**description** | Option<**String**> | Description is human readable field metadata | [optional]
**display_name** | Option<**String**> | DisplayName overrides Grafana default naming, should not be used from a data source | [optional]
**display_name_from_ds** | Option<**String**> | DisplayNameFromDS overrides Grafana default naming strategy. | [optional]
**filterable** | Option<**bool**> | Filterable indicates if the Field's data can be filtered by additional calls. | [optional]
**interval** | Option<**f64**> | Interval indicates the expected regular step between values in the series. When an interval exists, consumers can identify \"missing\" values when the expected value is not present. The grafana timeseries visualization will render disconnected values when missing values are found it the time field. The interval uses the same units as the values.  For time.Time, this is defined in milliseconds. | [optional]
**links** | Option<[**Vec<models::DataLink>**](DataLink.md)> | The behavior when clicking on a result | [optional]
**mappings** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**max** | Option<**f64**> | ConfFloat64 is a float64. It Marshals float64 values of NaN of Inf to null. | [optional]
**min** | Option<**f64**> | ConfFloat64 is a float64. It Marshals float64 values of NaN of Inf to null. | [optional]
**no_value** | Option<**String**> | Alternative to empty string | [optional]
**path** | Option<**String**> | Path is an explicit path to the field in the datasource. When the frame meta includes a path, this will default to `${frame.meta.path}/${field.name}  When defined, this value can be used as an identifier within the datasource scope, and may be used as an identifier to update values in a subsequent request | [optional]
**thresholds** | Option<[**models::ThresholdsConfig**](ThresholdsConfig.md)> |  | [optional]
**r#type** | Option<[**models::FieldTypeConfig**](FieldTypeConfig.md)> |  | [optional]
**unit** | Option<**String**> | Numeric Options | [optional]
**writeable** | Option<**bool**> | Writeable indicates that the datasource knows how to update this value | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


