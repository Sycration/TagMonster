# AiExtractStructured

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**items** | [**Vec<models::AiItemBase>**](AiItem--Base.md) | The items to be processed by the LLM. Currently you can use files only. | 
**metadata_template** | Option<[**models::AiExtractStructuredMetadataTemplate**](AiExtractStructured_metadata_template.md)> |  | [optional]
**fields** | Option<[**Vec<models::AiExtractStructuredFieldsInner>**](AiExtractStructured_fields_inner.md)> | The fields to be extracted from the provided items. For your request to work, you must provide either `metadata_template` or `fields`, but not both. | [optional]
**ai_agent** | Option<[**models::AiExtractStructuredAgent**](AiExtractStructuredAgent.md)> | The AI agent to be used for the structured extraction. Defaults to the Standard Agent if not specified. If you want to use Enhanced Extract Agent, see [Enhanced Extract Agent](g://box-ai/ai-tutorials/extract-metadata-structured/#enhanced-extract-agent) for details. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


