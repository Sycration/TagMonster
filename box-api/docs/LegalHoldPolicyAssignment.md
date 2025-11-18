# LegalHoldPolicyAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this legal hold assignment. | [optional]
**r#type** | Option<**String**> | The value will always be `legal_hold_policy_assignment`. | [optional]
**legal_hold_policy** | Option<[**models::LegalHoldPolicyMini**](LegalHoldPolicy--Mini.md)> | The policy that the legal hold policy assignment is part of. | [optional]
**assigned_to** | Option<[**models::LegalHoldPolicyAssignedItem**](LegalHoldPolicyAssignedItem.md)> | The item that the legal hold policy is assigned to. Includes type and ID. | [optional]
**assigned_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created the legal hold policy assignment. | [optional]
**assigned_at** | Option<**String**> | When the legal hold policy assignment object was created. | [optional]
**deleted_at** | Option<**String**> | When the assignment release request was sent. (Because it can take time for an assignment to fully delete, this isn't quite the same time that the assignment is fully deleted). If null, Assignment was not deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


