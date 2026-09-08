# \SeasonsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**seasons_aggregate**](SeasonsApi.md#seasons_aggregate) | **GET** /Seasons/operations/aggregate | 
[**seasons_field_values**](SeasonsApi.md#seasons_field_values) | **GET** /Seasons/operations/field-values | 
[**seasons_get**](SeasonsApi.md#seasons_get) | **GET** /Seasons/item/{id} | 
[**seasons_list**](SeasonsApi.md#seasons_list) | **GET** /Seasons/list | 
[**seasons_variable_distribution**](SeasonsApi.md#seasons_variable_distribution) | **GET** /Seasons/operations/variable-distribution | 



## seasons_aggregate

> Vec<models::SeasonsAggregate> seasons_aggregate(groupby, ordering, additional_filters, aggregates, display_name, having, id, index_on, is_active, metrics, parent_uuid, season_type, start_time)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**display_name** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**is_active** | Option<**bool**> |  |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**parent_uuid** | Option<**String**> |  |  |
**season_type** | Option<**String**> |  |  |
**start_time** | Option<**String**> |  |  |

### Return type

[**Vec<models::SeasonsAggregate>**](SeasonsAggregate.md)

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


## seasons_get

> models::Seasons seasons_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Seasons**](Seasons.md)

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


## seasons_variable_distribution

> Vec<models::AgentVariableDistribution200ResponseInner> seasons_variable_distribution(bucket_size, max, metric, min, additional_filters, display_name, id, is_active, parent_uuid, season_type, start_time)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**display_name** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**is_active** | Option<**bool**> |  |  |
**parent_uuid** | Option<**String**> |  |  |
**season_type** | Option<**String**> |  |  |
**start_time** | Option<**String**> |  |  |

### Return type

[**Vec<models::AgentVariableDistribution200ResponseInner>**](Agent_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

