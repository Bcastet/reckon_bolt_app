# \FieldValuesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**agent_field_values**](FieldValuesApi.md#agent_field_values) | **GET** /Agent/operations/field-values | 
[**client_organization_field_values**](FieldValuesApi.md#client_organization_field_values) | **GET** /ClientOrganization/operations/field-values | 
[**competitive_draft_slots_field_values**](FieldValuesApi.md#competitive_draft_slots_field_values) | **GET** /CompetitiveDraftSlots/operations/field-values | 
[**competitive_game_summaries_field_values**](FieldValuesApi.md#competitive_game_summaries_field_values) | **GET** /CompetitiveGameSummaries/operations/field-values | 
[**competitive_games_field_values**](FieldValuesApi.md#competitive_games_field_values) | **GET** /CompetitiveGames/operations/field-values | 
[**competitive_round_summaries_field_values**](FieldValuesApi.md#competitive_round_summaries_field_values) | **GET** /CompetitiveRoundSummaries/operations/field-values | 
[**competitive_team_round_summaries_field_values**](FieldValuesApi.md#competitive_team_round_summaries_field_values) | **GET** /CompetitiveTeamRoundSummaries/operations/field-values | 
[**competitive_tiers_field_values**](FieldValuesApi.md#competitive_tiers_field_values) | **GET** /CompetitiveTiers/operations/field-values | 
[**game_metrics_field_values**](FieldValuesApi.md#game_metrics_field_values) | **GET** /GameMetrics/operations/field-values | 
[**gear_field_values**](FieldValuesApi.md#gear_field_values) | **GET** /Gear/operations/field-values | 
[**league_field_values**](FieldValuesApi.md#league_field_values) | **GET** /League/operations/field-values | 
[**maps_field_values**](FieldValuesApi.md#maps_field_values) | **GET** /Maps/operations/field-values | 
[**player_field_values**](FieldValuesApi.md#player_field_values) | **GET** /Player/operations/field-values | 
[**scrim_game_summaries_field_values**](FieldValuesApi.md#scrim_game_summaries_field_values) | **GET** /ScrimGameSummaries/operations/field-values | 
[**scrim_games_field_values**](FieldValuesApi.md#scrim_games_field_values) | **GET** /ScrimGames/operations/field-values | 
[**scrim_round_summaries_field_values**](FieldValuesApi.md#scrim_round_summaries_field_values) | **GET** /ScrimRoundSummaries/operations/field-values | 
[**scrim_team_round_summaries_field_values**](FieldValuesApi.md#scrim_team_round_summaries_field_values) | **GET** /ScrimTeamRoundSummaries/operations/field-values | 
[**seasons_field_values**](FieldValuesApi.md#seasons_field_values) | **GET** /Seasons/operations/field-values | 
[**solo_q_accounts_field_values**](FieldValuesApi.md#solo_q_accounts_field_values) | **GET** /SoloQAccounts/operations/field-values | 
[**solo_q_game_summaries_field_values**](FieldValuesApi.md#solo_q_game_summaries_field_values) | **GET** /SoloQGameSummaries/operations/field-values | 
[**solo_q_games_field_values**](FieldValuesApi.md#solo_q_games_field_values) | **GET** /SoloQGames/operations/field-values | 
[**solo_q_round_summaries_field_values**](FieldValuesApi.md#solo_q_round_summaries_field_values) | **GET** /SoloQRoundSummaries/operations/field-values | 
[**solo_q_team_round_summaries_field_values**](FieldValuesApi.md#solo_q_team_round_summaries_field_values) | **GET** /SoloQTeamRoundSummaries/operations/field-values | 
[**team_field_values**](FieldValuesApi.md#team_field_values) | **GET** /Team/operations/field-values | 
[**user_field_values**](FieldValuesApi.md#user_field_values) | **GET** /User/operations/field-values | 
[**weapons_field_values**](FieldValuesApi.md#weapons_field_values) | **GET** /Weapons/operations/field-values | 



## agent_field_values

> Vec<String> agent_field_values(field, ordering, additional_filters, id, role, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**uuid** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_organization_field_values

> Vec<String> client_organization_field_values(field, ordering, additional_filters)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |

### Return type

**Vec<String>**

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


## competitive_game_summaries_field_values

> Vec<String> competitive_game_summaries_field_values(field, ordering, additional_filters, agent, date, id, league, map, patch, player, team1, team2, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
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

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_games_field_values

> Vec<String> competitive_games_field_values(field, ordering, additional_filters, game_id, id, league, map, map_selection, phase, series_id, smokes_yolonet_version, split, team1, team2, utils_yolonet_version, winner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
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

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_round_summaries_field_values

> Vec<String> competitive_round_summaries_field_values(field, ordering, additional_filters, agent, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game, league, map, patch, plant_time, player, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, summary, team1, team2)


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

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_team_round_summaries_field_values

> Vec<String> competitive_team_round_summaries_field_values(field, ordering, additional_filters, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, league, map, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side, team1, team2)


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

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_tiers_field_values

> Vec<String> competitive_tiers_field_values(field, ordering, additional_filters, division, id, tier)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**division** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**tier** | Option<**i32**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_metrics_field_values

> Vec<String> game_metrics_field_values(field, ordering, additional_filters, label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**label** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gear_field_values

> Vec<String> gear_field_values(field, ordering, additional_filters, id, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**uuid** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## league_field_values

> Vec<String> league_field_values(field, ordering, additional_filters, id, league_group)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**league_group** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_field_values

> Vec<String> maps_field_values(field, ordering, additional_filters, id, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**uuid** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_field_values

> Vec<String> player_field_values(field, ordering, additional_filters, birthdate, contract_expires, current_team, domestic_league, grid_id, id, is_retired, last_team, nationality, previous_names, role, soloq_tracked, vlr_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
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
**vlr_id** | Option<**i32**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_game_summaries_field_values

> Vec<String> scrim_game_summaries_field_values(field, ordering, additional_filters, agent, date, id, map, patch, player, team1, team2, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
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

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_games_field_values

> Vec<String> scrim_games_field_values(field, ordering, additional_filters, id, map, team1, team2, winner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**winner** | Option<**String**> |  |  |

### Return type

**Vec<String>**

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


## seasons_field_values

> Vec<String> seasons_field_values(field, ordering, additional_filters, display_name, id, is_active, parent_uuid, season_type, start_time)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**display_name** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**is_active** | Option<**bool**> |  |  |
**parent_uuid** | Option<**String**> |  |  |
**season_type** | Option<**String**> |  |  |
**start_time** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_accounts_field_values

> Vec<String> solo_q_accounts_field_values(field, ordering, additional_filters, last_updated, player, puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**last_updated** | Option<**String**> |  |  |
**player** | Option<**String**> |  |  |
**puuid** | Option<**String**> |  |  |

### Return type

**Vec<String>**

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


## solo_q_games_field_values

> Vec<String> solo_q_games_field_values(field, ordering, additional_filters, date, id, map)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**date** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |

### Return type

**Vec<String>**

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


## solo_q_team_round_summaries_field_values

> Vec<String> solo_q_team_round_summaries_field_values(field, ordering, additional_filters, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, map, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side)


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

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_field_values

> Vec<String> team_field_values(field, ordering, additional_filters, coaching_staff, current_league, current_main_players, domestic_league, grid_id, id, name, next_opponent, players_list, renamed_to, vlr_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
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
**vlr_id** | Option<**i32**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_field_values

> Vec<String> user_field_values(field, ordering, additional_filters, organization, team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**organization** | Option<**String**> |  |  |
**team** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weapons_field_values

> Vec<String> weapons_field_values(field, ordering, additional_filters, category, id, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**category** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**uuid** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

