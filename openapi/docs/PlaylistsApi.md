# \PlaylistsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_playlist**](PlaylistsApi.md#create_playlist) | **POST** /playlists | Create playlist.
[**delete_playlist**](PlaylistsApi.md#delete_playlist) | **DELETE** /playlists/{uid} | Delete playlist.
[**get_playlist**](PlaylistsApi.md#get_playlist) | **GET** /playlists/{uid} | Get playlist.
[**get_playlist_items**](PlaylistsApi.md#get_playlist_items) | **GET** /playlists/{uid}/items | Get playlist items.
[**search_playlists**](PlaylistsApi.md#search_playlists) | **GET** /playlists | Get playlists.
[**update_playlist**](PlaylistsApi.md#update_playlist) | **PUT** /playlists/{uid} | Update playlist.



## create_playlist

> models::Playlist create_playlist(create_playlist_command)
Create playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_playlist_command** | [**CreatePlaylistCommand**](CreatePlaylistCommand.md) |  | [required] |

### Return type

[**models::Playlist**](Playlist.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_playlist

> models::SuccessResponseBody delete_playlist(uid)
Delete playlist.

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


## get_playlist

> models::PlaylistDto get_playlist(uid)
Get playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**models::PlaylistDto**](PlaylistDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_items

> Vec<models::PlaylistItemDto> get_playlist_items(uid)
Get playlist items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |

### Return type

[**Vec<models::PlaylistItemDto>**](PlaylistItemDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_playlists

> Vec<models::Playlist> search_playlists(query, limit)
Get playlists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |
**limit** | Option<**i64**> | in:limit |  |

### Return type

[**Vec<models::Playlist>**](Playlist.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_playlist

> models::PlaylistDto update_playlist(uid, update_playlist_command)
Update playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**update_playlist_command** | [**UpdatePlaylistCommand**](UpdatePlaylistCommand.md) |  | [required] |

### Return type

[**models::PlaylistDto**](PlaylistDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

