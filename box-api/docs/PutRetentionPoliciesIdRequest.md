# PutRetentionPoliciesIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy_name** | Option<**String**> | The name for the retention policy. | [optional]
**description** | Option<**String**> | The additional text description of the retention policy. | [optional]
**disposition_action** | Option<[**models::PutRetentionPoliciesIdRequestDispositionAction**](put_retention_policies_id_request_disposition_action.md)> |  | [optional]
**retention_type** | Option<**String**> | Specifies the retention type:  * `modifiable`: You can modify the retention policy. For example, you can add or remove folders, shorten or lengthen the policy duration, or delete the assignment. Use this type if your retention policy is not related to any regulatory purposes. * `non-modifiable`: You can modify the retention policy only in a limited way: add a folder, lengthen the duration, retire the policy, change the disposition action or notification settings. You cannot perform other actions, such as deleting the assignment or shortening the policy duration. Use this type to ensure compliance with regulatory retention policies.  When updating a retention policy, you can use `non-modifiable` type only. You can convert a `modifiable` policy to `non-modifiable`, but not the other way around. | [optional]
**retention_length** | Option<[**models::PostRetentionPoliciesRequestRetentionLength**](post_retention_policies_request_retention_length.md)> |  | [optional]
**status** | Option<**String**> | Used to retire a retention policy.  If not retiring a policy, do not include this parameter or set it to `null`. | [optional]
**can_owner_extend_retention** | Option<**bool**> | Determines if the owner of items under the policy can extend the retention when the original retention duration is about to end. | [optional]
**are_owners_notified** | Option<**bool**> | Determines if owners and co-owners of items under the policy are notified when the retention duration is about to end. | [optional]
**custom_notification_recipients** | Option<[**Vec<models::UserBase>**](User--Base.md)> | A list of users notified when the retention duration is about to end. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


