# TemplateSigner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inputs** | Option<[**Vec<models::TemplateSignerInput>**](TemplateSignerInput.md)> |  | [optional][readonly]
**email** | Option<**String**> | Email address of the signer. | [optional]
**role** | Option<**String**> | Defines the role of the signer in the signature request. A role of `signer` needs to sign the document, a role `approver` approves the document and a `final_copy_reader` role only receives the final signed document and signing log. | [optional][default to Signer]
**is_in_person** | Option<**bool**> | Used in combination with an embed URL for a sender. After the sender signs, they will be redirected to the next `in_person` signer. | [optional]
**order** | Option<**i32**> | Order of the signer. | [optional]
**signer_group_id** | Option<**String**> | If provided, this value points signers that are assigned the same inputs and belongs to same signer group. A signer group is not a Box Group. It is an entity that belongs to the template itself and can only be used within Box Sign requests created from it. | [optional]
**label** | Option<**String**> | A placeholder label for the signer set by the template creator to differentiate between signers. | [optional]
**public_id** | Option<**String**> | An identifier for the signer. This can be used to identify a signer within the template. | [optional]
**is_password_required** | Option<**bool**> | If true for signers with a defined email, the password provided when the template was created is used by default.  If true for signers without a specified / defined email, the creator needs to provide a password when using the template. | [optional]
**is_phone_number_required** | Option<**bool**> | If true for signers with a defined email, the phone number provided when the template was created is used by default.  If true for signers without a specified / defined email, the template creator needs to provide a phone number when creating a request. | [optional]
**login_required** | Option<**bool**> | If true, the signer is required to login to access the document. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


