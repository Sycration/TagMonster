# Outcome

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of a specific outcome. | 
**collaborators** | Option<[**models::CollaboratorVariable**](CollaboratorVariable.md)> | Lists collaborators affected by the workflow result. | [optional]
**completion_rule** | Option<[**models::CompletionRuleVariable**](CompletionRuleVariable.md)> | Determines if an action should be completed by all or any assignees. | [optional]
**file_collaborator_role** | Option<[**models::RoleVariable**](RoleVariable.md)> | Determines if the workflow outcome for a file affects a specific collaborator role. | [optional]
**task_collaborators** | Option<[**models::CollaboratorVariable**](CollaboratorVariable.md)> | Lists collaborators affected by the task workflow result. | [optional]
**role** | Option<[**models::RoleVariable**](RoleVariable.md)> | Determines if the workflow outcome affects a specific collaborator role. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


