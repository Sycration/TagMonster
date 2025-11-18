# TermsOfServiceUserStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this terms of service user status. | 
**r#type** | **String** | The value will always be `terms_of_service_user_status`. | 
**tos** | Option<[**models::TermsOfServiceBase**](TermsOfService--Base.md)> | The terms of service. | [optional]
**user** | Option<[**models::UserMini**](User--Mini.md)> | The user. | [optional]
**is_accepted** | Option<**bool**> | If the user has accepted the terms of services. | [optional]
**created_at** | Option<**String**> | When the legal item was created. | [optional]
**modified_at** | Option<**String**> | When the legal item was modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


