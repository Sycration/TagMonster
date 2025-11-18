# AppItemEventSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The id of the `AppItem`. | 
**r#type** | **String** | The type of the source that this event represents. Can only be `app_item`. | 
**app_item_type** | **String** | The type of the `AppItem`. | 
**user** | Option<[**models::UserMini**](User--Mini.md)> | The user that triggered the event. | [optional]
**group** | Option<[**models::GroupMini**](Group--Mini.md)> | The group that triggered the event. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


