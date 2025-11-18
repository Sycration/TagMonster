# PostRetentionPoliciesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy_name** | **String** | The name for the retention policy. | 
**description** | Option<**String**> | The additional text description of the retention policy. | [optional]
**policy_type** | **String** | The type of the retention policy. A retention policy type can either be `finite`, where a specific amount of time to retain the content is known upfront, or `indefinite`, where the amount of time to retain the content is still unknown. | 
**disposition_action** | **String** | The disposition action of the retention policy. `permanently_delete` deletes the content retained by the policy permanently. `remove_retention` lifts retention policy from the content, allowing it to be deleted by users once the retention policy has expired. | 
**retention_length** | Option<[**models::PostRetentionPoliciesRequestRetentionLength**](post_retention_policies_request_retention_length.md)> |  | [optional]
**retention_type** | Option<**String**> | Specifies the retention type:  * `modifiable`: You can modify the retention policy. For example, you can add or remove folders, shorten or lengthen the policy duration, or delete the assignment. Use this type if your retention policy is not related to any regulatory purposes.  * `non_modifiable`: You can modify the retention policy only in a limited way: add a folder, lengthen the duration, retire the policy, change the disposition action or notification settings. You cannot perform other actions, such as deleting the assignment or shortening the policy duration. Use this type to ensure compliance with regulatory retention policies. | [optional]
**can_owner_extend_retention** | Option<**bool**> | Whether the owner of a file will be allowed to extend the retention. | [optional]
**are_owners_notified** | Option<**bool**> | Whether owner and co-owners of a file are notified when the policy nears expiration. | [optional]
**custom_notification_recipients** | Option<[**Vec<models::UserMini>**](User--Mini.md)> | A list of users notified when the retention policy duration is about to end. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


