# AiLlmEndpointParamsIbm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the AI LLM endpoint params object for IBM. This parameter is **required**. | 
**temperature** | Option<**f64**> | What sampling temperature to use, between 0 and 1. Higher values like 0.8 will make the output more random,  while lower values like 0.2 will make it more focused and deterministic.  We generally recommend altering this or `top_p` but not both. | [optional]
**top_p** | Option<**f64**> | An alternative to sampling with temperature, called nucleus sampling, where the model considers the results  of the tokens with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability  mass are considered. We generally recommend altering this or temperature but not both. | [optional]
**top_k** | Option<**f64**> | `Top-K` changes how the model selects tokens for output. A low `top-K` means the next selected token is the most probable among all tokens in the model's vocabulary (also called greedy decoding), while a high `top-K` means that the next token is selected from among the three most probable tokens by using temperature. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


