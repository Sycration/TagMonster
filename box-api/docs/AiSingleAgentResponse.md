# AiSingleAgentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the AI Agent. | 
**r#type** | Option<**String**> | The type of agent used to handle queries. | [optional]
**origin** | **String** | The provider of the AI Agent. | 
**name** | **String** | The name of the AI Agent. | 
**access_state** | **String** | The state of the AI Agent. Possible values are: `enabled`, `disabled`, and `enabled_for_selected_users`. | 
**created_by** | Option<[**models::UserBase**](User--Base.md)> | The user who created this agent. | [optional]
**created_at** | Option<**String**> | The ISO date-time formatted timestamp of when this AI agent was created. | [optional]
**modified_by** | Option<[**models::UserBase**](User--Base.md)> | The user who most recently modified this agent. | [optional]
**modified_at** | Option<**String**> | The ISO date-time formatted timestamp of when this AI agent was recently modified. | [optional]
**icon_reference** | Option<**String**> | The icon reference of the AI Agent. | [optional]
**allowed_entities** | Option<[**Vec<models::AiAgentAllowedEntity>**](AiAgentAllowedEntity.md)> | List of allowed users or groups. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


