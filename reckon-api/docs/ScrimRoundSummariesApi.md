# \ScrimRoundSummariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scrim_round_summaries_aggregate**](ScrimRoundSummariesApi.md#scrim_round_summaries_aggregate) | **GET** /ScrimRoundSummaries/operations/aggregate | 
[**scrim_round_summaries_field_values**](ScrimRoundSummariesApi.md#scrim_round_summaries_field_values) | **GET** /ScrimRoundSummaries/operations/field-values | 
[**scrim_round_summaries_get**](ScrimRoundSummariesApi.md#scrim_round_summaries_get) | **GET** /ScrimRoundSummaries/item/{id} | 
[**scrim_round_summaries_list**](ScrimRoundSummariesApi.md#scrim_round_summaries_list) | **GET** /ScrimRoundSummaries/list | 
[**scrim_round_summaries_variable_distribution**](ScrimRoundSummariesApi.md#scrim_round_summaries_variable_distribution) | **GET** /ScrimRoundSummaries/operations/variable-distribution | 



## scrim_round_summaries_aggregate

> Vec<models::ScrimRoundSummariesAggregate> scrim_round_summaries_aggregate(groupby, ordering, additional_filters, agent, aggregates, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game, having, index_on, map, metrics, patch, plant_time, player, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, summary, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**index_on** | Option<**String**> | Index results on a column |  |
**map** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**player** | Option<**String**> |  |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**summary** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |

### Return type

[**Vec<models::ScrimRoundSummariesAggregate>**](ScrimRoundSummariesAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_round_summaries_field_values

> Vec<String> scrim_round_summaries_field_values(field, ordering, additional_filters, agent, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game, map, patch, plant_time, player, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, summary, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**player** | Option<**String**> |  |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**summary** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_round_summaries_get

> models::ScrimRoundSummaries scrim_round_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScrimRoundSummaries**](ScrimRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_round_summaries_list

> Vec<models::ScrimRoundSummaries> scrim_round_summaries_list(additional_filters, agent, annotations, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game, map, only_values, ordering, patch, plant_time, player, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, summary, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**player** | Option<**String**> |  |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**summary** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |

### Return type

[**Vec<models::ScrimRoundSummaries>**](ScrimRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_round_summaries_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> scrim_round_summaries_variable_distribution(bucket_size, max, metric, min, additional_filters, agent, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game, map, patch, plant_time, player, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, summary, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**player** | Option<**String**> |  |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**summary** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

