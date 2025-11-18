# TermsOfService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this terms of service. | 
**r#type** | **String** | The value will always be `terms_of_service`. | 
**status** | Option<**String**> | Whether these terms are enabled or not. | [optional]
**enterprise** | Option<[**models::TermsOfServiceAllOfEnterprise**](TermsOfService_allOf_enterprise.md)> |  | [optional]
**tos_type** | Option<**String**> | Whether to apply these terms to managed users or external users. | [optional]
**text** | Option<**String**> | The text for your terms and conditions. This text could be empty if the `status` is set to `disabled`. | [optional]
**created_at** | Option<**String**> | When the legal item was created. | [optional]
**modified_at** | Option<**String**> | When the legal item was modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


