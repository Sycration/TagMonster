# KeywordSkillCard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The optional date and time this card was created at. | [optional]
**r#type** | **String** | The value will always be `skill_card`. | 
**skill_card_type** | **String** | The value will always be `keyword`. | 
**skill_card_title** | Option<[**models::KeywordSkillCardSkillCardTitle**](KeywordSkillCard_skill_card_title.md)> |  | [optional]
**skill** | [**models::KeywordSkillCardSkill**](KeywordSkillCard_skill.md) |  | 
**invocation** | [**models::KeywordSkillCardInvocation**](KeywordSkillCard_invocation.md) |  | 
**entries** | [**Vec<models::KeywordSkillCardEntriesInner>**](KeywordSkillCard_entries_inner.md) | An list of entries in the metadata card. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


