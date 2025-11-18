# CreateAiAgent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of agent used to handle queries. | 
**name** | **String** | The name of the AI Agent. | 
**access_state** | **String** | The state of the AI Agent. Possible values are: `enabled`, `disabled`, and `enabled_for_selected_users`. | 
**icon_reference** | Option<**String**> | The icon reference of the AI Agent. It should have format of the URL `https://cdn01.boxcdn.net/app-assets/aistudio/avatars/<file_name>` where possible values of `file_name` are: `logo_boxAi.png`,`logo_stamp.png`,`logo_legal.png`,`logo_finance.png`,`logo_config.png`,`logo_handshake.png`,`logo_analytics.png`,`logo_classification.png`. | [optional]
**allowed_entities** | Option<[**Vec<models::AiAgentAllowedEntity>**](AiAgentAllowedEntity.md)> | List of allowed users or groups. | [optional]
**ask** | Option<[**models::AiStudioAgentAsk**](AiStudioAgentAsk.md)> |  | [optional]
**text_gen** | Option<[**models::AiStudioAgentTextGen**](AiStudioAgentTextGen.md)> |  | [optional]
**extract** | Option<[**models::AiStudioAgentExtract**](AiStudioAgentExtract.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


