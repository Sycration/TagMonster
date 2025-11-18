# AiStudioAgentTextGenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of AI agent used for generating text. | 
**access_state** | **String** | The state of the AI Agent capability. Possible values are: `enabled` and `disabled`. | 
**description** | **String** | The description of the AI agent. | 
**custom_instructions** | Option<**String**> | Custom instructions for the AI agent. | [optional]
**suggested_questions** | Option<**Vec<String>**> | Suggested questions for the AI agent. If null, suggested question will be generated. If empty, no suggested questions will be displayed. | [optional]
**basic_gen** | Option<[**models::AiStudioAgentBasicGenToolResponse**](AiStudioAgentBasicGenToolResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


