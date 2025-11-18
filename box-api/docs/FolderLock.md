# FolderLock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**folder** | Option<[**models::FolderMini**](Folder--Mini.md)> | The folder that the lock applies to. | [optional]
**id** | Option<**String**> | The unique identifier for this folder lock. | [optional]
**r#type** | Option<**String**> | The object type, always `folder_lock`. | [optional]
**created_by** | Option<[**models::UserBase**](User--Base.md)> | The user or group that created the lock. | [optional]
**created_at** | Option<**String**> | When the folder lock object was created. | [optional]
**locked_operations** | Option<[**models::FolderLockLockedOperations**](FolderLock_locked_operations.md)> |  | [optional]
**lock_type** | Option<**String**> | The lock type, always `freeze`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


