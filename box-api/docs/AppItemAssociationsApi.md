# \AppItemAssociationsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_files_id_app_item_associations**](AppItemAssociationsApi.md#get_files_id_app_item_associations) | **GET** /files/{file_id}/app_item_associations | List file app item associations
[**get_folders_id_app_item_associations**](AppItemAssociationsApi.md#get_folders_id_app_item_associations) | **GET** /folders/{folder_id}/app_item_associations | List folder app item associations



## get_files_id_app_item_associations

> models::AppItemAssociations get_files_id_app_item_associations(file_id, limit, marker, application_type)
List file app item associations

**This is a beta feature, which means that its availability might be limited.** Returns all app items the file is associated with. This includes app items associated with ancestors of the file. Assuming the context user has access to the file, the type/ids are revealed even if the context user does not have **View** permission on the app item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**application_type** | Option<**String**> | If given, only return app items for this application type. |  |

### Return type

[**models::AppItemAssociations**](AppItemAssociations.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders_id_app_item_associations

> models::AppItemAssociations get_folders_id_app_item_associations(folder_id, limit, marker, application_type)
List folder app item associations

**This is a beta feature, which means that its availability might be limited.** Returns all app items the folder is associated with. This includes app items associated with ancestors of the folder. Assuming the context user has access to the folder, the type/ids are revealed even if the context user does not have **View** permission on the app item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**application_type** | Option<**String**> | If given, returns only app items for this application type. |  |

### Return type

[**models::AppItemAssociations**](AppItemAssociations.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

