# FileVersionRetention

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this file version retention. | [optional]
**r#type** | Option<**String**> | The value will always be `file_version_retention`. | [optional]
**file_version** | Option<[**models::FileVersionMini**](FileVersion--Mini.md)> | The file version this file version retention was applied to. | [optional]
**file** | Option<[**models::FileMini**](File--Mini.md)> | The file this file version retention was applied to. | [optional]
**applied_at** | Option<**String**> | When this file version retention object was created. | [optional]
**disposition_at** | Option<**String**> | When the retention expires on this file version retention. | [optional]
**winning_retention_policy** | Option<[**models::RetentionPolicyMini**](RetentionPolicy--Mini.md)> | The winning retention policy applied to this file version retention. A file version can have multiple retention policies applied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


