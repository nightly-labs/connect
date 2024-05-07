# \LibraryElementsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_library_element**](LibraryElementsApi.md#create_library_element) | **POST** /library-elements | Create library element.
[**delete_library_element_by_uid**](LibraryElementsApi.md#delete_library_element_by_uid) | **DELETE** /library-elements/{library_element_uid} | Delete library element.
[**get_library_element_by_name**](LibraryElementsApi.md#get_library_element_by_name) | **GET** /library-elements/name/{library_element_name} | Get library element by name.
[**get_library_element_by_uid**](LibraryElementsApi.md#get_library_element_by_uid) | **GET** /library-elements/{library_element_uid} | Get library element by UID.
[**get_library_element_connections**](LibraryElementsApi.md#get_library_element_connections) | **GET** /library-elements/{library_element_uid}/connections/ | Get library element connections.
[**get_library_elements**](LibraryElementsApi.md#get_library_elements) | **GET** /library-elements | Get all library elements.
[**update_library_element**](LibraryElementsApi.md#update_library_element) | **PATCH** /library-elements/{library_element_uid} | Update library element.



## create_library_element

> models::LibraryElementResponse create_library_element(create_library_element_command)
Create library element.

Creates a new library element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_library_element_command** | [**CreateLibraryElementCommand**](CreateLibraryElementCommand.md) |  | [required] |

### Return type

[**models::LibraryElementResponse**](LibraryElementResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_library_element_by_uid

> models::SuccessResponseBody delete_library_element_by_uid(library_element_uid)
Delete library element.

Deletes an existing library element as specified by the UID. This operation cannot be reverted. You cannot delete a library element that is connected. This operation cannot be reverted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_element_uid** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_element_by_name

> models::LibraryElementArrayResponse get_library_element_by_name(library_element_name)
Get library element by name.

Returns a library element with the given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_element_name** | **String** |  | [required] |

### Return type

[**models::LibraryElementArrayResponse**](LibraryElementArrayResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_element_by_uid

> models::LibraryElementResponse get_library_element_by_uid(library_element_uid)
Get library element by UID.

Returns a library element with the given UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_element_uid** | **String** |  | [required] |

### Return type

[**models::LibraryElementResponse**](LibraryElementResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_element_connections

> models::LibraryElementConnectionsResponse get_library_element_connections(library_element_uid)
Get library element connections.

Returns a list of connections for a library element based on the UID specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_element_uid** | **String** |  | [required] |

### Return type

[**models::LibraryElementConnectionsResponse**](LibraryElementConnectionsResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_elements

> models::LibraryElementSearchResponse get_library_elements(search_string, kind, sort_direction, type_filter, exclude_uid, folder_filter, per_page, page)
Get all library elements.

Returns a list of all library elements the authenticated user has permission to view. Use the `perPage` query parameter to control the maximum number of library elements returned; the default limit is `100`. You can also use the `page` query parameter to fetch library elements from any page other than the first one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_string** | Option<**String**> | Part of the name or description searched for. |  |
**kind** | Option<**i64**> | Kind of element to search for. |  |
**sort_direction** | Option<**String**> | Sort order of elements. |  |
**type_filter** | Option<**String**> | A comma separated list of types to filter the elements by |  |
**exclude_uid** | Option<**String**> | Element UID to exclude from search results. |  |
**folder_filter** | Option<**String**> | A comma separated list of folder ID(s) to filter the elements by. |  |
**per_page** | Option<**i64**> | The number of results per page. |  |[default to 100]
**page** | Option<**i64**> | The page for a set of records, given that only perPage records are returned at a time. Numbering starts at 1. |  |[default to 1]

### Return type

[**models::LibraryElementSearchResponse**](LibraryElementSearchResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_library_element

> models::LibraryElementResponse update_library_element(library_element_uid, patch_library_element_command)
Update library element.

Updates an existing library element identified by uid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_element_uid** | **String** |  | [required] |
**patch_library_element_command** | [**PatchLibraryElementCommand**](PatchLibraryElementCommand.md) |  | [required] |

### Return type

[**models::LibraryElementResponse**](LibraryElementResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

