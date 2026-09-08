# \SoloQRoundSummariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**solo_q_round_summaries_aggregate**](SoloQRoundSummariesApi.md#solo_q_round_summaries_aggregate) | **GET** /SoloQRoundSummaries/operations/aggregate | 
[**solo_q_round_summaries_field_values**](SoloQRoundSummariesApi.md#solo_q_round_summaries_field_values) | **GET** /SoloQRoundSummaries/operations/field-values | 
[**solo_q_round_summaries_get**](SoloQRoundSummariesApi.md#solo_q_round_summaries_get) | **GET** /SoloQRoundSummaries/item/{id} | 
[**solo_q_round_summaries_list**](SoloQRoundSummariesApi.md#solo_q_round_summaries_list) | **GET** /SoloQRoundSummaries/list | 
[**solo_q_round_summaries_variable_distribution**](SoloQRoundSummariesApi.md#solo_q_round_summaries_variable_distribution) | **GET** /SoloQRoundSummaries/operations/variable-distribution | 



## solo_q_round_summaries_aggregate

> Vec<models::SoloQRoundSummariesAggregate> solo_q_round_summaries_aggregate(groupby, ordering, account, additional_filters, agent, aggregates, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, competitive_player, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game_id, having, index_on, map, metrics, patch, plant_time, puuid, rank_tier, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**account** | Option<**i32**> |  |  |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game_id** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**index_on** | Option<**String**> | Index results on a column |  |
**map** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |

### Return type

[**Vec<models::SoloQRoundSummariesAggregate>**](SoloQRoundSummariesAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_round_summaries_field_values

> Vec<String> solo_q_round_summaries_field_values(field, ordering, account, additional_filters, agent, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, competitive_player, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game_id, map, patch, plant_time, puuid, rank_tier, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**account** | Option<**i32**> |  |  |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game_id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_round_summaries_get

> models::SoloQRoundSummaries solo_q_round_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SoloQRoundSummaries**](SoloQRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_round_summaries_list

> Vec<models::SoloQRoundSummaries> solo_q_round_summaries_list(account, additional_filters, agent, annotations, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, competitive_player, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game_id, map, only_values, ordering, patch, plant_time, puuid, rank_tier, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account** | Option<**i32**> |  |  |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game_id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |

### Return type

[**Vec<models::SoloQRoundSummaries>**](SoloQRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_round_summaries_variable_distribution

> Vec<models::AgentVariableDistribution200ResponseInner> solo_q_round_summaries_variable_distribution(bucket_size, max, metric, min, account, additional_filters, agent, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, competitive_player, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game_id, map, patch, plant_time, puuid, rank_tier, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**account** | Option<**i32**> |  |  |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**callout_15s** | Option<**String**> |  |  |
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_attack_site** | Option<**String**> |  |  |
**freeze_time_end_timestamp** | Option<**i32**> |  |  |
**game_id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**plant_time** | Option<**i32**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_id** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**shield** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**start_time_seconds** | Option<**i32**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |

### Return type

[**Vec<models::AgentVariableDistribution200ResponseInner>**](Agent_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

