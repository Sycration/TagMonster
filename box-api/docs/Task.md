# Task

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this task. | [optional]
**r#type** | Option<**String**> | The value will always be `task`. | [optional]
**item** | Option<[**models::FileMini**](File--Mini.md)> | The file associated with the task. | [optional]
**due_at** | Option<**String**> | When the task is due. | [optional]
**action** | Option<**String**> | The type of task the task assignee will be prompted to perform. | [optional]
**message** | Option<**String**> | A message that will be included with the task. | [optional]
**task_assignment_collection** | Option<[**models::TaskAssignments**](TaskAssignments.md)> | A collection of task assignment objects associated with the task. | [optional]
**is_completed** | Option<**bool**> | Whether the task has been completed. | [optional]
**created_by** | Option<[**models::UserMini**](User--Mini.md)> | The user who created the task. | [optional]
**created_at** | Option<**String**> | When the task object was created. | [optional]
**completion_rule** | Option<**String**> | Defines which assignees need to complete this task before the task is considered completed.  * `all_assignees` requires all assignees to review or approve the task in order for it to be considered completed. * `any_assignee` accepts any one assignee to review or approve the task in order for it to be considered completed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


