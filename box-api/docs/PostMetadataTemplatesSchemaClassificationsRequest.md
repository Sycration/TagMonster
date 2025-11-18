# PostMetadataTemplatesSchemaClassificationsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | **String** | The scope in which to create the classifications. This should be `enterprise` or `enterprise_{id}` where `id` is the unique ID of the enterprise. | 
**template_key** | **String** | Defines the list of metadata templates. | 
**display_name** | **String** | The name of the template as shown in web and mobile interfaces. | 
**hidden** | Option<**bool**> | Determines if the classification template is hidden or available on web and mobile devices. | [optional]
**copy_instance_on_item_copy** | Option<**bool**> | Determines if classifications are copied along when the file or folder is copied. | [optional]
**fields** | [**Vec<models::PostMetadataTemplatesSchemaClassificationsRequestFieldsInner>**](post_metadata_templates_schema_classifications_request_fields_inner.md) | The classification template requires exactly one field, which holds all the valid classification values. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


