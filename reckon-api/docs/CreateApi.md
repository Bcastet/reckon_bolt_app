# \CreateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**player_create**](CreateApi.md#player_create) | **POST** /Player/item/{id} | 
[**solo_q_accounts_create**](CreateApi.md#solo_q_accounts_create) | **POST** /SoloQAccounts/item/{id} | 
[**user_create**](CreateApi.md#user_create) | **POST** /User/item/{id} | 



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


## user_create

> models::NewUser user_create(id, user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user** | [**User**](User.md) |  | [required] |

### Return type

[**models::NewUser**](NewUser.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

