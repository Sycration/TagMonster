# PutFilesIdMetadataGlobalBoxSkillsCardsRequestInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**op** | Option<**String**> | The value will always be `replace`. | [optional]
**path** | Option<**String**> | The JSON Path that represents the card to replace. In most cases this will be in the format `/cards/{index}` where `index` is the zero-indexed position of the card in the list of cards. | [optional]
**value** | Option<[**models::SkillCard**](SkillCard.md)> | The card to insert into the list of cards at the position defined by `path`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


