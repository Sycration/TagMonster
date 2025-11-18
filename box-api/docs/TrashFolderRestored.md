# TrashFolderRestored

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`. | [optional]
**etag** | Option<**String**> | The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened. | [optional]
**r#type** | Option<**String**> | The value will always be `folder`. | [optional]
**sequence_id** | Option<**String**> | A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource. | [optional]
**name** | Option<**String**> | The name of the folder. | [optional]
**created_at** | Option<**String**> | The date and time when the folder was created. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**modified_at** | Option<**String**> | The date and time when the folder was last updated. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**description** | Option<**String**> | The optional description of this folder. | [optional]
**size** | Option<**i64**> | The folder size in bytes.  Be careful parsing this integer as its value can get very large. | [optional]
**path_collection** | Option<[**models::FileAllOfPathCollection**](File_allOf_path_collection.md)> |  | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created this folder. | [optional]
**modified_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who last modified this folder. | [optional]
**trashed_at** | Option<**String**> | The time at which this folder was put in the trash - becomes `null` after restore. | [optional]
**purged_at** | Option<**String**> | The time at which this folder is expected to be purged from the trash  - becomes `null` after restore. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this folder was originally created. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this folder was last updated. | [optional]
**owned_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who owns this folder. | [optional]
**shared_link** | Option<**String**> | The shared link for this file. This will be `null` if a folder had been trashed, even though the original shared link does become active again. | [optional]
**folder_upload_email** | Option<**String**> | The folder upload email for this folder. This will be `null` if a folder has been trashed, even though the original upload email does become active again. | [optional]
**parent** | Option<[**models::FolderMini**](Folder--Mini.md)> | The optional folder that this folder is located within.  This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash, * `trashed` when the item has been moved to the trash but not deleted, * `deleted` when the item has been permanently deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


