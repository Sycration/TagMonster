# UserCollaborations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this user. | 
**r#type** | **String** | The value will always be `user`. | 
**name** | Option<**String**> | The display name of this user. If the collaboration status is `pending`, an empty string is returned. | [optional]
**login** | Option<**String**> | The primary email address of this user. If the collaboration status is `pending`, an empty string is returned. | [optional]
**is_active** | Option<**bool**> | If set to `false`, the user is either deactivated or deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


