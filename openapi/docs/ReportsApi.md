# \ReportsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_report**](ReportsApi.md#create_report) | **POST** /reports | Create a report.
[**delete_report**](ReportsApi.md#delete_report) | **DELETE** /reports/{id} | Delete a report.
[**get_report**](ReportsApi.md#get_report) | **GET** /reports/{id} | Get a report.
[**get_report_settings**](ReportsApi.md#get_report_settings) | **GET** /reports/settings | Get settings.
[**get_reports**](ReportsApi.md#get_reports) | **GET** /reports | List reports.
[**render_report_pdfs**](ReportsApi.md#render_report_pdfs) | **GET** /reports/render/pdfs | Render report for multiple dashboards.
[**save_report_settings**](ReportsApi.md#save_report_settings) | **POST** /reports/settings | Save settings.
[**send_report**](ReportsApi.md#send_report) | **POST** /reports/email | Send a report.
[**send_test_email**](ReportsApi.md#send_test_email) | **POST** /reports/test-email | Send test report via email.
[**update_report**](ReportsApi.md#update_report) | **PUT** /reports/{id} | Update a report.



## create_report

> models::CreateReport200Response create_report(create_or_update_report_config)
Create a report.

Available to org admins only and with a valid license.  You need to have a permission with action `reports.admin:create`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_or_update_report_config** | [**CreateOrUpdateReportConfig**](CreateOrUpdateReportConfig.md) |  | [required] |

### Return type

[**models::CreateReport200Response**](createReport_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_report

> models::SuccessResponseBody delete_report(id)
Delete a report.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.delete` with scope `reports:id:<report ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report

> models::Report get_report(id)
Get a report.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports:read` with scope `reports:id:<report ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::Report**](Report.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report_settings

> models::ReportSettings get_report_settings()
Get settings.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.settings:read`x.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReportSettings**](ReportSettings.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reports

> Vec<models::Report> get_reports()
List reports.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports:read` with scope `reports:*`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Report>**](Report.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_report_pdfs

> Vec<i32> render_report_pdfs(dashboard_id, orientation, layout, title, scale_factor, include_tables)
Render report for multiple dashboards.

Available to all users and with a valid license.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | Option<**String**> |  |  |
**orientation** | Option<**String**> |  |  |
**layout** | Option<**String**> |  |  |
**title** | Option<**String**> |  |  |
**scale_factor** | Option<**String**> |  |  |
**include_tables** | Option<**String**> |  |  |

### Return type

**Vec<i32>**

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_report_settings

> models::SuccessResponseBody save_report_settings(report_settings)
Save settings.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.settings:write`xx.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_settings** | [**ReportSettings**](ReportSettings.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_report

> models::SuccessResponseBody send_report(report_email)
Send a report.

Generate and send a report. This API waits for the report to be generated before returning. We recommend that you set the client’s timeout to at least 60 seconds. Available to org admins only and with a valid license.  Only available in Grafana Enterprise v7.0+. This API endpoint is experimental and may be deprecated in a future release. On deprecation, a migration strategy will be provided and the endpoint will remain functional until the next major release of Grafana.  You need to have a permission with action `reports:send`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_email** | [**ReportEmail**](ReportEmail.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_email

> models::SuccessResponseBody send_test_email(create_or_update_report_config)
Send test report via email.

Available to org admins only and with a valid license.  You need to have a permission with action `reports:send`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_or_update_report_config** | [**CreateOrUpdateReportConfig**](CreateOrUpdateReportConfig.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_report

> models::SuccessResponseBody update_report(id, create_or_update_report_config)
Update a report.

Available to org admins only and with a valid or expired license.  You need to have a permission with action `reports.admin:write` with scope `reports:id:<report ID>`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**create_or_update_report_config** | [**CreateOrUpdateReportConfig**](CreateOrUpdateReportConfig.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

