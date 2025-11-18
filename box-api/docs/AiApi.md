# \AiApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ai_agent_default**](AiApi.md#get_ai_agent_default) | **GET** /ai_agent_default | Get AI agent default configuration
[**post_ai_ask**](AiApi.md#post_ai_ask) | **POST** /ai/ask | Ask question
[**post_ai_extract**](AiApi.md#post_ai_extract) | **POST** /ai/extract | Extract metadata (freeform)
[**post_ai_extract_structured**](AiApi.md#post_ai_extract_structured) | **POST** /ai/extract_structured | Extract metadata (structured)
[**post_ai_text_gen**](AiApi.md#post_ai_text_gen) | **POST** /ai/text_gen | Generate text



## get_ai_agent_default

> models::AiAgent get_ai_agent_default(mode, language, model)
Get AI agent default configuration

Get the AI agent default config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mode** | **String** | The mode to filter the agent config to return. | [required] |
**language** | Option<**String**> | The ISO language code to return the agent config for. If the language is not supported the default agent config is returned. |  |
**model** | Option<**String**> | The model to return the default agent config for. |  |

### Return type

[**models::AiAgent**](AiAgent.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ai_ask

> models::AiResponseFull post_ai_ask(ai_ask)
Ask question

Sends an AI request to supported LLMs and returns an answer specifically focused on the user's question given the provided context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_ask** | Option<[**AiAsk**](AiAsk.md)> |  |  |

### Return type

[**models::AiResponseFull**](AiResponse--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ai_extract

> models::AiResponse post_ai_extract(ai_extract)
Extract metadata (freeform)

Sends an AI request to supported Large Language Models (LLMs) and extracts metadata in form of key-value pairs. In this request, both the prompt and the output can be freeform. Metadata template setup before sending the request is not required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_extract** | Option<[**AiExtract**](AiExtract.md)> |  |  |

### Return type

[**models::AiResponse**](AiResponse.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ai_extract_structured

> models::AiExtractStructuredResponse post_ai_extract_structured(ai_extract_structured)
Extract metadata (structured)

Sends an AI request to supported Large Language Models (LLMs) and returns extracted metadata as a set of key-value pairs.  To define the extraction structure, provide either a metadata template or a list of fields. To learn more about creating templates, see [Creating metadata templates in the Admin Console](https://support.box.com/hc/en-us/articles/360044194033-Customizing-Metadata-Templates) or use the [metadata template API](g://metadata/templates/create).   This endpoint also supports [Enhanced Extract Agent](g://box-ai/ai-tutorials/extract-metadata-structured/#enhanced-extract-agent).  For information about supported file formats and languages, see the [Extract metadata from file (structured)](g://box-ai/ai-tutorials/extract-metadata-structured) API guide.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_extract_structured** | Option<[**AiExtractStructured**](AiExtractStructured.md)> |  |  |

### Return type

[**models::AiExtractStructuredResponse**](AiExtractStructuredResponse.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ai_text_gen

> models::AiResponse post_ai_text_gen(ai_text_gen)
Generate text

Sends an AI request to supported Large Language Models (LLMs) and returns generated text based on the provided prompt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_text_gen** | Option<[**AiTextGen**](AiTextGen.md)> |  |  |

### Return type

[**models::AiResponse**](AiResponse.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

