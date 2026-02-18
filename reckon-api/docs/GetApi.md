# \GetApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**client_organization_get**](GetApi.md#client_organization_get) | **GET** /ClientOrganization/item/{id} | 
[**competitive_draft_slots_get**](GetApi.md#competitive_draft_slots_get) | **GET** /CompetitiveDraftSlots/item/{id} | 
[**competitive_game_summaries_get**](GetApi.md#competitive_game_summaries_get) | **GET** /CompetitiveGameSummaries/item/{id} | 
[**competitive_games_get**](GetApi.md#competitive_games_get) | **GET** /CompetitiveGames/item/{id} | 
[**competitive_round_summaries_get**](GetApi.md#competitive_round_summaries_get) | **GET** /CompetitiveRoundSummaries/item/{id} | 
[**competitive_team_round_summaries_get**](GetApi.md#competitive_team_round_summaries_get) | **GET** /CompetitiveTeamRoundSummaries/item/{id} | 
[**game_metrics_get**](GetApi.md#game_metrics_get) | **GET** /GameMetrics/item/{id} | 
[**league_get**](GetApi.md#league_get) | **GET** /League/item/{id} | 
[**login_create**](GetApi.md#login_create) | **POST** /Login | 
[**maps_get**](GetApi.md#maps_get) | **GET** /Maps/item/{id} | 
[**player_get**](GetApi.md#player_get) | **GET** /Player/item/{id} | 
[**scrim_game_summaries_get**](GetApi.md#scrim_game_summaries_get) | **GET** /ScrimGameSummaries/item/{id} | 
[**scrim_games_get**](GetApi.md#scrim_games_get) | **GET** /ScrimGames/item/{id} | 
[**scrim_round_summaries_get**](GetApi.md#scrim_round_summaries_get) | **GET** /ScrimRoundSummaries/item/{id} | 
[**scrim_team_round_summaries_get**](GetApi.md#scrim_team_round_summaries_get) | **GET** /ScrimTeamRoundSummaries/item/{id} | 
[**solo_q_accounts_get**](GetApi.md#solo_q_accounts_get) | **GET** /SoloQAccounts/item/{id} | 
[**team_get**](GetApi.md#team_get) | **GET** /Team/item/{id} | 
[**user_get**](GetApi.md#user_get) | **GET** /User/item/{id} | 



## client_organization_get

> models::ClientOrganization client_organization_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ClientOrganization**](ClientOrganization.md)

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


## competitive_game_summaries_get

> models::CompetitiveGameSummaries competitive_game_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CompetitiveGameSummaries**](CompetitiveGameSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_games_get

> models::CompetitiveGames competitive_games_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CompetitiveGames**](CompetitiveGames.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_round_summaries_get

> models::CompetitiveRoundSummaries competitive_round_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CompetitiveRoundSummaries**](CompetitiveRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## competitive_team_round_summaries_get

> models::CompetitiveTeamRoundSummaries competitive_team_round_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CompetitiveTeamRoundSummaries**](CompetitiveTeamRoundSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_metrics_get

> models::GameMetrics game_metrics_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GameMetrics**](GameMetrics.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## league_get

> models::League league_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::League**](League.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_create

> models::Login login_create(username, password, token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**password** | **String** |  | [required] |
**token** | **String** |  | [required] |

### Return type

[**models::Login**](Login.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_get

> models::Maps maps_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Maps**](Maps.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_get

> models::Player player_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Player**](Player.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_game_summaries_get

> models::ScrimGameSummaries scrim_game_summaries_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScrimGameSummaries**](ScrimGameSummaries.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scrim_games_get

> models::ScrimGames scrim_games_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScrimGames**](ScrimGames.md)

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


## solo_q_accounts_get

> models::SoloQAccounts solo_q_accounts_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SoloQAccounts**](SoloQAccounts.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_get

> models::Team team_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Team**](Team.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get

> models::User user_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

