# AiLlmEndpointParamsGoogle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of the AI LLM endpoint params object for Google. This parameter is **required**. | 
**temperature** | Option<**f64**> | The temperature is used for sampling during response generation, which occurs when `top-P` and `top-K` are applied. Temperature controls the degree of randomness in the token selection. | [optional]
**top_p** | Option<**f64**> | `Top-P` changes how the model selects tokens for output. Tokens are selected from the most (see `top-K`) to least probable until the sum of their probabilities equals the `top-P` value. | [optional]
**top_k** | Option<**f64**> | `Top-K` changes how the model selects tokens for output. A low `top-K` means the next selected token is the most probable among all tokens in the model's vocabulary (also called greedy decoding), while a high `top-K` means that the next token is selected from among the three most probable tokens by using temperature. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


