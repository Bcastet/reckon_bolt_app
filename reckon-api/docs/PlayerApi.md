# \PlayerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_player_account**](PlayerApi.md#add_player_account) | **POST** /Player/item/{id}/add_account | 
[**player_aggregate**](PlayerApi.md#player_aggregate) | **GET** /Player/operations/aggregate | 
[**player_create**](PlayerApi.md#player_create) | **POST** /Player/item/{id} | 
[**player_field_values**](PlayerApi.md#player_field_values) | **GET** /Player/operations/field-values | 
[**player_get**](PlayerApi.md#player_get) | **GET** /Player/item/{id} | 
[**player_list**](PlayerApi.md#player_list) | **GET** /Player/list | 
[**player_patch**](PlayerApi.md#player_patch) | **PATCH** /Player/item/{id} | 
[**player_variable_distribution**](PlayerApi.md#player_variable_distribution) | **GET** /Player/operations/variable-distribution | 



## add_player_account

> Vec<String> add_player_account(id, add_account)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**add_account** | [**AddAccount**](AddAccount.md) |  | [required] |

### Return type

**Vec<String>**

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_aggregate

> Vec<models::PlayerAggregate> player_aggregate(groupby, ordering, additional_filters, aggregates, birthdate, contract_expires, current_team, domestic_league, grid_id, having, id, index_on, is_retired, last_team, metrics, nationality, previous_names, role, soloq_tracked)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**birthdate** | Option<**String**> |  |  |
**contract_expires** | Option<**String**> |  |  |
**current_team** | Option<**String**> |  |  |
**domestic_league** | Option<**String**> |  |  |
**grid_id** | Option<**i32**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**is_retired** | Option<**bool**> |  |  |
**last_team** | Option<**String**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**nationality** | Option<**String**> |  |  |
**previous_names** | Option<**String**> |  |  |
**role** | Option<**String**> |  |  |
**soloq_tracked** | Option<**bool**> |  |  |

### Return type

[**Vec<models::PlayerAggregate>**](PlayerAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_create

> models::Player player_create(id, player)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**player** | [**Player**](Player.md) |  | [required] |

### Return type

[**models::Player**](Player.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_field_values

> Vec<String> player_field_values(field, ordering, additional_filters, birthdate, contract_expires, current_team, domestic_league, grid_id, id, is_retired, last_team, nationality, previous_names, role, soloq_tracked)


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

### Return type

**Vec<String>**

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


## player_list

> Vec<models::Player> player_list(additional_filters, annotations, birthdate, contract_expires, current_team, domestic_league, grid_id, id, is_retired, last_team, nationality, only_values, ordering, previous_names, role, soloq_tracked)


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

### Return type

[**Vec<models::Player>**](Player.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## player_patch

> models::Player player_patch(id, patched_player)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_player** | Option<[**PatchedPlayer**](PatchedPlayer.md)> |  |  |

### Return type

[**models::Player**](Player.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
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

