# \ShieldInformationBarriersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_shield_information_barriers**](ShieldInformationBarriersApi.md#get_shield_information_barriers) | **GET** /shield_information_barriers | List shield information barriers
[**get_shield_information_barriers_id**](ShieldInformationBarriersApi.md#get_shield_information_barriers_id) | **GET** /shield_information_barriers/{shield_information_barrier_id} | Get shield information barrier with specified ID
[**post_shield_information_barriers**](ShieldInformationBarriersApi.md#post_shield_information_barriers) | **POST** /shield_information_barriers | Create shield information barrier
[**post_shield_information_barriers_change_status**](ShieldInformationBarriersApi.md#post_shield_information_barriers_change_status) | **POST** /shield_information_barriers/change_status | Add changed status of shield information barrier with specified ID



## get_shield_information_barriers

> models::ShieldInformationBarriers get_shield_information_barriers(marker, limit)
List shield information barriers

Retrieves a list of shield information barrier objects for the enterprise of JWT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**models::ShieldInformationBarriers**](ShieldInformationBarriers.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barriers_id

> models::ShieldInformationBarrier get_shield_information_barriers_id(shield_information_barrier_id)
Get shield information barrier with specified ID

Get shield information barrier based on provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_id** | **String** | The ID of the shield information barrier. | [required] |

### Return type

[**models::ShieldInformationBarrier**](ShieldInformationBarrier.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_shield_information_barriers

> models::ShieldInformationBarrier post_shield_information_barriers(post_shield_information_barriers_request)
Create shield information barrier

Creates a shield information barrier to separate individuals/groups within the same firm and prevents confidential information passing between them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_shield_information_barriers_request** | Option<[**PostShieldInformationBarriersRequest**](PostShieldInformationBarriersRequest.md)> |  |  |

### Return type

[**models::ShieldInformationBarrier**](ShieldInformationBarrier.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_shield_information_barriers_change_status

> models::ShieldInformationBarrier post_shield_information_barriers_change_status(post_shield_information_barriers_change_status_request)
Add changed status of shield information barrier with specified ID

Change status of shield information barrier with the specified ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_shield_information_barriers_change_status_request** | Option<[**PostShieldInformationBarriersChangeStatusRequest**](PostShieldInformationBarriersChangeStatusRequest.md)> |  |  |

### Return type

[**models::ShieldInformationBarrier**](ShieldInformationBarrier.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

