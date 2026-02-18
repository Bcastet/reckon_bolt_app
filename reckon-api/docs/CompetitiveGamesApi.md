# \CompetitiveGamesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**competitive_games_aggregate**](CompetitiveGamesApi.md#competitive_games_aggregate) | **GET** /CompetitiveGames/operations/aggregate | 
[**competitive_games_field_values**](CompetitiveGamesApi.md#competitive_games_field_values) | **GET** /CompetitiveGames/operations/field-values | 
[**competitive_games_get**](CompetitiveGamesApi.md#competitive_games_get) | **GET** /CompetitiveGames/item/{id} | 
[**competitive_games_list**](CompetitiveGamesApi.md#competitive_games_list) | **GET** /CompetitiveGames/list | 
[**competitive_games_variable_distribution**](CompetitiveGamesApi.md#competitive_games_variable_distribution) | **GET** /CompetitiveGames/operations/variable-distribution | 



## competitive_games_aggregate

> Vec<models::CompetitiveGamesAggregate> competitive_games_aggregate(groupby, ordering, additional_filters, aggregates, game_id, having, id, index_on, league, map, map_selection, metrics, phase, series_id, smokes_yolonet_version, split, team1, team2, utils_yolonet_version, winner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**game_id** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**map_selection** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**phase** | Option<**String**> |  |  |
**series_id** | Option<**i32**> |  |  |
**smokes_yolonet_version** | Option<**String**> |  |  |
**split** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**utils_yolonet_version** | Option<**String**> |  |  |
**winner** | Option<**String**> |  |  |

### Return type

[**Vec<models::CompetitiveGamesAggregate>**](CompetitiveGamesAggregate.md)

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

