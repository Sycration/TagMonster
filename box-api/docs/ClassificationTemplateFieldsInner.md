# ClassificationTemplateFieldsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique ID of the field. | 
**r#type** | **String** | The array item type. | 
**key** | **String** | Defines classifications  available in the enterprise. | 
**display_name** | **String** | The value will always be `Classification`. | 
**hidden** | Option<**bool**> | Classifications are always visible to web and mobile users. | [optional]
**options** | [**Vec<models::ClassificationTemplateFieldsInnerOptionsInner>**](ClassificationTemplate_fields_inner_options_inner.md) | A list of classifications available in this enterprise. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


