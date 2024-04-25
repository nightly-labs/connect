# \AnnotationsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_annotation_by_id**](AnnotationsApi.md#delete_annotation_by_id) | **DELETE** /annotations/{annotation_id} | Delete Annotation By ID.
[**get_annotation_by_id**](AnnotationsApi.md#get_annotation_by_id) | **GET** /annotations/{annotation_id} | Get Annotation by ID.
[**get_annotation_tags**](AnnotationsApi.md#get_annotation_tags) | **GET** /annotations/tags | Find Annotations Tags.
[**get_annotations**](AnnotationsApi.md#get_annotations) | **GET** /annotations | Find Annotations.
[**mass_delete_annotations**](AnnotationsApi.md#mass_delete_annotations) | **POST** /annotations/mass-delete | Delete multiple annotations.
[**patch_annotation**](AnnotationsApi.md#patch_annotation) | **PATCH** /annotations/{annotation_id} | Patch Annotation.
[**post_annotation**](AnnotationsApi.md#post_annotation) | **POST** /annotations | Create Annotation.
[**post_graphite_annotation**](AnnotationsApi.md#post_graphite_annotation) | **POST** /annotations/graphite | Create Annotation in Graphite format.
[**update_annotation**](AnnotationsApi.md#update_annotation) | **PUT** /annotations/{annotation_id} | Update Annotation.



## delete_annotation_by_id

> models::SuccessResponseBody delete_annotation_by_id(annotation_id)
Delete Annotation By ID.

Deletes the annotation that matches the specified ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_id** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_annotation_by_id

> models::Annotation get_annotation_by_id(annotation_id)
Get Annotation by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_id** | **String** |  | [required] |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_annotation_tags

> models::GetAnnotationTagsResponse get_annotation_tags(tag, limit)
Find Annotations Tags.

Find all the event tags created in the annotations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | Option<**String**> | Tag is a string that you can use to filter tags. |  |
**limit** | Option<**String**> | Max limit for results returned. |  |[default to 100]

### Return type

[**models::GetAnnotationTagsResponse**](GetAnnotationTagsResponse.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_annotations

> Vec<models::Annotation> get_annotations(from, to, user_id, alert_id, dashboard_id, dashboard_uid, panel_id, limit, tags, r#type, match_any)
Find Annotations.

Starting in Grafana v6.4 regions annotations are now returned in one entity that now includes the timeEnd property.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**i64**> | Find annotations created after specific epoch datetime in milliseconds. |  |
**to** | Option<**i64**> | Find annotations created before specific epoch datetime in milliseconds. |  |
**user_id** | Option<**i64**> | Limit response to annotations created by specific user. |  |
**alert_id** | Option<**i64**> | Find annotations for a specified alert. |  |
**dashboard_id** | Option<**i64**> | Find annotations that are scoped to a specific dashboard |  |
**dashboard_uid** | Option<**String**> | Find annotations that are scoped to a specific dashboard |  |
**panel_id** | Option<**i64**> | Find annotations that are scoped to a specific panel |  |
**limit** | Option<**i64**> | Max limit for results returned. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Use this to filter organization annotations. Organization annotations are annotations from an annotation data source that are not connected specifically to a dashboard or panel. You can filter by multiple tags. |  |
**r#type** | Option<**String**> | Return alerts or user created annotations |  |
**match_any** | Option<**bool**> | Match any or all tags |  |

### Return type

[**Vec<models::Annotation>**](Annotation.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mass_delete_annotations

> models::SuccessResponseBody mass_delete_annotations(mass_delete_annotations_cmd)
Delete multiple annotations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mass_delete_annotations_cmd** | [**MassDeleteAnnotationsCmd**](MassDeleteAnnotationsCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_annotation

> models::SuccessResponseBody patch_annotation(annotation_id, patch_annotations_cmd)
Patch Annotation.

Updates one or more properties of an annotation that matches the specified ID. This operation currently supports updating of the `text`, `tags`, `time` and `timeEnd` properties. This is available in Grafana 6.0.0-beta2 and above.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_id** | **String** |  | [required] |
**patch_annotations_cmd** | [**PatchAnnotationsCmd**](PatchAnnotationsCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_annotation

> models::PostAnnotation200Response post_annotation(post_annotations_cmd)
Create Annotation.

Creates an annotation in the Grafana database. The dashboardId and panelId fields are optional. If they are not specified then an organization annotation is created and can be queried in any dashboard that adds the Grafana annotations data source. When creating a region annotation include the timeEnd property. The format for `time` and `timeEnd` should be epoch numbers in millisecond resolution. The response for this HTTP request is slightly different in versions prior to v6.4. In prior versions you would also get an endId if you where creating a region. But in 6.4 regions are represented using a single event with time and timeEnd properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_annotations_cmd** | [**PostAnnotationsCmd**](PostAnnotationsCmd.md) |  | [required] |

### Return type

[**models::PostAnnotation200Response**](postAnnotation_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_graphite_annotation

> models::PostAnnotation200Response post_graphite_annotation(post_graphite_annotations_cmd)
Create Annotation in Graphite format.

Creates an annotation by using Graphite-compatible event format. The `when` and `data` fields are optional. If `when` is not specified then the current time will be used as annotation’s timestamp. The `tags` field can also be in prior to Graphite `0.10.0` format (string with multiple tags being separated by a space).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_graphite_annotations_cmd** | [**PostGraphiteAnnotationsCmd**](PostGraphiteAnnotationsCmd.md) |  | [required] |

### Return type

[**models::PostAnnotation200Response**](postAnnotation_200_response.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_annotation

> models::SuccessResponseBody update_annotation(annotation_id, update_annotations_cmd)
Update Annotation.

Updates all properties of an annotation that matches the specified id. To only update certain property, consider using the Patch Annotation operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_id** | **String** |  | [required] |
**update_annotations_cmd** | [**UpdateAnnotationsCmd**](UpdateAnnotationsCmd.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

