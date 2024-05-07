# GettableApiAlertingConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**global** | Option<[**models::GlobalConfig**](GlobalConfig.md)> |  | [optional]
**inhibit_rules** | Option<[**Vec<models::InhibitRule>**](InhibitRule.md)> |  | [optional]
**mute_time_provenances** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**mute_time_intervals** | Option<[**Vec<models::MuteTimeInterval>**](MuteTimeInterval.md)> | MuteTimeIntervals is deprecated and will be removed before Alertmanager 1.0. | [optional]
**receivers** | Option<[**Vec<models::GettableApiReceiver>**](GettableApiReceiver.md)> | Override with our superset receiver type | [optional]
**route** | Option<[**models::Route**](Route.md)> |  | [optional]
**templates** | Option<**Vec<String>**> |  | [optional]
**time_intervals** | Option<[**Vec<models::TimeInterval>**](TimeInterval.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


