# \DatasourcesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_data_source**](DatasourcesApi.md#add_data_source) | **POST** /datasources | Create a data source.
[**call_datasource_resource_by_id**](DatasourcesApi.md#call_datasource_resource_by_id) | **GET** /datasources/{id}/resources/{datasource_proxy_route} | Fetch data source resources by Id.
[**call_datasource_resource_with_uid**](DatasourcesApi.md#call_datasource_resource_with_uid) | **GET** /datasources/uid/{uid}/resources/{datasource_proxy_route} | Fetch data source resources.
[**check_datasource_health_by_id**](DatasourcesApi.md#check_datasource_health_by_id) | **GET** /datasources/{id}/health | Sends a health check request to the plugin datasource identified by the ID.
[**check_datasource_health_with_uid**](DatasourcesApi.md#check_datasource_health_with_uid) | **GET** /datasources/uid/{uid}/health | Sends a health check request to the plugin datasource identified by the UID.
[**datasource_proxy_delet_ecalls**](DatasourcesApi.md#datasource_proxy_delet_ecalls) | **DELETE** /datasources/proxy/{id}/{datasource_proxy_route} | Data source proxy DELETE calls.
[**datasource_proxy_deleteby_ui_dcalls**](DatasourcesApi.md#datasource_proxy_deleteby_ui_dcalls) | **DELETE** /datasources/proxy/uid/{uid}/{datasource_proxy_route} | Data source proxy DELETE calls.
[**datasource_proxy_ge_tcalls**](DatasourcesApi.md#datasource_proxy_ge_tcalls) | **GET** /datasources/proxy/{id}/{datasource_proxy_route} | Data source proxy GET calls.
[**datasource_proxy_getby_ui_dcalls**](DatasourcesApi.md#datasource_proxy_getby_ui_dcalls) | **GET** /datasources/proxy/uid/{uid}/{datasource_proxy_route} | Data source proxy GET calls.
[**datasource_proxy_pos_tcalls**](DatasourcesApi.md#datasource_proxy_pos_tcalls) | **POST** /datasources/proxy/{id}/{datasource_proxy_route} | Data source proxy POST calls.
[**datasource_proxy_postby_ui_dcalls**](DatasourcesApi.md#datasource_proxy_postby_ui_dcalls) | **POST** /datasources/proxy/uid/{uid}/{datasource_proxy_route} | Data source proxy POST calls.
[**delete_data_source_by_id**](DatasourcesApi.md#delete_data_source_by_id) | **DELETE** /datasources/{id} | Delete an existing data source by id.
[**delete_data_source_by_name**](DatasourcesApi.md#delete_data_source_by_name) | **DELETE** /datasources/name/{name} | Delete an existing data source by name.
[**delete_data_source_by_uid**](DatasourcesApi.md#delete_data_source_by_uid) | **DELETE** /datasources/uid/{uid} | Delete an existing data source by UID.
[**get_data_source_by_id**](DatasourcesApi.md#get_data_source_by_id) | **GET** /datasources/{id} | Get a single data source by Id.
[**get_data_source_by_name**](DatasourcesApi.md#get_data_source_by_name) | **GET** /datasources/name/{name} | Get a single data source by Name.
[**get_data_source_by_uid**](DatasourcesApi.md#get_data_source_by_uid) | **GET** /datasources/uid/{uid} | Get a single data source by UID.
[**get_data_source_id_by_name**](DatasourcesApi.md#get_data_source_id_by_name) | **GET** /datasources/id/{name} | Get data source Id by Name.
[**get_data_sources**](DatasourcesApi.md#get_data_sources) | **GET** /datasources | Get all data sources.
[**update_data_source_by_id**](DatasourcesApi.md#update_data_source_by_id) | **PUT** /datasources/{id} | Update an existing data source by its sequential ID.
[**update_data_source_by_uid**](DatasourcesApi.md#update_data_source_by_uid) | **PUT** /datasources/uid/{uid} | Update an existing data source.



## add_data_source

> models::AddDataSource200Response add_data_source(add_data_source_command)
Create a data source.

By defining `password` and `basicAuthPassword` under secureJsonData property Grafana encrypts them securely as an encrypted blob in the database. The response then lists the encrypted fields under secureJsonFields.  If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:create`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_data_source_command** | [**AddDataSourceCommand**](AddDataSourceCommand.md) |  | [required] |

### Return type

[**models::AddDataSource200Response**](addDataSource_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_datasource_resource_by_id

> models::SuccessResponseBody call_datasource_resource_by_id(datasource_proxy_route, id)
Fetch data source resources by Id.

Please refer to [updated API](#/datasources/callDatasourceResourceWithUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource_proxy_route** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_datasource_resource_with_uid

> models::SuccessResponseBody call_datasource_resource_with_uid(datasource_proxy_route, uid)
Fetch data source resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource_proxy_route** | **String** |  | [required] |
**uid** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_datasource_health_by_id

> models::SuccessResponseBody check_datasource_health_by_id(id)
Sends a health check request to the plugin datasource identified by the ID.

Please refer to [updated API](#/datasources/checkDatasourceHealthWithUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_datasource_health_with_uid

> models::SuccessResponseBody check_datasource_health_with_uid(uid)
Sends a health check request to the plugin datasource identified by the UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasource_proxy_delet_ecalls

> datasource_proxy_delet_ecalls(id, datasource_proxy_route)
Data source proxy DELETE calls.

Proxies all calls to the actual data source.  Please refer to [updated API](#/datasources/datasourceProxyDELETEByUIDcalls) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**datasource_proxy_route** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasource_proxy_deleteby_ui_dcalls

> datasource_proxy_deleteby_ui_dcalls(uid, datasource_proxy_route)
Data source proxy DELETE calls.

Proxies all calls to the actual data source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**datasource_proxy_route** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasource_proxy_ge_tcalls

> datasource_proxy_ge_tcalls(datasource_proxy_route, id)
Data source proxy GET calls.

Proxies all calls to the actual data source.  Please refer to [updated API](#/datasources/datasourceProxyGETByUIDcalls) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource_proxy_route** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasource_proxy_getby_ui_dcalls

> datasource_proxy_getby_ui_dcalls(datasource_proxy_route, uid)
Data source proxy GET calls.

Proxies all calls to the actual data source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource_proxy_route** | **String** |  | [required] |
**uid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasource_proxy_pos_tcalls

> datasource_proxy_pos_tcalls(datasource_proxy_route, id, body)
Data source proxy POST calls.

Proxies all calls to the actual data source. The data source should support POST methods for the specific path and role as defined  Please refer to [updated API](#/datasources/datasourceProxyPOSTByUIDcalls) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource_proxy_route** | **String** |  | [required] |
**id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## datasource_proxy_postby_ui_dcalls

> datasource_proxy_postby_ui_dcalls(datasource_proxy_route, uid, body)
Data source proxy POST calls.

Proxies all calls to the actual data source. The data source should support POST methods for the specific path and role as defined

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource_proxy_route** | **String** |  | [required] |
**uid** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_source_by_id

> models::SuccessResponseBody delete_data_source_by_id(id)
Delete an existing data source by id.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:delete` and scopes: `datasources:*`, `datasources:id:*` and `datasources:id:1` (single data source).  Please refer to [updated API](#/datasources/deleteDataSourceByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_source_by_name

> models::DeleteDataSourceByName200Response delete_data_source_by_name(name)
Delete an existing data source by name.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:delete` and scopes: `datasources:*`, `datasources:name:*` and `datasources:name:test_datasource` (single data source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DeleteDataSourceByName200Response**](deleteDataSourceByName_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_source_by_uid

> models::SuccessResponseBody delete_data_source_by_uid(uid)
Delete an existing data source by UID.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:delete` and scopes: `datasources:*`, `datasources:uid:*` and `datasources:uid:kLtEtcRGk` (single data source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_source_by_id

> models::DataSource get_data_source_by_id(id)
Get a single data source by Id.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:read` and scopes: `datasources:*`, `datasources:id:*` and `datasources:id:1` (single data source).  Please refer to [updated API](#/datasources/getDataSourceByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DataSource**](DataSource.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_source_by_name

> models::DataSource get_data_source_by_name(name)
Get a single data source by Name.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:read` and scopes: `datasources:*`, `datasources:name:*` and `datasources:name:test_datasource` (single data source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DataSource**](DataSource.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_source_by_uid

> models::DataSource get_data_source_by_uid(uid)
Get a single data source by UID.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:read` and scopes: `datasources:*`, `datasources:uid:*` and `datasources:uid:kLtEtcRGk` (single data source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**models::DataSource**](DataSource.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_source_id_by_name

> models::GetDataSourceIdByName200Response get_data_source_id_by_name(name)
Get data source Id by Name.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:read` and scopes: `datasources:*`, `datasources:name:*` and `datasources:name:test_datasource` (single data source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::GetDataSourceIdByName200Response**](getDataSourceIdByName_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_sources

> Vec<models::DataSourceListItemDto> get_data_sources()
Get all data sources.

If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:read` and scope: `datasources:*`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DataSourceListItemDto>**](DataSourceListItemDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_data_source_by_id

> models::AddDataSource200Response update_data_source_by_id(id, update_data_source_command)
Update an existing data source by its sequential ID.

Similar to creating a data source, `password` and `basicAuthPassword` should be defined under secureJsonData in order to be stored securely as an encrypted blob in the database. Then, the encrypted fields are listed under secureJsonFields section in the response.  If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:write` and scopes: `datasources:*`, `datasources:id:*` and `datasources:id:1` (single data source).  Please refer to [updated API](#/datasources/updateDataSourceByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_data_source_command** | [**UpdateDataSourceCommand**](UpdateDataSourceCommand.md) |  | [required] |

### Return type

[**models::AddDataSource200Response**](addDataSource_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_data_source_by_uid

> models::AddDataSource200Response update_data_source_by_uid(uid, update_data_source_command)
Update an existing data source.

Similar to creating a data source, `password` and `basicAuthPassword` should be defined under secureJsonData in order to be stored securely as an encrypted blob in the database. Then, the encrypted fields are listed under secureJsonFields section in the response.  If you are running Grafana Enterprise and have Fine-grained access control enabled you need to have a permission with action: `datasources:write` and scopes: `datasources:*`, `datasources:uid:*` and `datasources:uid:1` (single data source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**update_data_source_command** | [**UpdateDataSourceCommand**](UpdateDataSourceCommand.md) |  | [required] |

### Return type

[**models::AddDataSource200Response**](addDataSource_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

