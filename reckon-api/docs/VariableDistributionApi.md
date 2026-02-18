# \VariableDistributionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**client_organization_variable_distribution**](VariableDistributionApi.md#client_organization_variable_distribution) | **GET** /ClientOrganization/operations/variable-distribution | 
[**competitive_draft_slots_variable_distribution**](VariableDistributionApi.md#competitive_draft_slots_variable_distribution) | **GET** /CompetitiveDraftSlots/operations/variable-distribution | 
[**competitive_game_summaries_variable_distribution**](VariableDistributionApi.md#competitive_game_summaries_variable_distribution) | **GET** /CompetitiveGameSummaries/operations/variable-distribution | 
[**competitive_games_variable_distribution**](VariableDistributionApi.md#competitive_games_variable_distribution) | **GET** /CompetitiveGames/operations/variable-distribution | 
[**competitive_round_summaries_variable_distribution**](VariableDistributionApi.md#competitive_round_summaries_variable_distribution) | **GET** /CompetitiveRoundSummaries/operations/variable-distribution | 
[**competitive_team_round_summaries_variable_distribution**](VariableDistributionApi.md#competitive_team_round_summaries_variable_distribution) | **GET** /CompetitiveTeamRoundSummaries/operations/variable-distribution | 
[**game_metrics_variable_distribution**](VariableDistributionApi.md#game_metrics_variable_distribution) | **GET** /GameMetrics/operations/variable-distribution | 
[**league_variable_distribution**](VariableDistributionApi.md#league_variable_distribution) | **GET** /League/operations/variable-distribution | 
[**maps_variable_distribution**](VariableDistributionApi.md#maps_variable_distribution) | **GET** /Maps/operations/variable-distribution | 
[**player_variable_distribution**](VariableDistributionApi.md#player_variable_distribution) | **GET** /Player/operations/variable-distribution | 
[**scrim_game_summaries_variable_distribution**](VariableDistributionApi.md#scrim_game_summaries_variable_distribution) | **GET** /ScrimGameSummaries/operations/variable-distribution | 
[**scrim_games_variable_distribution**](VariableDistributionApi.md#scrim_games_variable_distribution) | **GET** /ScrimGames/operations/variable-distribution | 
[**scrim_round_summaries_variable_distribution**](VariableDistributionApi.md#scrim_round_summaries_variable_distribution) | **GET** /ScrimRoundSummaries/operations/variable-distribution | 
[**scrim_team_round_summaries_variable_distribution**](VariableDistributionApi.md#scrim_team_round_summaries_variable_distribution) | **GET** /ScrimTeamRoundSummaries/operations/variable-distribution | 
[**solo_q_accounts_variable_distribution**](VariableDistributionApi.md#solo_q_accounts_variable_distribution) | **GET** /SoloQAccounts/operations/variable-distribution | 
[**team_variable_distribution**](VariableDistributionApi.md#team_variable_distribution) | **GET** /Team/operations/variable-distribution | 
[**user_variable_distribution**](VariableDistributionApi.md#user_variable_distribution) | **GET** /User/operations/variable-distribution | 



## client_organization_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> client_organization_variable_distribution(bucket_size, max, metric, min, additional_filters)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

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


## competitive_game_summaries_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> competitive_game_summaries_variable_distribution(bucket_size, max, metric, min, additional_filters, agent, date, id, league, map, patch, player, team1, team2, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**player** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_games_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> competitive_games_variable_distribution(bucket_size, max, metric, min, additional_filters, game_id, id, league, map, map_selection, phase, series_id, smokes_yolonet_version, split, team1, team2, utils_yolonet_version, winner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**game_id** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**map_selection** | Option<**String**> |  |  |
**phase** | Option<**String**> |  |  |
**series_id** | Option<**i32**> |  |  |
**smokes_yolonet_version** | Option<**String**> |  |  |
**split** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**utils_yolonet_version** | Option<**String**> |  |  |
**winner** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_round_summaries_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> competitive_round_summaries_variable_distribution(bucket_size, max, metric, min, additional_filters, agent, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game, league, map, patch, plant_time, player, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, summary, team1, team2)


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
**league** | Option<**String**> |  |  |
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


## competitive_team_round_summaries_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> competitive_team_round_summaries_variable_distribution(bucket_size, max, metric, min, additional_filters, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, league, map, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side, team1, team2)


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
**league** | Option<**String**> |  |  |
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


## game_metrics_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> game_metrics_variable_distribution(bucket_size, max, metric, min, additional_filters, label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**label** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## league_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> league_variable_distribution(bucket_size, max, metric, min, additional_filters, id, league_group)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**league_group** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> maps_variable_distribution(bucket_size, max, metric, min, additional_filters, id, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**uuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> player_variable_distribution(bucket_size, max, metric, min, additional_filters, birthdate, contract_expires, current_team, domestic_league, grid_id, id, is_retired, last_team, nationality, previous_names, role, soloq_tracked)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**birthdate** | Option<**String**> |  |  |
**contract_expires** | Option<**String**> |  |  |
**current_team** | Option<**String**> |  |  |
**domestic_league** | Option<**String**> |  |  |
**grid_id** | Option<**i32**> |  |  |
**id** | Option<**String**> |  |  |
**is_retired** | Option<**bool**> |  |  |
**last_team** | Option<**String**> |  |  |
**nationality** | Option<**String**> |  |  |
**previous_names** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**soloq_tracked** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_game_summaries_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> scrim_game_summaries_variable_distribution(bucket_size, max, metric, min, additional_filters, agent, date, id, map, patch, player, team1, team2, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**date** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**patch** | Option<**String**> |  |  |
**player** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_games_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> scrim_games_variable_distribution(bucket_size, max, metric, min, additional_filters, id, map, team1, team2, winner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**winner** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

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


## solo_q_accounts_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> solo_q_accounts_variable_distribution(bucket_size, max, metric, min, additional_filters, puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**puuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> team_variable_distribution(bucket_size, max, metric, min, additional_filters, coaching_staff, current_league, current_main_players, domestic_league, grid_id, id, name, next_opponent, players_list, renamed_to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**coaching_staff** | Option<**String**> |  |  |
**current_league** | Option<**String**> |  |  |
**current_main_players** | Option<**String**> |  |  |
**domestic_league** | Option<**String**> |  |  |
**grid_id** | Option<**i32**> |  |  |
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**next_opponent** | Option<**String**> |  |  |
**players_list** | Option<**String**> |  |  |
**renamed_to** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> user_variable_distribution(bucket_size, max, metric, min, additional_filters, organization, team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**organization** | Option<**String**> |  |  |
**team** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

