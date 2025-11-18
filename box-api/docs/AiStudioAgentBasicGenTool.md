# AiStudioAgentBasicGenTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | Option<**String**> | The model used for the AI agent for basic text. For specific model values, see the [available models list](g://box-ai/supported-models). | [optional]
**num_tokens_for_completion** | Option<**i32**> | The number of tokens for completion. | [optional]
**llm_endpoint_params** | Option<[**models::AiLlmEndpointParams**](AiLlmEndpointParams.md)> |  | [optional]
**system_message** | Option<**String**> | System messages aim at helping the LLM understand its role and what it is supposed to do. The input for `{current_date}` is optional, depending on the use. | [optional]
**prompt_template** | Option<**String**> | The prompt template contains contextual information of the request and the user prompt.  When using the `prompt_template` parameter, you **must include** input for `{user_question}`. Inputs for `{current_date}` and `{content}` are optional, depending on the use. | [optional]
**embeddings** | Option<[**models::AiAgentLongTextToolAllOfEmbeddings**](AiAgentLongTextTool_allOf_embeddings.md)> |  | [optional]
**content_template** | Option<**String**> | How the content should be included in a request to the LLM. Input for `{content}` is optional, depending on the use. | [optional]
**is_custom_instructions_included** | Option<**bool**> | True if system message contains custom instructions placeholder, false otherwise. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


