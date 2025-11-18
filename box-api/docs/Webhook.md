# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this webhook. | [optional]
**r#type** | Option<**String**> | The value will always be `webhook`. | [optional]
**target** | Option<[**models::PostWebhooksRequestTarget**](post_webhooks_request_target.md)> |  | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created the webhook. | [optional]
**created_at** | Option<**String**> | A timestamp identifying the time that the webhook was created. | [optional]
**address** | Option<**String**> | The URL that is notified by this webhook. | [optional]
**triggers** | Option<**Vec<String>**> | An array of event names that this webhook is to be triggered for. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


