# SearchResultWithSharedLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessible_via_shared_link** | Option<**String**> | The optional shared link through which the user has access to this item. This value is only returned for items for which the user has recently accessed the file through a shared link. For all other items this value will return `null`. | [optional]
**item** | Option<[**models::SearchResultWithSharedLinkItem**](SearchResultWithSharedLinkItem.md)> | The file, folder or web link that matched the search query. | [optional]
**r#type** | Option<**String**> | The result type. The value is always `search_result`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


