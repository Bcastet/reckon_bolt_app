# \TeamApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**team_aggregate**](TeamApi.md#team_aggregate) | **GET** /Team/operations/aggregate | 
[**team_field_values**](TeamApi.md#team_field_values) | **GET** /Team/operations/field-values | 
[**team_get**](TeamApi.md#team_get) | **GET** /Team/item/{id} | 
[**team_list**](TeamApi.md#team_list) | **GET** /Team/list | 
[**team_patch**](TeamApi.md#team_patch) | **PATCH** /Team/item/{id} | 
[**team_variable_distribution**](TeamApi.md#team_variable_distribution) | **GET** /Team/operations/variable-distribution | 



## team_aggregate

> Vec<models::TeamAggregate> team_aggregate(groupby, ordering, additional_filters, aggregates, coaching_staff, current_league, current_main_players, domestic_league, grid_id, having, id, index_on, metrics, name, next_opponent, players_list, renamed_to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**coaching_staff** | Option<**String**> |  |  |
**current_league** | Option<**String**> |  |  |
**current_main_players** | Option<**String**> |  |  |
**domestic_league** | Option<**String**> |  |  |
**grid_id** | Option<**i32**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**name** | Option<**String**> |  |  |
**next_opponent** | Option<**String**> |  |  |
**players_list** | Option<**String**> |  |  |
**renamed_to** | Option<**String**> |  |  |

### Return type

[**Vec<models::TeamAggregate>**](TeamAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_field_values

> Vec<String> team_field_values(field, ordering, additional_filters, coaching_staff, current_league, current_main_players, domestic_league, grid_id, id, name, next_opponent, players_list, renamed_to)


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

### Return type

**Vec<String>**

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


## team_list

> Vec<models::Team> team_list(additional_filters, annotations, coaching_staff, current_league, current_main_players, domestic_league, grid_id, id, name, next_opponent, only_values, ordering, players_list, renamed_to)


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

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_patch

> models::Team team_patch(id, patched_team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_team** | Option<[**PatchedTeam**](PatchedTeam.md)> |  |  |

### Return type

[**models::Team**](Team.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
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

