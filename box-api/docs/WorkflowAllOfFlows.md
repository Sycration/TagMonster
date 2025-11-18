# WorkflowAllOfFlows

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The identifier of the flow. | [optional]
**r#type** | Option<**String**> | The flow's resource type. | [optional]
**trigger** | Option<[**serde_json::Value**](serde_json::Value.md)> | Trigger that initiates flow. | [optional]
**outcomes** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Actions that are completed once the flow is triggered. | [optional]
**created_at** | Option<**String**> | When this flow was created. | [optional]
**created_by** | Option<[**models::UserBase**](User--Base.md)> | The user who created this flow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


