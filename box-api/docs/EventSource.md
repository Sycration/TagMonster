# EventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_type** | **String** | The type of the item that the event represents. Can be `file` or `folder`. | 
**item_id** | **String** | The unique identifier that represents the item. | 
**item_name** | **String** | The name of the item. | 
**classification** | Option<[**models::EventSourceClassification**](EventSource_classification.md)> |  | [optional]
**parent** | Option<[**models::FolderMini**](Folder--Mini.md)> | The optional folder that this folder is located within.  This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**owned_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who owns this item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


