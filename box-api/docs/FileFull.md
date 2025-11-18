# FileFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier that represent a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | 
**etag** | Option<**String**> | The HTTP `etag` of this file. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the file if (no) changes have happened. | [optional]
**r#type** | **String** | The value will always be `file`. | 
**sequence_id** | Option<**String**> | A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource. | [optional]
**name** | Option<**String**> | The name of the file. | [optional]
**sha1** | Option<**String**> | The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file. | [optional]
**file_version** | Option<[**models::FileVersionMini**](FileVersion--Mini.md)> | The information about the current version of the file. | [optional]
**description** | Option<**String**> | The optional description of this file. If the description exceeds 255 characters, the first 255 characters are set as a file description and the rest of it is ignored. | [optional]
**size** | Option<**i32**> | The file size in bytes. Be careful parsing this integer as it can get very large and cause an integer overflow. | [optional]
**path_collection** | Option<[**models::FileAllOfPathCollection**](File_allOf_path_collection.md)> |  | [optional]
**created_at** | Option<**String**> | The date and time when the file was created on Box. | [optional]
**modified_at** | Option<**String**> | The date and time when the file was last updated on Box. | [optional]
**trashed_at** | Option<**String**> | The time at which this file was put in the trash. | [optional]
**purged_at** | Option<**String**> | The time at which this file is expected to be purged from the trash. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this file was originally created, which might be before it was uploaded to Box. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this file was last updated, which might be before it was uploaded to Box. | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created this file. | [optional]
**modified_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who last modified this file. | [optional]
**owned_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who owns this file. | [optional]
**shared_link** | Option<[**models::FileAllOfSharedLink**](File_allOf_shared_link.md)> |  | [optional]
**parent** | Option<[**models::FolderMini**](Folder--Mini.md)> | The folder that this file is located within. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted. | [optional]
**version_number** | Option<**String**> | The version number of this file. | [optional]
**comment_count** | Option<**i32**> | The number of comments on this file. | [optional]
**permissions** | Option<[**models::FileFullAllOfPermissions**](File__Full_allOf_permissions.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | The tags for this item. These tags are shown in the Box web app and mobile apps next to an item.  To add or remove a tag, retrieve the item's current tags, modify them, and then update this field.  There is a limit of 100 tags per item, and 10,000 unique tags per enterprise. | [optional]
**lock** | Option<[**models::FileFullAllOfLock**](File__Full_allOf_lock.md)> |  | [optional]
**extension** | Option<**String**> | Indicates the (optional) file extension for this file. By default, this is set to an empty string. | [optional]
**is_package** | Option<**bool**> | Indicates if the file is a package. Packages are commonly used by Mac Applications and can include iWork files. | [optional]
**expiring_embed_link** | Option<[**models::FileFullAllOfExpiringEmbedLink**](File__Full_allOf_expiring_embed_link.md)> |  | [optional]
**watermark_info** | Option<[**models::FileFullAllOfWatermarkInfo**](File__Full_allOf_watermark_info.md)> |  | [optional]
**is_accessible_via_shared_link** | Option<**bool**> | Specifies if the file can be accessed via the direct shared link or a shared link to a parent folder. | [optional]
**allowed_invitee_roles** | Option<**Vec<String>**> | A list of the types of roles that user can be invited at when sharing this file. | [optional]
**is_externally_owned** | Option<**bool**> | Specifies if this file is owned by a user outside of the authenticated enterprise. | [optional]
**has_collaborations** | Option<**bool**> | Specifies if this file has any other collaborators. | [optional]
**metadata** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, models::MetadataFull>>**](std::collections::HashMap.md)> | An object containing the metadata instances that have been attached to this file.  Each metadata instance is uniquely identified by its `scope` and `templateKey`. There can only be one instance of any metadata template attached to each file. Each metadata instance is nested within an object with the `templateKey` as the key, which again itself is nested in an object with the `scope` as the key. | [optional]
**expires_at** | Option<**String**> | When the file will automatically be deleted. | [optional]
**representations** | Option<[**models::FileFullAllOfRepresentations**](File__Full_allOf_representations.md)> |  | [optional]
**classification** | Option<[**models::FileFullAllOfClassification**](File__Full_allOf_classification.md)> |  | [optional]
**uploader_display_name** | Option<**String**> | The display name of the user that uploaded the file. In most cases this is the name of the user logged in at the time of the upload.  If the file was uploaded using a File Request form that requires the user to provide an email address, this field is populated with that email address. If an email address was not required in the File Request form, this field is set to return a value of `File Request`.  In all other anonymous cases where no email was provided this field will default to a value of `Someone`. | [optional]
**disposition_at** | Option<**String**> | The retention expiration timestamp for the given file. | [optional]
**shared_link_permission_options** | Option<**Vec<String>**> | A list of the types of roles that user can be invited at when sharing this file. | [optional]
**is_associated_with_app_item** | Option<**bool**> | This field will return true if the file or any ancestor of the file is associated with at least one app item. Note that this will return true even if the context user does not have access to the app item(s) associated with the file. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


