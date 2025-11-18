# Invite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this invite. | 
**r#type** | **String** | The value will always be `invite`. | 
**invited_to** | Option<[**models::Enterprise**](Enterprise.md)> |  | [optional]
**actionable_by** | Option<[**models::UserMini**](User--Mini.md)> |  | [optional]
**invited_by** | Option<[**models::UserMini**](User--Mini.md)> |  | [optional]
**status** | Option<**String**> | The status of the invite. | [optional]
**created_at** | Option<**String**> | When the invite was created. | [optional]
**modified_at** | Option<**String**> | When the invite was modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


