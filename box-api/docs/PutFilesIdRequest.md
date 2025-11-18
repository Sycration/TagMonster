# PutFilesIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | An optional different name for the file. This can be used to rename the file.  File names must be unique within their parent folder. The name check is case-insensitive, so a file  named `New File` cannot be created in a parent folder that already contains a folder named `new file`. | [optional]
**description** | Option<**String**> | The description for a file. This can be seen in the right-hand sidebar panel when viewing a file in the Box web app. Additionally, this index is used in the search index of the file, allowing users to find the file by the content in the description. | [optional]
**parent** | Option<[**models::PutFilesIdRequestParent**](put_files_id_request_parent.md)> |  | [optional]
**shared_link** | Option<[**models::PutFilesIdRequestSharedLink**](put_files_id_request_shared_link.md)> |  | [optional]
**lock** | Option<[**models::PutFilesIdRequestLock**](put_files_id_request_lock.md)> |  | [optional]
**disposition_at** | Option<**String**> | The retention expiration timestamp for the given file. This date cannot be shortened once set on a file. | [optional]
**permissions** | Option<[**models::PutFilesIdRequestPermissions**](put_files_id_request_permissions.md)> |  | [optional]
**collections** | Option<[**Vec<models::Reference>**](Reference.md)> | An array of collections to make this file a member of. Currently we only support the `favorites` collection.  To get the ID for a collection, use the [List all collections][1] endpoint.  Passing an empty array `[]` or `null` will remove the file from all collections.  [1]: e://get-collections | [optional]
**tags** | Option<**Vec<String>**> | The tags for this item. These tags are shown in the Box web app and mobile apps next to an item.  To add or remove a tag, retrieve the item's current tags, modify them, and then update this field.  There is a limit of 100 tags per item, and 10,000 unique tags per enterprise. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


