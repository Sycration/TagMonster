# IntegrationMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique identifier of a folder mapping (part of a composite key together with `integration_type`). | 
**r#type** | **String** | Mapping type. | 
**integration_type** | Option<**String**> | Identifies the Box partner app, with which the mapping is associated. Currently only supports Slack. (part of the composite key together with `id`). | [optional]
**is_manually_created** | Option<**bool**> | Identifies whether the mapping has been manually set (as opposed to being automatically created). | [optional]
**options** | Option<[**models::IntegrationMappingSlackOptions**](IntegrationMappingSlackOptions.md)> |  | [optional]
**created_by** | Option<[**models::UserIntegrationMappings**](UserIntegrationMappings.md)> | An object representing the user who created the integration mapping. | [optional]
**modified_by** | Option<[**models::UserIntegrationMappings**](UserIntegrationMappings.md)> | The user who last modified the integration mapping. | [optional]
**partner_item** | [**models::IntegrationMappingPartnerItemSlack**](IntegrationMappingPartnerItemSlack.md) | Mapped item object for Slack. | 
**box_item** | [**models::FolderMini**](Folder--Mini.md) | The Box folder, to which the object from the partner app domain (referenced in `partner_item_id`) is mapped. | 
**created_at** | Option<**String**> | When the integration mapping object was created. | [optional]
**modified_at** | Option<**String**> | When the integration mapping object was last modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


