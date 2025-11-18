# AiExtractStructuredResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**answer** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | AI extract response. The content of this response may vary depending on the requested configuration. | 
**created_at** | **String** | The ISO date formatted timestamp of when the answer to the prompt was created. | 
**completion_reason** | Option<**String**> | The reason the response finishes. | [optional]
**ai_agent_info** | Option<[**models::AiAgentInfo**](AiAgentInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


