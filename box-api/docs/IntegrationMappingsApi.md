# \IntegrationMappingsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_integration_mappings_slack_id**](IntegrationMappingsApi.md#delete_integration_mappings_slack_id) | **DELETE** /integration_mappings/slack/{integration_mapping_id} | Delete Slack integration mapping
[**delete_integration_mappings_teams_id**](IntegrationMappingsApi.md#delete_integration_mappings_teams_id) | **DELETE** /integration_mappings/teams/{integration_mapping_id} | Delete Teams integration mapping
[**get_integration_mappings_slack**](IntegrationMappingsApi.md#get_integration_mappings_slack) | **GET** /integration_mappings/slack | List Slack integration mappings
[**get_integration_mappings_teams**](IntegrationMappingsApi.md#get_integration_mappings_teams) | **GET** /integration_mappings/teams | List Teams integration mappings
[**post_integration_mappings_slack**](IntegrationMappingsApi.md#post_integration_mappings_slack) | **POST** /integration_mappings/slack | Create Slack integration mapping
[**post_integration_mappings_teams**](IntegrationMappingsApi.md#post_integration_mappings_teams) | **POST** /integration_mappings/teams | Create Teams integration mapping
[**put_integration_mappings_slack_id**](IntegrationMappingsApi.md#put_integration_mappings_slack_id) | **PUT** /integration_mappings/slack/{integration_mapping_id} | Update Slack integration mapping
[**put_integration_mappings_teams_id**](IntegrationMappingsApi.md#put_integration_mappings_teams_id) | **PUT** /integration_mappings/teams/{integration_mapping_id} | Update Teams integration mapping



## delete_integration_mappings_slack_id

> delete_integration_mappings_slack_id(integration_mapping_id)
Delete Slack integration mapping

Deletes a [Slack integration mapping](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack).   You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_id** | **String** | An ID of an integration mapping. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_integration_mappings_teams_id

> delete_integration_mappings_teams_id(integration_mapping_id)
Delete Teams integration mapping

Deletes a [Teams integration mapping](https://support.box.com/hc/en-us/articles/360044681474-Using-Box-for-Teams). You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_id** | **String** | An ID of an integration mapping. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integration_mappings_slack

> models::IntegrationMappings get_integration_mappings_slack(marker, limit, partner_item_type, partner_item_id, box_item_id, box_item_type, is_manually_created)
List Slack integration mappings

Lists [Slack integration mappings](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack) in a users' enterprise.  You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**partner_item_type** | Option<**String**> | Mapped item type, for which the mapping should be returned. |  |
**partner_item_id** | Option<**String**> | ID of the mapped item, for which the mapping should be returned. |  |
**box_item_id** | Option<**String**> | Box item ID, for which the mappings should be returned. |  |
**box_item_type** | Option<**String**> | Box item type, for which the mappings should be returned. |  |
**is_manually_created** | Option<**bool**> | Whether the mapping has been manually created. |  |

### Return type

[**models::IntegrationMappings**](IntegrationMappings.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integration_mappings_teams

> models::IntegrationMappingsTeams get_integration_mappings_teams(partner_item_type, partner_item_id, box_item_id, box_item_type)
List Teams integration mappings

Lists [Teams integration mappings](https://support.box.com/hc/en-us/articles/360044681474-Using-Box-for-Teams) in a users' enterprise. You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_item_type** | Option<**String**> | Mapped item type, for which the mapping should be returned. |  |
**partner_item_id** | Option<**String**> | ID of the mapped item, for which the mapping should be returned. |  |
**box_item_id** | Option<**String**> | Box item ID, for which the mappings should be returned. |  |
**box_item_type** | Option<**String**> | Box item type, for which the mappings should be returned. |  |

### Return type

[**models::IntegrationMappingsTeams**](IntegrationMappingsTeams.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integration_mappings_slack

> models::IntegrationMapping post_integration_mappings_slack(integration_mapping_slack_create_request)
Create Slack integration mapping

Creates a [Slack integration mapping](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack) by mapping a Slack channel to a Box item.  You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_slack_create_request** | Option<[**IntegrationMappingSlackCreateRequest**](IntegrationMappingSlackCreateRequest.md)> |  |  |

### Return type

[**models::IntegrationMapping**](IntegrationMapping.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integration_mappings_teams

> models::IntegrationMappingTeams post_integration_mappings_teams(integration_mapping_teams_create_request)
Create Teams integration mapping

Creates a [Teams integration mapping](https://support.box.com/hc/en-us/articles/360044681474-Using-Box-for-Teams) by mapping a Teams channel to a Box item. You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_teams_create_request** | Option<[**IntegrationMappingTeamsCreateRequest**](IntegrationMappingTeamsCreateRequest.md)> |  |  |

### Return type

[**models::IntegrationMappingTeams**](IntegrationMappingTeams.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_integration_mappings_slack_id

> models::IntegrationMapping put_integration_mappings_slack_id(integration_mapping_id, put_integration_mappings_slack_id_request)
Update Slack integration mapping

Updates a [Slack integration mapping](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack). Supports updating the Box folder ID and options.  You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_id** | **String** | An ID of an integration mapping. | [required] |
**put_integration_mappings_slack_id_request** | Option<[**PutIntegrationMappingsSlackIdRequest**](PutIntegrationMappingsSlackIdRequest.md)> | At least one of `box_item` and `options` must be provided. |  |

### Return type

[**models::IntegrationMapping**](IntegrationMapping.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_integration_mappings_teams_id

> models::IntegrationMappingTeams put_integration_mappings_teams_id(integration_mapping_id, put_integration_mappings_teams_id_request)
Update Teams integration mapping

Updates a [Teams integration mapping](https://support.box.com/hc/en-us/articles/360044681474-Using-Box-for-Teams). Supports updating the Box folder ID and options. You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_id** | **String** | An ID of an integration mapping. | [required] |
**put_integration_mappings_teams_id_request** | Option<[**PutIntegrationMappingsTeamsIdRequest**](PutIntegrationMappingsTeamsIdRequest.md)> |  |  |

### Return type

[**models::IntegrationMappingTeams**](IntegrationMappingTeams.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

