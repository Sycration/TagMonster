# PostCollaborationsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item** | [**models::PostCollaborationsRequestItem**](post_collaborations_request_item.md) |  | 
**accessible_by** | [**models::PostCollaborationsRequestAccessibleBy**](post_collaborations_request_accessible_by.md) |  | 
**role** | **String** | The level of access granted. | 
**is_access_only** | Option<**bool**> | If set to `true`, collaborators have access to shared items, but such items won't be visible in the All Files list. Additionally, collaborators won't see the path to the root folder for the shared item. | [optional]
**can_view_path** | Option<**bool**> | Determines if the invited users can see the entire parent path to the associated folder. The user will not gain privileges in any parent folder and therefore can not see content the user is not collaborated on.  Be aware that this meaningfully increases the time required to load the invitee's **All Files** page. We recommend you limit the number of collaborations with `can_view_path` enabled to 1,000 per user.  Only owner or co-owners can invite collaborators with a `can_view_path` of `true`.  `can_view_path` can only be used for folder collaborations. | [optional]
**expires_at** | Option<**String**> | Set the expiration date for the collaboration. At this date, the collaboration will be automatically removed from the item.  This feature will only work if the **Automatically remove invited collaborators: Allow folder owners to extend the expiry date** setting has been enabled in the **Enterprise Settings** of the **Admin Console**. When the setting is not enabled, collaborations can not have an expiry date and a value for this field will be result in an error. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


