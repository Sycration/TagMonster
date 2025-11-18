# AiExtract

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prompt** | **String** | The prompt provided to a Large Language Model (LLM) in the request. The prompt can be up to 10000 characters long and it can be an XML or a JSON schema. | 
**items** | [**Vec<models::AiItemBase>**](AiItem--Base.md) | The items that LLM will process. Currently, you can use files only. | 
**ai_agent** | Option<[**models::AiExtractAgent**](AiExtractAgent.md)> | The AI agent to be used for the extraction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


