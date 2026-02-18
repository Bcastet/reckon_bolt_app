# \CompetitiveDraftSlotsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**competitive_draft_slots_aggregate**](CompetitiveDraftSlotsApi.md#competitive_draft_slots_aggregate) | **GET** /CompetitiveDraftSlots/operations/aggregate | 
[**competitive_draft_slots_field_values**](CompetitiveDraftSlotsApi.md#competitive_draft_slots_field_values) | **GET** /CompetitiveDraftSlots/operations/field-values | 
[**competitive_draft_slots_get**](CompetitiveDraftSlotsApi.md#competitive_draft_slots_get) | **GET** /CompetitiveDraftSlots/item/{id} | 
[**competitive_draft_slots_list**](CompetitiveDraftSlotsApi.md#competitive_draft_slots_list) | **GET** /CompetitiveDraftSlots/list | 
[**competitive_draft_slots_variable_distribution**](CompetitiveDraftSlotsApi.md#competitive_draft_slots_variable_distribution) | **GET** /CompetitiveDraftSlots/operations/variable-distribution | 



## competitive_draft_slots_aggregate

> Vec<models::CompetitiveDraftSlotsAggregate> competitive_draft_slots_aggregate(groupby, ordering, additional_filters, aggregates, draft_action_id, having, index_on, league, map, metrics, phase, series, split, team1, team2, teams, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**draft_action_id** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**index_on** | Option<**String**> | Index results on a column |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**phase** | Option<**String**> |  |  |
**series** | Option<**i32**> |  |  |
**split** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**teams** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |

### Return type

[**Vec<models::CompetitiveDraftSlotsAggregate>**](CompetitiveDraftSlotsAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_draft_slots_field_values

> Vec<String> competitive_draft_slots_field_values(field, ordering, additional_filters, draft_action_id, league, map, phase, series, split, team1, team2, teams, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**draft_action_id** | Option<**String**> |  |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**phase** | Option<**String**> |  |  |
**series** | Option<**i32**> |  |  |
**split** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**teams** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_draft_slots_get

> models::CompetitiveDraftSlots competitive_draft_slots_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CompetitiveDraftSlots**](CompetitiveDraftSlots.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_draft_slots_list

> Vec<models::CompetitiveDraftSlots> competitive_draft_slots_list(additional_filters, annotations, draft_action_id, league, map, only_values, ordering, phase, series, split, team1, team2, teams, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**draft_action_id** | Option<**String**> |  |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**phase** | Option<**String**> |  |  |
**series** | Option<**i32**> |  |  |
**split** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**teams** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |

### Return type

[**Vec<models::CompetitiveDraftSlots>**](CompetitiveDraftSlots.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_draft_slots_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> competitive_draft_slots_variable_distribution(bucket_size, max, metric, min, additional_filters, draft_action_id, league, map, phase, series, split, team1, team2, teams, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**draft_action_id** | Option<**String**> |  |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**phase** | Option<**String**> |  |  |
**series** | Option<**i32**> |  |  |
**split** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**teams** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

