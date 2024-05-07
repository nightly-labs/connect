# \OrgInvitesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_org_invite**](OrgInvitesApi.md#add_org_invite) | **POST** /org/invites | Add invite.
[**get_pending_org_invites**](OrgInvitesApi.md#get_pending_org_invites) | **GET** /org/invites | Get pending invites.
[**revoke_invite**](OrgInvitesApi.md#revoke_invite) | **DELETE** /org/invites/{invitation_code}/revoke | Revoke invite.



## add_org_invite

> models::SuccessResponseBody add_org_invite(add_invite_form)
Add invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_invite_form** | [**AddInviteForm**](AddInviteForm.md) |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_org_invites

> Vec<models::TempUserDto> get_pending_org_invites()
Get pending invites.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TempUserDto>**](TempUserDTO.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_invite

> models::SuccessResponseBody revoke_invite(invitation_code)
Revoke invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_code** | **String** |  | [required] |

### Return type

[**models::SuccessResponseBody**](SuccessResponseBody.md)

### Authorization

[api_key](../README.md#api_key), [basic](../README.md#basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

