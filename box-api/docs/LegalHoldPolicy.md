# LegalHoldPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this legal hold policy. | 
**r#type** | **String** | The value will always be `legal_hold_policy`. | 
**policy_name** | Option<**String**> | Name of the legal hold policy. | [optional]
**description** | Option<**String**> | Description of the legal hold policy. Optional property with a 500 character limit. | [optional]
**status** | Option<**String**> | Possible values: * 'active' - the policy is not in a transition state. * 'applying' - that the policy is in the process of   being applied. * 'releasing' - that the process is in the process   of being released. * 'released' - the policy is no longer active. | [optional]
**assignment_counts** | Option<[**models::LegalHoldPolicyAllOfAssignmentCounts**](LegalHoldPolicy_allOf_assignment_counts.md)> |  | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created the legal hold policy object. | [optional]
**created_at** | Option<**String**> | When the legal hold policy object was created. | [optional]
**modified_at** | Option<**String**> | When the legal hold policy object was modified. Does not update when assignments are added or removed. | [optional]
**deleted_at** | Option<**String**> | When the policy release request was sent. (Because it can take time for a policy to fully delete, this isn't quite the same time that the policy is fully deleted).  If `null`, the policy was not deleted. | [optional]
**filter_started_at** | Option<**String**> | User-specified, optional date filter applies to Custodian assignments only. | [optional]
**filter_ended_at** | Option<**String**> | User-specified, optional date filter applies to Custodian assignments only. | [optional]
**release_notes** | Option<**String**> | Optional notes about why the policy was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


