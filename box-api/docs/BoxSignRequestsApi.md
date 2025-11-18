# \BoxSignRequestsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sign_requests**](BoxSignRequestsApi.md#get_sign_requests) | **GET** /sign_requests | List Box Sign requests
[**get_sign_requests_id**](BoxSignRequestsApi.md#get_sign_requests_id) | **GET** /sign_requests/{sign_request_id} | Get Box Sign request by ID
[**post_sign_requests**](BoxSignRequestsApi.md#post_sign_requests) | **POST** /sign_requests | Create Box Sign request
[**post_sign_requests_id_cancel**](BoxSignRequestsApi.md#post_sign_requests_id_cancel) | **POST** /sign_requests/{sign_request_id}/cancel | Cancel Box Sign request
[**post_sign_requests_id_resend**](BoxSignRequestsApi.md#post_sign_requests_id_resend) | **POST** /sign_requests/{sign_request_id}/resend | Resend Box Sign request



## get_sign_requests

> models::SignRequests get_sign_requests(marker, limit, senders, shared_requests)
List Box Sign requests

Gets signature requests created by a user. If the `sign_files` and/or `parent_folder` are deleted, the signature request will not return in the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**senders** | Option<[**Vec<String>**](String.md)> | A list of sender emails to filter the signature requests by sender. If provided, `shared_requests` must be set to `true`. |  |
**shared_requests** | Option<**bool**> | If set to `true`, only includes requests that user is not an owner, but user is a collaborator. Collaborator access is determined by the user access level of the sign files of the request. Default is `false`. Must be set to `true` if `senders` are provided. |  |[default to false]

### Return type

[**models::SignRequests**](SignRequests.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sign_requests_id

> models::SignRequest get_sign_requests_id(sign_request_id)
Get Box Sign request by ID

Gets a sign request by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_id** | **String** | The ID of the signature request. | [required] |

### Return type

[**models::SignRequest**](SignRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sign_requests

> models::SignRequest post_sign_requests(sign_request_create_request)
Create Box Sign request

Creates a signature request. This involves preparing a document for signing and sending the signature request to signers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_create_request** | Option<[**SignRequestCreateRequest**](SignRequestCreateRequest.md)> |  |  |

### Return type

[**models::SignRequest**](SignRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sign_requests_id_cancel

> models::SignRequest post_sign_requests_id_cancel(sign_request_id)
Cancel Box Sign request

Cancels a sign request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_id** | **String** | The ID of the signature request. | [required] |

### Return type

[**models::SignRequest**](SignRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sign_requests_id_resend

> post_sign_requests_id_resend(sign_request_id)
Resend Box Sign request

Resends a signature request email to all outstanding signers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_id** | **String** | The ID of the signature request. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

