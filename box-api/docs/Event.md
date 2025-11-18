# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The value will always be `event`. | [optional]
**created_at** | Option<**String**> | When the event object was created. | [optional]
**recorded_at** | Option<**String**> | When the event object was recorded in database. | [optional]
**event_id** | Option<**String**> | The ID of the event object. You can use this to detect duplicate events. | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user that performed the action represented by the event. Some events may be performed by users not logged into Box. In that case, not all attributes of the object are populated and the event is attributed to a unknown user (`user_id = 2`). | [optional]
**event_type** | Option<**String**> | An event type that can trigger an event. | [optional]
**session_id** | Option<**String**> | The session of the user that performed the action. Not all events will populate this attribute. | [optional]
**source** | Option<[**models::EventSourceResource**](EventSourceResource.md)> | The resource that triggered this event. For more information, check out the guide on event triggers. | [optional]
**additional_details** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | This object provides additional information about the event if available.  This can include how a user performed an event as well as additional information to correlate an event to external KeySafe logs. Not all events have an `additional_details` object.  This object is only available in the Enterprise Events. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


