# \ListApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**agent_list**](ListApi.md#agent_list) | **GET** /Agent/list | 
[**client_organization_list**](ListApi.md#client_organization_list) | **GET** /ClientOrganization/list | 
[**competitive_draft_slots_list**](ListApi.md#competitive_draft_slots_list) | **GET** /CompetitiveDraftSlots/list | 
[**competitive_game_summaries_list**](ListApi.md#competitive_game_summaries_list) | **GET** /CompetitiveGameSummaries/list | 
[**competitive_games_list**](ListApi.md#competitive_games_list) | **GET** /CompetitiveGames/list | 
[**competitive_round_summaries_list**](ListApi.md#competitive_round_summaries_list) | **GET** /CompetitiveRoundSummaries/list | 
[**competitive_team_round_summaries_list**](ListApi.md#competitive_team_round_summaries_list) | **GET** /CompetitiveTeamRoundSummaries/list | 
[**competitive_tiers_list**](ListApi.md#competitive_tiers_list) | **GET** /CompetitiveTiers/list | 
[**game_metrics_list**](ListApi.md#game_metrics_list) | **GET** /GameMetrics/list | 
[**gear_list**](ListApi.md#gear_list) | **GET** /Gear/list | 
[**league_list**](ListApi.md#league_list) | **GET** /League/list | 
[**maps_list**](ListApi.md#maps_list) | **GET** /Maps/list | 
[**player_list**](ListApi.md#player_list) | **GET** /Player/list | 
[**scrim_game_summaries_list**](ListApi.md#scrim_game_summaries_list) | **GET** /ScrimGameSummaries/list | 
[**scrim_games_list**](ListApi.md#scrim_games_list) | **GET** /ScrimGames/list | 
[**scrim_round_summaries_list**](ListApi.md#scrim_round_summaries_list) | **GET** /ScrimRoundSummaries/list | 
[**scrim_team_round_summaries_list**](ListApi.md#scrim_team_round_summaries_list) | **GET** /ScrimTeamRoundSummaries/list | 
[**seasons_list**](ListApi.md#seasons_list) | **GET** /Seasons/list | 
[**solo_q_accounts_list**](ListApi.md#solo_q_accounts_list) | **GET** /SoloQAccounts/list | 
[**solo_q_game_summaries_list**](ListApi.md#solo_q_game_summaries_list) | **GET** /SoloQGameSummaries/list | 
[**solo_q_games_list**](ListApi.md#solo_q_games_list) | **GET** /SoloQGames/list | 
[**solo_q_round_summaries_list**](ListApi.md#solo_q_round_summaries_list) | **GET** /SoloQRoundSummaries/list | 
[**solo_q_team_round_summaries_list**](ListApi.md#solo_q_team_round_summaries_list) | **GET** /SoloQTeamRoundSummaries/list | 
[**team_list**](ListApi.md#team_list) | **GET** /Team/list | 
[**user_list**](ListApi.md#user_list) | **GET** /User/list | 
[**weapons_list**](ListApi.md#weapons_list) | **GET** /Weapons/list | 



## agent_list

> Vec<models::Agent> agent_list(additional_filters, annotations, id, only_values, ordering, role, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**id** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**role** | Option<**String**> |  |  |
**uuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::Agent>**](Agent.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_organization_list

> Vec<models::ClientOrganization> client_organization_list(additional_filters, annotations, only_values, ordering)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |

### Return type

[**Vec<models::ClientOrganization>**](ClientOrganization.md)

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


## competitive_game_summaries_list

> Vec<models::CompetitiveGameSummaries> competitive_game_summaries_list(additional_filters, agent, annotations, date, id, league, map, only_values, ordering, patch, player, team1, team2, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**date** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**patch** | Option<**String**> |  |  |
**player** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::CompetitiveGameSummaries>**](CompetitiveGameSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_games_list

> Vec<models::CompetitiveGames> competitive_games_list(additional_filters, annotations, game_id, id, league, map, map_selection, only_values, ordering, phase, series_id, smokes_yolonet_version, split, team1, team2, utils_yolonet_version, winner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**game_id** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**map_selection** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**phase** | Option<**String**> |  |  |
**series_id** | Option<**i32**> |  |  |
**smokes_yolonet_version** | Option<**String**> |  |  |
**split** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**utils_yolonet_version** | Option<**String**> |  |  |
**winner** | Option<**String**> |  |  |

### Return type

[**Vec<models::CompetitiveGames>**](CompetitiveGames.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_round_summaries_list

> Vec<models::CompetitiveRoundSummaries> competitive_round_summaries_list(additional_filters, agent, annotations, attack_pattern_full, attack_pattern_short, bomb_site, callout_15s, date, defense_pattern_full, defense_pattern_short, first_attack_site, freeze_time_end_timestamp, game, league, map, only_values, ordering, patch, plant_time, player, round_eco_type, round_eco_type_no_bonus, round_id, round_index, shield, side, start_time_seconds, summary, team1, team2)


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
**league** | Option<**String**> |  |  |
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

[**Vec<models::CompetitiveRoundSummaries>**](CompetitiveRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_team_round_summaries_list

> Vec<models::CompetitiveTeamRoundSummaries> competitive_team_round_summaries_list(additional_filters, annotations, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, league, map, only_values, ordering, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side, team1, team2)


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
**league** | Option<**String**> |  |  |
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

[**Vec<models::CompetitiveTeamRoundSummaries>**](CompetitiveTeamRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_tiers_list

> Vec<models::CompetitiveTiers> competitive_tiers_list(additional_filters, annotations, division, id, only_values, ordering, tier)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**division** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**tier** | Option<**i32**> |  |  |

### Return type

[**Vec<models::CompetitiveTiers>**](CompetitiveTiers.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_metrics_list

> Vec<models::GameMetrics> game_metrics_list(additional_filters, annotations, label, only_values, ordering)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**label** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |

### Return type

[**Vec<models::GameMetrics>**](GameMetrics.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gear_list

> Vec<models::Gear> gear_list(additional_filters, annotations, id, only_values, ordering, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**id** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**uuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::Gear>**](Gear.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## league_list

> Vec<models::League> league_list(additional_filters, annotations, id, league_group, only_values, ordering)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**id** | Option<**String**> |  |  |
**league_group** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |

### Return type

[**Vec<models::League>**](League.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_list

> Vec<models::Maps> maps_list(additional_filters, annotations, id, only_values, ordering, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**id** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**uuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::Maps>**](Maps.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_list

> Vec<models::Player> player_list(additional_filters, annotations, birthdate, contract_expires, current_team, domestic_league, grid_id, id, is_retired, last_team, nationality, only_values, ordering, previous_names, role, soloq_tracked, vlr_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**birthdate** | Option<**String**> |  |  |
**contract_expires** | Option<**String**> |  |  |
**current_team** | Option<**String**> |  |  |
**domestic_league** | Option<**String**> |  |  |
**grid_id** | Option<**i32**> |  |  |
**id** | Option<**String**> |  |  |
**is_retired** | Option<**bool**> |  |  |
**last_team** | Option<**String**> |  |  |
**nationality** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**previous_names** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**soloq_tracked** | Option<**bool**> |  |  |
**vlr_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::Player>**](Player.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_game_summaries_list

> Vec<models::ScrimGameSummaries> scrim_game_summaries_list(additional_filters, agent, annotations, date, id, map, only_values, ordering, patch, player, team1, team2, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**date** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**patch** | Option<**String**> |  |  |
**player** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ScrimGameSummaries>**](ScrimGameSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_games_list

> Vec<models::ScrimGames> scrim_games_list(additional_filters, annotations, id, map, only_values, ordering, team1, team2, winner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**winner** | Option<**String**> |  |  |

### Return type

[**Vec<models::ScrimGames>**](ScrimGames.md)

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


## seasons_list

> Vec<models::Seasons> seasons_list(additional_filters, annotations, display_name, id, is_active, only_values, ordering, parent_uuid, season_type, start_time)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**display_name** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**is_active** | Option<**bool**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**parent_uuid** | Option<**String**> |  |  |
**season_type** | Option<**String**> |  |  |
**start_time** | Option<**String**> |  |  |

### Return type

[**Vec<models::Seasons>**](Seasons.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_accounts_list

> Vec<models::SoloQAccounts> solo_q_accounts_list(additional_filters, annotations, last_updated, only_values, ordering, player, puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**last_updated** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**player** | Option<**String**> |  |  |
**puuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::SoloQAccounts>**](SoloQAccounts.md)

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


## solo_q_games_list

> Vec<models::SoloQGames> solo_q_games_list(additional_filters, annotations, date, id, map, only_values, ordering)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**date** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |

### Return type

[**Vec<models::SoloQGames>**](SoloQGames.md)

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


## solo_q_team_round_summaries_list

> Vec<models::SoloQTeamRoundSummaries> solo_q_team_round_summaries_list(additional_filters, annotations, assists, attack_pattern_full, attack_pattern_short, bomb_site, combat_score, damages, date, deaths, defense_pattern_full, defense_pattern_short, first_blood, first_death, first_true_blood, first_true_death, game, id, kast, kills, map, only_values, ordering, round_eco_type, round_eco_type_no_bonus, round_end, round_index, side)


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

### Return type

[**Vec<models::SoloQTeamRoundSummaries>**](SoloQTeamRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_list

> Vec<models::Team> team_list(additional_filters, annotations, coaching_staff, current_league, current_main_players, domestic_league, grid_id, id, name, next_opponent, only_values, ordering, players_list, renamed_to, vlr_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**coaching_staff** | Option<**String**> |  |  |
**current_league** | Option<**String**> |  |  |
**current_main_players** | Option<**String**> |  |  |
**domestic_league** | Option<**String**> |  |  |
**grid_id** | Option<**i32**> |  |  |
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**next_opponent** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**players_list** | Option<**String**> |  |  |
**renamed_to** | Option<**String**> |  |  |
**vlr_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list

> Vec<models::User> user_list(additional_filters, annotations, only_values, ordering, organization, team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**organization** | Option<**String**> |  |  |
**team** | Option<**String**> |  |  |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weapons_list

> Vec<models::Weapons> weapons_list(additional_filters, annotations, category, id, only_values, ordering, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**category** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**uuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::Weapons>**](Weapons.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

