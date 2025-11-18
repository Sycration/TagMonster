# Collaboration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this collaboration. | 
**r#type** | **String** | The value will always be `collaboration`. | 
**item** | Option<[**models::CollaborationItem**](CollaborationItem.md)> | The file or folder to which access is granted. The field is `null` when the collaboration `status` is `pending` or the collaboration is created on an app item (see `app_item` field). | [optional]
**app_item** | Option<[**models::AppItem**](AppItem.md)> | An `app_item` to which access is granted. The field is `null` when the collaboration is created on an item (see `item` field), or the `app_item` is inaccessible. The role cascades to all items associated with the `app_item`. | [optional]
**accessible_by** | Option<[**models::CollaborationAccessGrantee**](CollaborationAccessGrantee.md)> |  | [optional]
**invite_email** | Option<**String**> | The email address used to invite an unregistered collaborator, if they are not a registered user. | [optional]
**role** | Option<**String**> | The level of access granted. | [optional]
**expires_at** | Option<**String**> | When the collaboration will expire, or `null` if no expiration date is set. | [optional]
**is_access_only** | Option<**bool**> | If set to `true`, collaborators have access to shared items, but such items won't be visible in the All Files list. Additionally, collaborators won't see the path to the root folder for the shared item. | [optional]
**status** | Option<**String**> | The status of the collaboration invitation. If the status is `pending`, `login` and `name` return an empty string. | [optional]
**acknowledged_at** | Option<**String**> | When the `status` of the collaboration object changed to `accepted` or `rejected`. | [optional]
**created_by** | Option<[**models::UserCollaborations**](User--Collaborations.md)> | The user who created the collaboration object. | [optional]
**created_at** | Option<**String**> | When the collaboration object was created. | [optional]
**modified_at** | Option<**String**> | When the collaboration object was last modified. | [optional]
**acceptance_requirements_status** | Option<[**models::CollaborationAcceptanceRequirementsStatus**](Collaboration_acceptance_requirements_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


