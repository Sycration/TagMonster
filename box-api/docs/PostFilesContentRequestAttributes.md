# PostFilesContentRequestAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the file.  File names must be unique within their parent folder. The name check is case-insensitive, so a file named `New File` cannot be created in a parent folder that already contains a folder named `new file`. | 
**parent** | [**models::PostFilesContentRequestAttributesParent**](post_files_content_request_attributes_parent.md) |  | 
**content_created_at** | Option<**String**> | Defines the time the file was originally created at.  If not set, the upload time will be used. | [optional]
**content_modified_at** | Option<**String**> | Defines the time the file was last modified at.  If not set, the upload time will be used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


