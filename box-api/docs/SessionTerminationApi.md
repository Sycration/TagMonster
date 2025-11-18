# \SessionTerminationApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_groups_terminate_sessions**](SessionTerminationApi.md#post_groups_terminate_sessions) | **POST** /groups/terminate_sessions | Create jobs to terminate user group session
[**post_users_terminate_sessions**](SessionTerminationApi.md#post_users_terminate_sessions) | **POST** /users/terminate_sessions | Create jobs to terminate users session



## post_groups_terminate_sessions

> models::SessionTerminationMessage post_groups_terminate_sessions(post_groups_terminate_sessions_request)
Create jobs to terminate user group session

Validates the roles and permissions of the group, and creates asynchronous jobs to terminate the group's sessions. Returns the status for the POST request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_groups_terminate_sessions_request** | Option<[**PostGroupsTerminateSessionsRequest**](PostGroupsTerminateSessionsRequest.md)> |  |  |

### Return type

[**models::SessionTerminationMessage**](SessionTerminationMessage.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_terminate_sessions

> models::SessionTerminationMessage post_users_terminate_sessions(post_users_terminate_sessions_request)
Create jobs to terminate users session

Validates the roles and permissions of the user, and creates asynchronous jobs to terminate the user's sessions. Returns the status for the POST request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_users_terminate_sessions_request** | Option<[**PostUsersTerminateSessionsRequest**](PostUsersTerminateSessionsRequest.md)> |  |  |

### Return type

[**models::SessionTerminationMessage**](SessionTerminationMessage.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

