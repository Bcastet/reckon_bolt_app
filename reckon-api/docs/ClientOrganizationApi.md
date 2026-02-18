# \ClientOrganizationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**client_organization_aggregate**](ClientOrganizationApi.md#client_organization_aggregate) | **GET** /ClientOrganization/operations/aggregate | 
[**client_organization_field_values**](ClientOrganizationApi.md#client_organization_field_values) | **GET** /ClientOrganization/operations/field-values | 
[**client_organization_get**](ClientOrganizationApi.md#client_organization_get) | **GET** /ClientOrganization/item/{id} | 
[**client_organization_list**](ClientOrganizationApi.md#client_organization_list) | **GET** /ClientOrganization/list | 
[**client_organization_patch**](ClientOrganizationApi.md#client_organization_patch) | **PATCH** /ClientOrganization/item/{id} | 
[**client_organization_variable_distribution**](ClientOrganizationApi.md#client_organization_variable_distribution) | **GET** /ClientOrganization/operations/variable-distribution | 



## client_organization_aggregate

> Vec<models::ClientOrganizationAggregate> client_organization_aggregate(groupby, ordering, additional_filters, aggregates, having, index_on, metrics)


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

### Return type

[**Vec<models::ClientOrganizationAggregate>**](ClientOrganizationAggregate.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_organization_field_values

> Vec<String> client_organization_field_values(field, ordering, additional_filters)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Field to retrieve the unique values from | [required] |
**ordering** | [**Vec<String>**](String.md) | Order by | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |

### Return type

**Vec<String>**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## client_organization_patch

> models::ClientOrganization client_organization_patch(id, patched_client_organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_client_organization** | Option<[**PatchedClientOrganization**](PatchedClientOrganization.md)> |  |  |

### Return type

[**models::ClientOrganization**](ClientOrganization.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_organization_variable_distribution

> Vec<models::ClientOrganizationVariableDistribution200ResponseInner> client_organization_variable_distribution(bucket_size, max, metric, min, additional_filters)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_size** | **i32** | Bucket size | [required] |
**max** | **i32** | Max threshold | [required] |
**metric** | **String** | Metric to get distribution from | [required] |
**min** | **i32** | Min threshold | [required] |
**additional_filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Other filters using lookups |  |

### Return type

[**Vec<models::ClientOrganizationVariableDistribution200ResponseInner>**](ClientOrganization_variable_distribution_200_response_inner.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

