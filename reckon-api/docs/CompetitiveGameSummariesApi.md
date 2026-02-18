# \CompetitiveGameSummariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**competitive_game_summaries_aggregate**](CompetitiveGameSummariesApi.md#competitive_game_summaries_aggregate) | **GET** /CompetitiveGameSummaries/operations/aggregate | 
[**competitive_game_summaries_field_values**](CompetitiveGameSummariesApi.md#competitive_game_summaries_field_values) | **GET** /CompetitiveGameSummaries/operations/field-values | 
[**competitive_game_summaries_get**](CompetitiveGameSummariesApi.md#competitive_game_summaries_get) | **GET** /CompetitiveGameSummaries/item/{id} | 
[**competitive_game_summaries_list**](CompetitiveGameSummariesApi.md#competitive_game_summaries_list) | **GET** /CompetitiveGameSummaries/list | 
[**competitive_game_summaries_variable_distribution**](CompetitiveGameSummariesApi.md#competitive_game_summaries_variable_distribution) | **GET** /CompetitiveGameSummaries/operations/variable-distribution | 



## competitive_game_summaries_aggregate

> Vec<models::CompetitiveGameSummariesAggregate> competitive_game_summaries_aggregate(groupby, ordering, additional_filters, agent, aggregates, date, having, id, index_on, league, map, metrics, patch, player, team1, team2, win)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**agent** | Option<**String**> |  |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**date** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**league** | Option<**String**> |  |  |
**map** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**patch** | Option<**String**> |  |  |
**player** | Option<**String**> |  |  |
**team1** | Option<**String**> |  |  |
**team2** | Option<**String**> |  |  |
**win** | Option<**bool**> |  |  |

### Return type

[**Vec<models::CompetitiveGameSummariesAggregate>**](CompetitiveGameSummariesAggregate.md)

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

