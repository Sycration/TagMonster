# RetentionPolicyAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for a retention policy assignment. | 
**r#type** | **String** | The value will always be `retention_policy_assignment`. | 
**retention_policy** | Option<[**models::RetentionPolicyMini**](RetentionPolicy--Mini.md)> | A mini representation of a retention policy object that has been assigned to the content. | [optional]
**assigned_to** | Option<[**models::RetentionPolicyAssignmentAssignedTo**](RetentionPolicyAssignment_assigned_to.md)> |  | [optional]
**filter_fields** | Option<[**Vec<models::RetentionPolicyAssignmentFilterFieldsInner>**](RetentionPolicyAssignment_filter_fields_inner.md)> | An array of field objects. Values are only returned if the `assigned_to` type is `metadata_template`. Otherwise, the array is blank. | [optional]
**assigned_by** | Option<[**models::UserMini**](User--Mini.md)> | A mini user object representing the user that created the retention policy assignment. | [optional]
**assigned_at** | Option<**String**> | When the retention policy assignment object was created. | [optional]
**start_date_field** | Option<**String**> | The date the retention policy assignment begins. If the `assigned_to` type is `metadata_template`, this field can be a date field's metadata attribute key id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


