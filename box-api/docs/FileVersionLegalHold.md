# FileVersionLegalHold

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this file version legal hold. | [optional]
**r#type** | Option<**String**> | The value will always be `file_version_legal_hold`. | [optional]
**file_version** | Option<[**models::FileVersionMini**](FileVersion--Mini.md)> | The file version that is held. | [optional]
**file** | Option<[**models::FileMini**](File--Mini.md)> | The file for the file version held. Note that there is no guarantee that the current version of this file is held. | [optional]
**legal_hold_policy_assignments** | Option<[**Vec<models::LegalHoldPolicyAssignment>**](LegalHoldPolicyAssignment.md)> | List of assignments contributing to this Hold. | [optional]
**deleted_at** | Option<**String**> | Time that this File-Version-Legal-Hold was deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


