# \ScrimTeamRoundSummariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scrim_team_round_summaries_aggregate**](ScrimTeamRoundSummariesApi.md#scrim_team_round_summaries_aggregate) | **GET** /ScrimTeamRoundSummaries/operations/aggregate | 
[**scrim_team_round_summaries_field_values**](ScrimTeamRoundSummariesApi.md#scrim_team_round_summaries_field_values) | **GET** /ScrimTeamRoundSummaries/operations/field-values | 
[**scrim_team_round_summaries_get**](ScrimTeamRoundSummariesApi.md#scrim_team_round_summaries_get) | **GET** /ScrimTeamRoundSummaries/item/{id} | 
[**scrim_team_round_summaries_list**](ScrimTeamRoundSummariesApi.md#scrim_team_round_summaries_list) | **GET** /ScrimTeamRoundSummaries/list | 
[**scrim_team_round_summaries_variable_distribution**](ScrimTeamRoundSummariesApi.md#scrim_team_round_summaries_variable_distribution) | **GET** /ScrimTeamRoundSummaries/operations/variable-distribution | 



## scrim_team_round_summaries_aggregate

> Vec<models::ScrimTeamRoundSummariesAggregate> scrim_team_round_summaries_aggregate(groupby, ordering, additional_filters, aggregates, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, having, id, index_on, kast, kills, map, metrics, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**assists** | Option<**i32**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**combat_score** | Option<**i32**> |  |  |
**damages** | Option<**i32**> |  |  |
**date** | Option<**String**> |  |  |
**deaths** | Option<**i32**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_blood** | Option<**bool**> |  |  |
**first_death** | Option<**bool**> |  |  |
**first_true_blood** | Option<**bool**> |  |  |
**first_true_death** | Option<**bool**> |  |  |
**game** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**kast** | Option<**i32**> |  |  |
**kills** | Option<**i32**> |  |  |
**map** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_end** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**side** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |

### Return type

[**Vec<models::ScrimTeamRoundSummariesAggregate>**](ScrimTeamRoundSummariesAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_team_round_summaries_field_values

> Vec<String> scrim_team_round_summaries_field_values(field, ordering, additional_filters, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, map, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**assists** | Option<**i32**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**combat_score** | Option<**i32**> |  |  |
**damages** | Option<**i32**> |  |  |
**date** | Option<**String**> |  |  |
**deaths** | Option<**i32**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_blood** | Option<**bool**> |  |  |
**first_death** | Option<**bool**> |  |  |
**first_true_blood** | Option<**bool**> |  |  |
**first_true_death** | Option<**bool**> |  |  |
**game** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**kast** | Option<**i32**> |  |  |
**kills** | Option<**i32**> |  |  |
**map** | Option<**String**> |  |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_end** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**side** | Option<**String**> |  |  |
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


## scrim_team_round_summaries_get

> models::ScrimTeamRoundSummaries scrim_team_round_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScrimTeamRoundSummaries**](ScrimTeamRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_team_round_summaries_list

> Vec<models::ScrimTeamRoundSummaries> scrim_team_round_summaries_list(additional_filters, annotations, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, map, only_values, ordering, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**assists** | Option<**i32**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**combat_score** | Option<**i32**> |  |  |
**damages** | Option<**i32**> |  |  |
**date** | Option<**String**> |  |  |
**deaths** | Option<**i32**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_blood** | Option<**bool**> |  |  |
**first_death** | Option<**bool**> |  |  |
**first_true_blood** | Option<**bool**> |  |  |
**first_true_death** | Option<**bool**> |  |  |
**game** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**kast** | Option<**i32**> |  |  |
**kills** | Option<**i32**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_end** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**side** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |

### Return type

[**Vec<models::ScrimTeamRoundSummaries>**](ScrimTeamRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_team_round_summaries_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> scrim_team_round_summaries_variable_distribution(bucket_size, max, metric, min, additional_filters, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, map, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side, team1, team2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**assists** | Option<**i32**> |  |  |
**attack_pattern_full** | Option<**String**> |  |  |
**attack_pattern_short** | Option<**String**> |  |  |
**bomb_site** | Option<**String**> |  |  |
**combat_score** | Option<**i32**> |  |  |
**damages** | Option<**i32**> |  |  |
**date** | Option<**String**> |  |  |
**deaths** | Option<**i32**> |  |  |
**defense_pattern_full** | Option<**String**> |  |  |
**defense_pattern_short** | Option<**String**> |  |  |
**first_blood** | Option<**bool**> |  |  |
**first_death** | Option<**bool**> |  |  |
**first_true_blood** | Option<**bool**> |  |  |
**first_true_death** | Option<**bool**> |  |  |
**game** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**kast** | Option<**i32**> |  |  |
**kills** | Option<**i32**> |  |  |
**map** | Option<**String**> |  |  |
**round_eco_type** | Option<**String**> |  |  |
**round_eco_type_no_bonus** | Option<**String**> |  |  |
**round_end** | Option<**String**> |  |  |
**round_index** | Option<**i32**> |  |  |
**side** | Option<**String**> |  |  |
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

