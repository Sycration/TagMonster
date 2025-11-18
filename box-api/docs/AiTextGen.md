# AiTextGen

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prompt** | **String** | The prompt provided by the client to be answered by the LLM. The prompt's length is limited to 10000 characters. | 
**items** | [**Vec<models::AiTextGenItemsInner>**](AiTextGen_items_inner.md) | The items to be processed by the LLM, often files. The array can include **exactly one** element.  **Note**: Box AI handles documents with text representations up to 1MB in size. If the file size exceeds 1MB, the first 1MB of text representation will be processed. | 
**dialogue_history** | Option<[**Vec<models::AiDialogueHistory>**](AiDialogueHistory.md)> | The history of prompts and answers previously passed to the LLM. This parameter provides the additional context to the LLM when generating the response. | [optional]
**ai_agent** | Option<[**models::AiTextGenAgent**](AiTextGenAgent.md)> | The AI agent to be used for generating text. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


