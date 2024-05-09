# \MigrationsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cloud_migration_token**](MigrationsApi.md#create_cloud_migration_token) | **POST** /cloudmigration/token | Create gcom access token.
[**create_migration**](MigrationsApi.md#create_migration) | **POST** /cloudmigration/migration | Create a migration.
[**delete_cloud_migration**](MigrationsApi.md#delete_cloud_migration) | **DELETE** /cloudmigration/migration/{id} | Delete a migration.
[**get_cloud_migration**](MigrationsApi.md#get_cloud_migration) | **GET** /cloudmigration/migration/{id} | Get a cloud migration.
[**get_cloud_migration_run**](MigrationsApi.md#get_cloud_migration_run) | **GET** /cloudmigration/migration/{id}/run/{runID} | Get the result of a single migration run.
[**get_cloud_migration_run_list**](MigrationsApi.md#get_cloud_migration_run_list) | **GET** /cloudmigration/migration/{id}/run | Get a list of migration runs for a migration.
[**get_migration_list**](MigrationsApi.md#get_migration_list) | **GET** /cloudmigration/migration | Get a list of all cloud migrations.
[**run_cloud_migration**](MigrationsApi.md#run_cloud_migration) | **POST** /cloudmigration/migration/{id}/run | Trigger the run of a migration to the Grafana Cloud.



## create_cloud_migration_token

> models::CreateAccessTokenResponseDto create_cloud_migration_token()
Create gcom access token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CreateAccessTokenResponseDto**](CreateAccessTokenResponseDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_migration

> models::CloudMigrationResponse create_migration(cloud_migration_request)
Create a migration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_migration_request** | [**CloudMigrationRequest**](CloudMigrationRequest.md) |  | [required] |

### Return type

[**models::CloudMigrationResponse**](CloudMigrationResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cloud_migration

> delete_cloud_migration(id)
Delete a migration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of an migration | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_migration

> models::CloudMigrationResponse get_cloud_migration(id)
Get a cloud migration.

It returns migrations that has been created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of an migration | [required] |

### Return type

[**models::CloudMigrationResponse**](CloudMigrationResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_migration_run

> models::MigrateDataResponseDto get_cloud_migration_run(id, run_id)
Get the result of a single migration run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of an migration | [required] |
**run_id** | **i64** | Run ID of a migration run | [required] |

### Return type

[**models::MigrateDataResponseDto**](MigrateDataResponseDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_migration_run_list

> models::CloudMigrationRunList get_cloud_migration_run_list(id)
Get a list of migration runs for a migration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of an migration | [required] |

### Return type

[**models::CloudMigrationRunList**](CloudMigrationRunList.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_migration_list

> models::CloudMigrationListResponse get_migration_list()
Get a list of all cloud migrations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CloudMigrationListResponse**](CloudMigrationListResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_cloud_migration

> models::MigrateDataResponseDto run_cloud_migration(id)
Trigger the run of a migration to the Grafana Cloud.

It returns migrations that has been created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of an migration | [required] |

### Return type

[**models::MigrateDataResponseDto**](MigrateDataResponseDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

