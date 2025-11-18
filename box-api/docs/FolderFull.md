# FolderFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`. | 
**etag** | Option<**String**> | The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened. | [optional]
**r#type** | **String** | The value will always be `folder`. | 
**sequence_id** | Option<**String**> | A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource. | [optional]
**name** | Option<**String**> | The name of the folder. | [optional]
**created_at** | Option<**String**> | The date and time when the folder was created. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**modified_at** | Option<**String**> | The date and time when the folder was last updated. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**description** | Option<**String**> | The optional description of this folder. | [optional]
**size** | Option<**i64**> | The folder size in bytes.  Be careful parsing this integer as its value can get very large. | [optional]
**path_collection** | Option<[**models::FolderAllOfPathCollection**](Folder_allOf_path_collection.md)> |  | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created this folder. | [optional]
**modified_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who last modified this folder. | [optional]
**trashed_at** | Option<**String**> | The time at which this folder was put in the trash. | [optional]
**purged_at** | Option<**String**> | The time at which this folder is expected to be purged from the trash. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this folder was originally created. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this folder was last updated. | [optional]
**owned_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who owns this folder. | [optional]
**shared_link** | Option<[**models::FolderAllOfSharedLink**](Folder_allOf_shared_link.md)> |  | [optional]
**folder_upload_email** | Option<[**models::FolderAllOfFolderUploadEmail**](Folder_allOf_folder_upload_email.md)> |  | [optional]
**parent** | Option<[**models::FolderMini**](Folder--Mini.md)> | The optional folder that this folder is located within.  This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted. | [optional]
**item_collection** | Option<[**models::Items**](Items.md)> | A page of the items that are in the folder.  This field can only be requested when querying a folder's information, not when querying a folder's items. | [optional]
**sync_state** | Option<**String**> | Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive. | [optional]
**has_collaborations** | Option<**bool**> | Specifies if this folder has any other collaborators. | [optional]
**permissions** | Option<[**models::FolderFullAllOfPermissions**](Folder__Full_allOf_permissions.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | The tags for this item. These tags are shown in the Box web app and mobile apps next to an item.  To add or remove a tag, retrieve the item's current tags, modify them, and then update this field.  There is a limit of 100 tags per item, and 10,000 unique tags per enterprise. | [optional]
**can_non_owners_invite** | Option<**bool**> | Specifies if users who are not the owner of the folder can invite new collaborators to the folder. | [optional]
**is_externally_owned** | Option<**bool**> | Specifies if this folder is owned by a user outside of the authenticated enterprise. | [optional]
**metadata** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, models::MetadataFull>>**](std::collections::HashMap.md)> | An object containing the metadata instances that have been attached to this folder.  Each metadata instance is uniquely identified by its `scope` and `templateKey`. There can only be one instance of any metadata template attached to each folder. Each metadata instance is nested within an object with the `templateKey` as the key, which again itself is nested in an object with the `scope` as the key. | [optional]
**is_collaboration_restricted_to_enterprise** | Option<**bool**> | Specifies if new invites to this folder are restricted to users within the enterprise. This does not affect existing collaborations. | [optional]
**allowed_shared_link_access_levels** | Option<**Vec<String>**> | A list of access levels that are available for this folder.  For some folders, like the root folder, this will always be an empty list as sharing is not allowed at that level. | [optional]
**allowed_invitee_roles** | Option<**Vec<String>**> | A list of the types of roles that user can be invited at when sharing this folder. | [optional]
**watermark_info** | Option<[**models::FolderFullAllOfWatermarkInfo**](Folder__Full_allOf_watermark_info.md)> |  | [optional]
**is_accessible_via_shared_link** | Option<**bool**> | Specifies if the folder can be accessed with the direct shared link or a shared link to a parent folder. | [optional]
**can_non_owners_view_collaborators** | Option<**bool**> | Specifies if collaborators who are not owners of this folder are restricted from viewing other collaborations on this folder.  It also restricts non-owners from inviting new collaborators. | [optional]
**classification** | Option<[**models::FolderFullAllOfClassification**](Folder__Full_allOf_classification.md)> |  | [optional]
**is_associated_with_app_item** | Option<**bool**> | This field will return true if the folder or any ancestor of the folder is associated with at least one app item. Note that this will return true even if the context user does not have access to the app item(s) associated with the folder. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


