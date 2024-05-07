# \QueryHistoryApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_query**](QueryHistoryApi.md#create_query) | **POST** /query-history | Add query to query history.
[**delete_query**](QueryHistoryApi.md#delete_query) | **DELETE** /query-history/{query_history_uid} | Delete query in query history.
[**patch_query_comment**](QueryHistoryApi.md#patch_query_comment) | **PATCH** /query-history/{query_history_uid} | Update comment for query in query history.
[**search_queries**](QueryHistoryApi.md#search_queries) | **GET** /query-history | Query history search.
[**star_query**](QueryHistoryApi.md#star_query) | **POST** /query-history/star/{query_history_uid} | Add star to query in query history.
[**unstar_query**](QueryHistoryApi.md#unstar_query) | **DELETE** /query-history/star/{query_history_uid} | Remove star to query in query history.



## create_query

> models::QueryHistoryResponse create_query(create_query_in_query_history_command)
Add query to query history.

Adds new query to query history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_query_in_query_history_command** | [**CreateQueryInQueryHistoryCommand**](CreateQueryInQueryHistoryCommand.md) |  | [required] |

### Return type

[**models::QueryHistoryResponse**](QueryHistoryResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_query

> models::QueryHistoryDeleteQueryResponse delete_query(query_history_uid)
Delete query in query history.

Deletes an existing query in query history as specified by the UID. This operation cannot be reverted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_history_uid** | **String** |  | [required] |

### Return type

[**models::QueryHistoryDeleteQueryResponse**](QueryHistoryDeleteQueryResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_query_comment

> models::QueryHistoryResponse patch_query_comment(query_history_uid, patch_query_comment_in_query_history_command)
Update comment for query in query history.

Updates comment for query in query history as specified by the UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_history_uid** | **String** |  | [required] |
**patch_query_comment_in_query_history_command** | [**PatchQueryCommentInQueryHistoryCommand**](PatchQueryCommentInQueryHistoryCommand.md) |  | [required] |

### Return type

[**models::QueryHistoryResponse**](QueryHistoryResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_queries

> models::QueryHistorySearchResponse search_queries(datasource_uid, search_string, only_starred, sort, page, limit, from, to)
Query history search.

Returns a list of queries in the query history that matches the search criteria. Query history search supports pagination. Use the `limit` parameter to control the maximum number of queries returned; the default limit is 100. You can also use the `page` query parameter to fetch queries from any page other than the first one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource_uid** | Option<[**Vec<String>**](String.md)> | List of data source UIDs to search for |  |
**search_string** | Option<**String**> | Text inside query or comments that is searched for |  |
**only_starred** | Option<**bool**> | Flag indicating if only starred queries should be returned |  |
**sort** | Option<**String**> | Sort method |  |[default to time-desc]
**page** | Option<**i64**> | Use this parameter to access hits beyond limit. Numbering starts at 1. limit param acts as page size. |  |
**limit** | Option<**i64**> | Limit the number of returned results |  |
**from** | Option<**i64**> | From range for the query history search |  |
**to** | Option<**i64**> | To range for the query history search |  |

### Return type

[**models::QueryHistorySearchResponse**](QueryHistorySearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## star_query

> models::QueryHistoryResponse star_query(query_history_uid)
Add star to query in query history.

Adds star to query in query history as specified by the UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_history_uid** | **String** |  | [required] |

### Return type

[**models::QueryHistoryResponse**](QueryHistoryResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unstar_query

> models::QueryHistoryResponse unstar_query(query_history_uid)
Remove star to query in query history.

Removes star from query in query history as specified by the UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_history_uid** | **String** |  | [required] |

### Return type

[**models::QueryHistoryResponse**](QueryHistoryResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

