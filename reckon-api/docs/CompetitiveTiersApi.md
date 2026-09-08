# \CompetitiveTiersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**competitive_tiers_aggregate**](CompetitiveTiersApi.md#competitive_tiers_aggregate) | **GET** /CompetitiveTiers/operations/aggregate | 
[**competitive_tiers_field_values**](CompetitiveTiersApi.md#competitive_tiers_field_values) | **GET** /CompetitiveTiers/operations/field-values | 
[**competitive_tiers_get**](CompetitiveTiersApi.md#competitive_tiers_get) | **GET** /CompetitiveTiers/item/{id} | 
[**competitive_tiers_list**](CompetitiveTiersApi.md#competitive_tiers_list) | **GET** /CompetitiveTiers/list | 
[**competitive_tiers_variable_distribution**](CompetitiveTiersApi.md#competitive_tiers_variable_distribution) | **GET** /CompetitiveTiers/operations/variable-distribution | 



## competitive_tiers_aggregate

> Vec<models::CompetitiveTiersAggregate> competitive_tiers_aggregate(groupby, ordering, additional_filters, aggregates, division, having, id, index_on, metrics, tier)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**division** | Option<**String**> |  |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**id** | Option<**String**> |  |  |
**index_on** | Option<**String**> | Index results on a column |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**tier** | Option<**i32**> |  |  |

### Return type

[**Vec<models::CompetitiveTiersAggregate>**](CompetitiveTiersAggregate.md)

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


## competitive_tiers_get

> models::CompetitiveTiers competitive_tiers_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::CompetitiveTiers**](CompetitiveTiers.md)

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


## competitive_tiers_variable_distribution

> Vec<models::AgentVariableDistribution200ResponseInner> competitive_tiers_variable_distribution(bucket_size, max, metric, min, additional_filters, division, id, tier)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**division** | Option<**String**> |  |  |
**id** | Option<**String**> |  |  |
**tier** | Option<**i32**> |  |  |

### Return type

[**Vec<models::AgentVariableDistribution200ResponseInner>**](Agent_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

