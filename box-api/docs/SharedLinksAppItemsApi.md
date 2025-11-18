# \SharedLinksAppItemsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_shared_items_app_items**](SharedLinksAppItemsApi.md#get_shared_items_app_items) | **GET** /shared_items#app_items | Find app item for shared link



## get_shared_items_app_items

> models::AppItem get_shared_items_app_items(boxapi)
Find app item for shared link

Returns the app item represented by a shared link.  The link can originate from the current enterprise or another.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**boxapi** | **String** | A header containing the shared link and optional password for the shared link.  The format for this header is `shared_link=[link]&shared_link_password=[password]`. | [required] |

### Return type

[**models::AppItem**](AppItem.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

