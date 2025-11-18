# RecentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The value will always be `recent_item`. | [optional]
**item** | Option<[**models::RecentItemResource**](RecentItemResource.md)> | The item that was recently accessed. | [optional]
**interaction_type** | Option<**String**> | The most recent type of access the user performed on the item. | [optional]
**interacted_at** | Option<**String**> | The time of the most recent interaction. | [optional]
**interaction_shared_link** | Option<**String**> | If the item was accessed through a shared link it will appear here, otherwise this will be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


