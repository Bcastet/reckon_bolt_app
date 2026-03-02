# \SoloQAccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**solo_q_accounts_aggregate**](SoloQAccountsApi.md#solo_q_accounts_aggregate) | **GET** /SoloQAccounts/operations/aggregate | 
[**solo_q_accounts_create**](SoloQAccountsApi.md#solo_q_accounts_create) | **POST** /SoloQAccounts/item/{id} | 
[**solo_q_accounts_field_values**](SoloQAccountsApi.md#solo_q_accounts_field_values) | **GET** /SoloQAccounts/operations/field-values | 
[**solo_q_accounts_get**](SoloQAccountsApi.md#solo_q_accounts_get) | **GET** /SoloQAccounts/item/{id} | 
[**solo_q_accounts_list**](SoloQAccountsApi.md#solo_q_accounts_list) | **GET** /SoloQAccounts/list | 
[**solo_q_accounts_patch**](SoloQAccountsApi.md#solo_q_accounts_patch) | **PATCH** /SoloQAccounts/item/{id} | 
[**solo_q_accounts_variable_distribution**](SoloQAccountsApi.md#solo_q_accounts_variable_distribution) | **GET** /SoloQAccounts/operations/variable-distribution | 



## solo_q_accounts_aggregate

> Vec<models::SoloQAccountsAggregate> solo_q_accounts_aggregate(groupby, ordering, additional_filters, aggregates, having, index_on, metrics, puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groupby** | [**Vec<String>**](String.md) | Fields to be grouped | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**aggregates** | Option<[**Vec<String>**](String.md)> | Expressions to compute |  |
**having** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**index_on** | Option<**String**> | Index results on a column |  |
**metrics** | Option<[**Vec<String>**](String.md)> | Metrics to compute |  |
**puuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::SoloQAccountsAggregate>**](SoloQAccountsAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_accounts_create

> models::SoloQAccounts solo_q_accounts_create(id, solo_q_accounts)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**solo_q_accounts** | [**SoloQAccounts**](SoloQAccounts.md) |  | [required] |

### Return type

[**models::SoloQAccounts**](SoloQAccounts.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_accounts_field_values

> Vec<String> solo_q_accounts_field_values(field, ordering, additional_filters, puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**puuid** | Option<**String**> |  |  |

### Return type

**Vec<String>**

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


## solo_q_accounts_list

> Vec<models::SoloQAccounts> solo_q_accounts_list(additional_filters, annotations, only_values, ordering, puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**annotations** | Option<[**Vec<String>**](String.md)> | Additional fiedls (joins,...) |  |
**only_values** | Option<[**Vec<String>**](String.md)> | Only get the specified fields |  |
**ordering** | Option<[**Vec<String>**](String.md)> | Order by |  |
**puuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::SoloQAccounts>**](SoloQAccounts.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_accounts_patch

> models::SoloQAccounts solo_q_accounts_patch(id, patched_solo_q_accounts)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_solo_q_accounts** | Option<[**PatchedSoloQAccounts**](PatchedSoloQAccounts.md)> |  |  |

### Return type

[**models::SoloQAccounts**](SoloQAccounts.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## solo_q_accounts_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> solo_q_accounts_variable_distribution(bucket_size, max, metric, min, additional_filters, puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |
**puuid** | Option<**String**> |  |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

