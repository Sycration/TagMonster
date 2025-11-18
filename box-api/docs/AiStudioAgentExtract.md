# AiStudioAgentExtract

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of AI agent to be used for metadata extraction. | 
**access_state** | **String** | The state of the AI Agent capability. Possible values are: `enabled` and `disabled`. | 
**description** | **String** | The description of the AI agent. | 
**custom_instructions** | Option<**String**> | Custom instructions for the AI agent. | [optional]
**long_text** | Option<[**models::AiStudioAgentLongTextTool**](AiStudioAgentLongTextTool.md)> |  | [optional]
**basic_text** | Option<[**models::AiStudioAgentBasicTextTool**](AiStudioAgentBasicTextTool.md)> |  | [optional]
**basic_image** | Option<[**models::AiStudioAgentBasicTextTool**](AiStudioAgentBasicTextTool.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


