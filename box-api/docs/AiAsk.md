# AiAsk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | **String** | Box AI handles text documents with text representations up to 1MB in size, or a maximum of 25 files, whichever comes first. If the text file size exceeds 1MB, the first 1MB of text representation will be processed. Box AI handles image documents with a resolution of 1024 x 1024 pixels, with a maximum of 5 images or 5 pages for multi-page images. If the number of image or image pages exceeds 5, the first 5 images or pages will be processed. If you set mode parameter to `single_item_qa`, the items array can have one element only. Currently Box AI does not support multi-modal requests. If both images and text are sent Box AI will only process the text. | 
**prompt** | **String** | The prompt provided by the client to be answered by the LLM. The prompt's length is limited to 10000 characters. | 
**items** | [**Vec<models::AiItemAsk>**](AiItemAsk.md) | The items to be processed by the LLM, often files. | 
**dialogue_history** | Option<[**Vec<models::AiDialogueHistory>**](AiDialogueHistory.md)> | The history of prompts and answers previously passed to the LLM. This provides additional context to the LLM in generating the response. | [optional]
**include_citations** | Option<**bool**> | A flag to indicate whether citations should be returned. | [optional]
**ai_agent** | Option<[**models::AiAskAgent**](AiAskAgent.md)> | The AI agent to be used to handle the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


