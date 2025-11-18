# GroupMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this group membership. | [optional]
**r#type** | Option<**String**> | The value will always be `group_membership`. | [optional]
**user** | Option<[**models::UserMini**](User--Mini.md)> | The user that the membership applies to. | [optional]
**group** | Option<[**models::GroupMini**](Group--Mini.md)> | The group that the membership applies to. | [optional]
**role** | Option<**String**> | The role of the user in the group. | [optional]
**created_at** | Option<**String**> | The time this membership was created. | [optional]
**modified_at** | Option<**String**> | The time this membership was last modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


