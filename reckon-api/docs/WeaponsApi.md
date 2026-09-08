# \WeaponsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**weapons_aggregate**](WeaponsApi.md#weapons_aggregate) | **GET** /Weapons/operations/aggregate | 
[**weapons_field_values**](WeaponsApi.md#weapons_field_values) | **GET** /Weapons/operations/field-values | 
[**weapons_get**](WeaponsApi.md#weapons_get) | **GET** /Weapons/item/{id} | 
[**weapons_list**](WeaponsApi.md#weapons_list) | **GET** /Weapons/list | 
[**weapons_variable_distribution**](WeaponsApi.md#weapons_variable_distribution) | **GET** /Weapons/operations/variable-distribution | 



## weapons_aggregate

> Vec<models::WeaponsAggregate> weapons_aggregate(groupby, ordering, additional_filters, aggregates, category, having, id, index_on, metrics, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**category** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**uuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::WeaponsAggregate>**](WeaponsAggregate.md)

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


## weapons_get

> models::Weapons weapons_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Weapons**](Weapons.md)

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


## weapons_variable_distribution

> Vec<models::AgentVariableDistribution200ResponseInner> weapons_variable_distribution(bucket_size, max, metric, min, additional_filters, category, id, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**category** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**uuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::AgentVariableDistribution200ResponseInner>**](Agent_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

