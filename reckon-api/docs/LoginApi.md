# \LoginApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**login_create**](LoginApi.md#login_create) | **POST** /Login | 



## login_create

> models::Login login_create(username, password, token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**password** | **String** |  | [required] |
**token** | **String** |  | [required] |

### Return type

[**models::Login**](Login.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

