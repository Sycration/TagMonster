# Users

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The limit that was used for these entries. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. The maximum value varies by API. | [optional]
**next_marker** | Option<**String**> | The marker for the start of the next page of results. | [optional]
**prev_marker** | Option<**String**> | The marker for the start of the previous page of results. | [optional]
**total_count** | Option<**i64**> | One greater than the offset of the last entry in the entire collection. The total number of entries in the collection may be less than `total_count`.  This field is only returned for calls that use offset-based pagination. For marker-based paginated APIs, this field will be omitted. | [optional]
**offset** | Option<**i64**> | The 0-based offset of the first entry in this set. This will be the same as the `offset` query parameter.  This field is only returned for calls that use offset-based pagination. For marker-based paginated APIs, this field will be omitted. | [optional]
**order** | Option<[**Vec<models::CollectionsAllOfOrder>**](Collections_allOf_order.md)> | The order by which items are returned.  This field is only returned for calls that use offset-based pagination. For marker-based paginated APIs, this field will be omitted. | [optional]
**entries** | Option<[**Vec<models::UserFull>**](User--Full.md)> | A list of users. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


