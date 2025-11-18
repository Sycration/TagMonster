# IntegrationMappingTeams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique identifier of a folder mapping (part of a composite key together with `integration_type`). | 
**r#type** | **String** | Mapping type. | 
**integration_type** | Option<**String**> | Identifies the Box partner app, with which the mapping is associated. Supports Slack and Teams. (part of the composite key together with `id`). | [optional]
**is_overridden_by_manual_mapping** | Option<**bool**> | Identifies whether the mapping has been manually set by the team owner from UI for channels (as opposed to being automatically created). | [optional]
**partner_item** | [**models::IntegrationMappingPartnerItemTeams**](IntegrationMappingPartnerItemTeams.md) | Mapped item object for Teams. | 
**box_item** | [**models::FolderReference**](FolderReference.md) | The Box folder, to which the object from the partner app domain (referenced in `partner_item_id`) is mapped. | 
**created_at** | Option<**String**> | When the integration mapping object was created. | [optional]
**modified_at** | Option<**String**> | When the integration mapping object was last modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


