# WebhookConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**http_config** | Option<[**models::HttpClientConfig**](HTTPClientConfig.md)> |  | [optional]
**max_alerts** | Option<**i32**> | MaxAlerts is the maximum number of alerts to be sent per webhook message. Alerts exceeding this threshold will be truncated. Setting this to 0 allows an unlimited number of alerts. | [optional]
**send_resolved** | Option<**bool**> |  | [optional]
**url** | Option<[**models::Url**](URL.md)> |  | [optional]
**url_file** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


