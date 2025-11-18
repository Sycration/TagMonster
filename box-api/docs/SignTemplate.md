# SignTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The value will always be `sign-template`. | [optional]
**id** | Option<**String**> | Template identifier. | [optional]
**name** | Option<**String**> | The name of the template. | [optional]
**email_subject** | Option<**String**> | Subject of signature request email. This is cleaned by sign request. If this field is not passed, a default subject will be used. | [optional]
**email_message** | Option<**String**> | Message to include in signature request email. The field is cleaned through sanitization of specific characters. However, some html tags are allowed. Links included in the message are also converted to hyperlinks in the email. The message may contain the following html tags including `a`, `abbr`, `acronym`, `b`, `blockquote`, `code`, `em`, `i`, `ul`, `li`, `ol`, and `strong`. Be aware that when the text to html ratio is too high, the email may end up in spam filters. Custom styles on these tags are not allowed. If this field is not passed, a default message will be used. | [optional]
**days_valid** | Option<**i32**> | Set the number of days after which the created signature request will automatically expire if not completed. By default, we do not apply any expiration date on signature requests, and the signature request does not expire. | [optional]
**parent_folder** | Option<[**models::FolderMini**](Folder--Mini.md)> | The destination folder to place final, signed document and signing log. Only `ID` and `type` fields are required. The root folder, folder ID `0`, cannot be used. | [optional]
**source_files** | Option<[**Vec<models::FileMini>**](File--Mini.md)> | List of files to create a signing document from. Only the ID and type fields are required for each file. | [optional]
**are_fields_locked** | Option<**bool**> | Indicates if the template input fields are editable or not. | [optional]
**are_options_locked** | Option<**bool**> | Indicates if the template document options are editable or not, for example renaming the document. | [optional]
**are_recipients_locked** | Option<**bool**> | Indicates if the template signers are editable or not. | [optional]
**are_email_settings_locked** | Option<**bool**> | Indicates if the template email settings are editable or not. | [optional]
**are_files_locked** | Option<**bool**> | Indicates if the template files are editable or not. This includes deleting or renaming template files. | [optional]
**signers** | Option<[**Vec<models::TemplateSigner>**](TemplateSigner.md)> | Array of signers for the template.  **Note**: It may happen that some signers specified in the template belong to conflicting [segments](r://shield-information-barrier-segment-member) (user groups). This means that due to the security policies, users are assigned to segments to prevent exchanges or communication that could lead to ethical conflicts. In such a case, an attempt to send a sign request based on a template that lists signers in conflicting segments will result in an error.  Read more about [segments and ethical walls](https://support.box.com/hc/en-us/articles/9920431507603-Understanding-Information-Barriers#h_01GFVJEHQA06N7XEZ4GCZ9GFAQ). | [optional]
**additional_info** | Option<[**models::SignTemplateAllOfAdditionalInfo**](SignTemplate_allOf_additional_info.md)> |  | [optional]
**ready_sign_link** | Option<[**models::SignTemplateAllOfReadySignLink**](SignTemplate_allOf_ready_sign_link.md)> |  | [optional]
**custom_branding** | Option<[**models::SignTemplateAllOfCustomBranding**](SignTemplate_allOf_custom_branding.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


