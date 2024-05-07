# Name

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country** | Option<**Vec<String>**> |  | [optional]
**extra_names** | Option<[**Vec<models::AttributeTypeAndValue>**](AttributeTypeAndValue.md)> | ExtraNames contains attributes to be copied, raw, into any marshaled distinguished names. Values override any attributes with the same OID. The ExtraNames field is not populated when parsing, see Names. | [optional]
**locality** | Option<**Vec<String>**> |  | [optional]
**names** | Option<[**Vec<models::AttributeTypeAndValue>**](AttributeTypeAndValue.md)> | Names contains all parsed attributes. When parsing distinguished names, this can be used to extract non-standard attributes that are not parsed by this package. When marshaling to RDNSequences, the Names field is ignored, see ExtraNames. | [optional]
**serial_number** | Option<**String**> |  | [optional]
**street_address** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


