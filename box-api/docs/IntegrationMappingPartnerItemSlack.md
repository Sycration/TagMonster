# IntegrationMappingPartnerItemSlack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the mapped item referenced in `id`. | 
**id** | **String** | ID of the mapped item (of type referenced in `type`). | 
**slack_workspace_id** | Option<**String**> | ID of the Slack workspace with which the item is associated. Use this parameter if Box for Slack is installed at a workspace level. Do not use `slack_org_id` at the same time. | [optional]
**slack_org_id** | Option<**String**> | ID of the Slack org with which the item is associated. Use this parameter if Box for Slack is installed at the org level. Do not use `slack_workspace_id` at the same time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


