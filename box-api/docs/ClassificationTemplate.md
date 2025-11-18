# ClassificationTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the classification template. | 
**r#type** | **String** | The value will always be `metadata_template`. | 
**scope** | **String** | The scope of the classification template. This is in the format `enterprise_{id}` where the `id` is the enterprise ID. | 
**template_key** | **String** | The value will always be `securityClassification-6VMVochwUWo`. | 
**display_name** | **String** | The name of this template as shown in web and mobile interfaces. | 
**hidden** | Option<**bool**> | Determines if the template is always available in web and mobile interfaces. | [optional]
**copy_instance_on_item_copy** | Option<**bool**> | Determines if  classifications are copied along when the file or folder is copied. | [optional]
**fields** | [**Vec<models::ClassificationTemplateFieldsInner>**](ClassificationTemplate_fields_inner.md) | A list of fields for this classification template. This includes only one field, the `Box__Security__Classification__Key`, which defines the different classifications available in this enterprise. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


