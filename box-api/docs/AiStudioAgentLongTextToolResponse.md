# AiStudioAgentLongTextToolResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model used for the AI agent for basic text. For specific model values, see the [available models list](g://box-ai/supported-models). | [optional]
**num_tokens_for_completion** | Option<**i32**> | The number of tokens for completion. | [optional]
**llm_endpoint_params** | Option<[**models::AiLlmEndpointParams**](AiLlmEndpointParams.md)> |  | [optional]
**system_message** | Option<**String**> | System messages try to help the LLM \"understand\" its role and what it is supposed to do. | [optional]
**prompt_template** | Option<**String**> | The prompt template contains contextual information of the request and the user prompt. When passing `prompt_template` parameters, you **must include** inputs for `{user_question}` and `{content}`. `{current_date}` is optional, depending on the use. | [optional]
**embeddings** | Option<[**models::AiAgentLongTextToolAllOfEmbeddings**](AiAgentLongTextTool_allOf_embeddings.md)> |  | [optional]
**is_custom_instructions_included** | Option<**bool**> | True if system message contains custom instructions placeholder, false otherwise. | [optional]
**warnings** | Option<**Vec<String>**> | Warnings concerning tool. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


