# \SoloQGameSummariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**solo_q_game_summaries_aggregate**](SoloQGameSummariesApi.md#solo_q_game_summaries_aggregate) | **GET** /SoloQGameSummaries/operations/aggregate | 
[**solo_q_game_summaries_field_values**](SoloQGameSummariesApi.md#solo_q_game_summaries_field_values) | **GET** /SoloQGameSummaries/operations/field-values | 
[**solo_q_game_summaries_get**](SoloQGameSummariesApi.md#solo_q_game_summaries_get) | **GET** /SoloQGameSummaries/item/{id} | 
[**solo_q_game_summaries_list**](SoloQGameSummariesApi.md#solo_q_game_summaries_list) | **GET** /SoloQGameSummaries/list | 
[**solo_q_game_summaries_variable_distribution**](SoloQGameSummariesApi.md#solo_q_game_summaries_variable_distribution) | **GET** /SoloQGameSummaries/operations/variable-distribution | 



## solo_q_game_summaries_aggregate

> Vec<models::SoloQGameSummariesAggregate> solo_q_game_summaries_aggregate(groupby, ordering, account, additional_filters, agent, aggregates, competitive_player, date, game_id, having, ig_name, index_on, map, metrics, patch, puuid, rank_tier, server, team, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**account** | Option<**i32**> |  |  |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**game_id** | Option<**uuid::Uuid**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**ig_name** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**map** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**patch** | Option<**String**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**server** | Option<**String**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::SoloQGameSummariesAggregate>**](SoloQGameSummariesAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_game_summaries_field_values

> Vec<String> solo_q_game_summaries_field_values(field, ordering, account, additional_filters, agent, competitive_player, date, game_id, ig_name, map, patch, puuid, rank_tier, server, team, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**account** | Option<**i32**> |  |  |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**game_id** | Option<**uuid::Uuid**> |  |  |
**ig_name** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**server** | Option<**String**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |
**win** | Option<**bool**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_game_summaries_get

> models::SoloQGameSummaries solo_q_game_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SoloQGameSummaries**](SoloQGameSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_game_summaries_list

> Vec<models::SoloQGameSummaries> solo_q_game_summaries_list(account, additional_filters, agent, annotations, competitive_player, date, game_id, ig_name, map, only_values, ordering, patch, puuid, rank_tier, server, team, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account** | Option<**i32**> |  |  |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**game_id** | Option<**uuid::Uuid**> |  |  |
**ig_name** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**patch** | Option<**String**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**server** | Option<**String**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::SoloQGameSummaries>**](SoloQGameSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_game_summaries_variable_distribution

> Vec<models::AgentVariableDistribution200ResponseInner> solo_q_game_summaries_variable_distribution(bucket_size, max, metric, min, account, additional_filters, agent, competitive_player, date, game_id, ig_name, map, patch, puuid, rank_tier, server, team, win)


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
**competitive_player** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**game_id** | Option<**uuid::Uuid**> |  |  |
**ig_name** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**puuid** | Option<**String**> |  |  |
**rank_tier** | Option<**i32**> | * `0` - Unranked0 * `1` - Unranked1 * `2` - Unranked2 * `3` - Iron 1 * `4` - Iron 2 * `5` - Iron 3 * `6` - Bronze 1 * `7` - Bronze 2 * `8` - Bronze 3 * `9` - Silver 1 * `10` - Silver 2 * `11` - Silver 3 * `12` - Gold 1 * `13` - Gold 2 * `14` - Gold 3 * `15` - Plat 1 * `16` - Plat 2 * `17` - Plat 3 * `18` - Diamond 1 * `19` - Diamond 2 * `20` - Diamond 3 * `21` - Ascendant 1 * `22` - Ascendant 2 * `23` - Ascendant 3 * `24` - Immortal 1 * `25` - Immortal 2 * `26` - Immortal 3 * `27` - Radiant |  |
**server** | Option<**String**> |  |  |
**team** | Option<**i32**> | * `1` - 1 * `2` - 2 |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::AgentVariableDistribution200ResponseInner>**](Agent_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

