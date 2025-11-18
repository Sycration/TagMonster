# AiStudioAgentAskResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of AI agent used to ask questions. | 
**access_state** | **String** | The state of the AI Agent capability. Possible values are: `enabled` and `disabled`. | 
**description** | **String** | The description of the AI agent. | 
**custom_instructions** | Option<**String**> | Custom instructions for the AI agent. | [optional]
**suggested_questions** | Option<**Vec<String>**> | Suggested questions for the AI agent. If null, suggested question will be generated. If empty, no suggested questions will be displayed. | [optional]
**long_text** | Option<[**models::AiStudioAgentLongTextToolResponse**](AiStudioAgentLongTextToolResponse.md)> |  | [optional]
**basic_text** | Option<[**models::AiStudioAgentBasicTextToolResponse**](AiStudioAgentBasicTextToolResponse.md)> |  | [optional]
**basic_image** | Option<[**models::AiStudioAgentBasicTextToolResponse**](AiStudioAgentBasicTextToolResponse.md)> |  | [optional]
**spreadsheet** | Option<[**models::AiStudioAgentSpreadsheetToolResponse**](AiStudioAgentSpreadsheetToolResponse.md)> |  | [optional]
**long_text_multi** | Option<[**models::AiStudioAgentLongTextToolResponse**](AiStudioAgentLongTextToolResponse.md)> |  | [optional]
**basic_text_multi** | Option<[**models::AiStudioAgentBasicTextToolResponse**](AiStudioAgentBasicTextToolResponse.md)> |  | [optional]
**basic_image_multi** | Option<[**models::AiStudioAgentBasicTextToolResponse**](AiStudioAgentBasicTextToolResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


