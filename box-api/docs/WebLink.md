# WebLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this web link. | 
**r#type** | **String** | The value will always be `web_link`. | 
**etag** | Option<**String**> | The entity tag of this web link. Used with `If-Match` headers. | [optional]
**url** | Option<**String**> | The URL this web link points to. | [optional]
**sequence_id** | Option<**String**> | A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource. | [optional]
**name** | Option<**String**> | The name of the web link. | [optional]
**parent** | Option<[**models::FolderMini**](Folder--Mini.md)> | The parent object the web link belongs to. | [optional]
**description** | Option<**String**> | The description accompanying the web link. This is visible within the Box web application. | [optional]
**path_collection** | Option<[**models::TrashWebLinkRestoredPathCollection**](TrashWebLinkRestored_path_collection.md)> |  | [optional]
**created_at** | Option<**String**> | When this file was created on Box’s servers. | [optional]
**modified_at** | Option<**String**> | When this file was last updated on the Box servers. | [optional]
**trashed_at** | Option<**String**> | When this file was moved to the trash. | [optional]
**purged_at** | Option<**String**> | When this file will be permanently deleted. | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created this web link. | [optional]
**modified_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who last modified this web link. | [optional]
**owned_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who owns this web link. | [optional]
**shared_link** | Option<[**models::WebLinkAllOfSharedLink**](WebLink_allOf_shared_link.md)> |  | [optional]
**item_status** | Option<**String**> | Whether this item is deleted or not. Values include `active`, `trashed` if the file has been moved to the trash, and `deleted` if the file has been permanently deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


