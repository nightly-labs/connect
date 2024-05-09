# \FoldersApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_folder**](FoldersApi.md#create_folder) | **POST** /folders | Create folder.
[**delete_folder**](FoldersApi.md#delete_folder) | **DELETE** /folders/{folder_uid} | Delete folder.
[**get_folder_by_id**](FoldersApi.md#get_folder_by_id) | **GET** /folders/id/{folder_id} | Get folder by id.
[**get_folder_by_uid**](FoldersApi.md#get_folder_by_uid) | **GET** /folders/{folder_uid} | Get folder by uid.
[**get_folder_descendant_counts**](FoldersApi.md#get_folder_descendant_counts) | **GET** /folders/{folder_uid}/counts | Gets the count of each descendant of a folder by kind. The folder is identified by UID.
[**get_folders**](FoldersApi.md#get_folders) | **GET** /folders | Get all folders.
[**move_folder**](FoldersApi.md#move_folder) | **POST** /folders/{folder_uid}/move | Move folder.
[**update_folder**](FoldersApi.md#update_folder) | **PUT** /folders/{folder_uid} | Update folder.



## create_folder

> models::Folder create_folder(create_folder_command)
Create folder.

If nested folders are enabled then it additionally expects the parent folder UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_folder_command** | [**CreateFolderCommand**](CreateFolderCommand.md) |  | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_folder

> models::DeleteFolder200Response delete_folder(folder_uid, force_delete_rules)
Delete folder.

Deletes an existing folder identified by UID along with all dashboards (and their alerts) stored in the folder. This operation cannot be reverted. If nested folders are enabled then it also deletes all the subfolders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**force_delete_rules** | Option<**bool**> | If `true` any Grafana 8 Alerts under this folder will be deleted. Set to `false` so that the request will fail if the folder contains any Grafana 8 Alerts. |  |[default to false]

### Return type

[**models::DeleteFolder200Response**](deleteFolder_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_by_id

> models::Folder get_folder_by_id(folder_id)
Get folder by id.

Returns the folder identified by id. This is deprecated. Please refer to [updated API](#/folders/getFolderByUID) instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **i64** |  | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_by_uid

> models::Folder get_folder_by_uid(folder_uid)
Get folder by uid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_descendant_counts

> std::collections::HashMap<String, i64> get_folder_descendant_counts(folder_uid)
Gets the count of each descendant of a folder by kind. The folder is identified by UID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |

### Return type

**std::collections::HashMap<String, i64>**

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders

> Vec<models::FolderSearchHit> get_folders(limit, page, parent_uid, permission)
Get all folders.

It returns all folders that the authenticated user has permission to view. If nested folders are enabled, it expects an additional query parameter with the parent folder UID and returns the immediate subfolders that the authenticated user has permission to view. If the parameter is not supplied then it returns immediate subfolders under the root that the authenticated user has permission to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Limit the maximum number of folders to return |  |[default to 1000]
**page** | Option<**i64**> | Page index for starting fetching folders |  |[default to 1]
**parent_uid** | Option<**String**> | The parent folder UID |  |
**permission** | Option<**String**> | Set to `Edit` to return folders that the user can edit |  |[default to View]

### Return type

[**Vec<models::FolderSearchHit>**](FolderSearchHit.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_folder

> models::Folder move_folder(folder_uid, move_folder_command)
Move folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**move_folder_command** | [**MoveFolderCommand**](MoveFolderCommand.md) |  | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_folder

> models::Folder update_folder(folder_uid, update_folder_command)
Update folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_uid** | **String** |  | [required] |
**update_folder_command** | [**UpdateFolderCommand**](UpdateFolderCommand.md) | To change the unique identifier (uid), provide another one. To overwrite an existing folder with newer version, set `overwrite` to `true`. Provide the current version to safelly update the folder: if the provided version differs from the stored one the request will fail, unless `overwrite` is `true`. | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

