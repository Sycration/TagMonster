# \AiStudioApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_ai_agents_id**](AiStudioApi.md#delete_ai_agents_id) | **DELETE** /ai_agents/{agent_id} | Delete AI agent
[**get_ai_agents**](AiStudioApi.md#get_ai_agents) | **GET** /ai_agents | List AI agents
[**get_ai_agents_id**](AiStudioApi.md#get_ai_agents_id) | **GET** /ai_agents/{agent_id} | Get AI agent by agent ID
[**post_ai_agents**](AiStudioApi.md#post_ai_agents) | **POST** /ai_agents | Create AI agent
[**put_ai_agents_id**](AiStudioApi.md#put_ai_agents_id) | **PUT** /ai_agents/{agent_id} | Update AI agent



## delete_ai_agents_id

> delete_ai_agents_id(agent_id)
Delete AI agent

Deletes an AI agent using the provided parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the agent to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ai_agents

> models::AiMultipleAgentResponse get_ai_agents(mode, fields, agent_state, include_box_default, marker, limit)
List AI agents

Lists AI agents based on the provided parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mode** | Option<[**Vec<String>**](String.md)> | The mode to filter the agent config to return. Possible values are: `ask`, `text_gen`, and `extract`. |  |
**fields** | Option<[**Vec<String>**](String.md)> | The fields to return in the response. |  |
**agent_state** | Option<[**Vec<String>**](String.md)> | The state of the agents to return. Possible values are: `enabled`, `disabled` and `enabled_for_selected_users`. |  |
**include_box_default** | Option<**bool**> | Whether to include the Box default agents in the response. |  |[default to false]
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**models::AiMultipleAgentResponse**](AiMultipleAgentResponse.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ai_agents_id

> models::AiSingleAgentResponseFull get_ai_agents_id(agent_id, fields)
Get AI agent by agent ID

Gets an AI Agent using the `agent_id` parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The agent id to get. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | The fields to return in the response. |  |

### Return type

[**models::AiSingleAgentResponseFull**](AiSingleAgentResponse--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ai_agents

> models::AiSingleAgentResponseFull post_ai_agents(create_ai_agent)
Create AI agent

Creates an AI agent. At least one of the following capabilities must be provided: `ask`, `text_gen`, `extract`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ai_agent** | Option<[**CreateAiAgent**](CreateAiAgent.md)> |  |  |

### Return type

[**models::AiSingleAgentResponseFull**](AiSingleAgentResponse--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_ai_agents_id

> models::AiSingleAgentResponseFull put_ai_agents_id(agent_id, create_ai_agent)
Update AI agent

Updates an AI agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the agent to update. | [required] |
**create_ai_agent** | Option<[**CreateAiAgent**](CreateAiAgent.md)> |  |  |

### Return type

[**models::AiSingleAgentResponseFull**](AiSingleAgentResponse--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

