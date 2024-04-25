# \SearchApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_sort_options**](SearchApi.md#list_sort_options) | **GET** /search/sorting | List search sorting options.
[**search**](SearchApi.md#search) | **GET** /search | 



## list_sort_options

> models::ListSortOptions200Response list_sort_options()
List search sorting options.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListSortOptions200Response**](listSortOptions_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> Vec<models::Hit> search(query, tag, r#type, dashboard_ids, dashboard_uids, folder_ids, folder_uids, starred, limit, page, permission, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Search Query |  |
**tag** | Option<[**Vec<String>**](String.md)> | List of tags to search for |  |
**r#type** | Option<**String**> | Type to search for, dash-folder or dash-db |  |
**dashboard_ids** | Option<[**Vec<i64>**](i64.md)> | List of dashboard id’s to search for This is deprecated: users should use the `dashboardUIDs` query parameter instead |  |
**dashboard_uids** | Option<[**Vec<String>**](String.md)> | List of dashboard uid’s to search for |  |
**folder_ids** | Option<[**Vec<i64>**](i64.md)> | List of folder id’s to search in for dashboards If it's `0` then it will query for the top level folders This is deprecated: users should use the `folderUIDs` query parameter instead |  |
**folder_uids** | Option<[**Vec<String>**](String.md)> | List of folder UID’s to search in for dashboards If it's an empty string then it will query for the top level folders |  |
**starred** | Option<**bool**> | Flag indicating if only starred Dashboards should be returned |  |
**limit** | Option<**i64**> | Limit the number of returned results (max 5000) |  |
**page** | Option<**i64**> | Use this parameter to access hits beyond limit. Numbering starts at 1. limit param acts as page size. Only available in Grafana v6.2+. |  |
**permission** | Option<**String**> | Set to `Edit` to return dashboards/folders that the user can edit |  |[default to View]
**sort** | Option<**String**> | Sort method; for listing all the possible sort methods use the search sorting endpoint. |  |[default to alpha-asc]

### Return type

[**Vec<models::Hit>**](Hit.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

