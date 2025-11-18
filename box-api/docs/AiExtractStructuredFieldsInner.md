# AiExtractStructuredFieldsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A unique identifier for the field. | 
**description** | Option<**String**> | A description of the field. | [optional]
**display_name** | Option<**String**> | The display name of the field. | [optional]
**prompt** | Option<**String**> | The context about the key that may include how to find and format it. | [optional]
**r#type** | Option<**String**> | The type of the field. It include but is not limited to string, float, date, enum, and multiSelect. | [optional]
**options** | Option<[**Vec<models::AiExtractStructuredFieldsInnerOptionsInner>**](AiExtractStructured_fields_inner_options_inner.md)> | A list of options for this field. This is most often used in combination with the enum and multiSelect field types. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


