# \RiotAccountApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**link_riot_account_create**](RiotAccountApi.md#link_riot_account_create) | **POST** /LinkRiotAccount | 



## link_riot_account_create

> models::LinkRiotAccountResponse link_riot_account_create(link_riot_account_request)


Link the authenticated user's account to their Riot Games identity.  The frontend is responsible for performing the full RSO OAuth2 authorization flow and obtaining the tokens.  It then sends the token payload here so the backend can:  1. Validate the access token against the Riot Account API. 2. Retrieve the player's PUUID, game name and tagline. 3. Persist everything on the ``AuthUser`` model for later use (e.g. fetching    match history, including private games).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_riot_account_request** | [**LinkRiotAccountRequest**](LinkRiotAccountRequest.md) |  | [required] |

### Return type

[**models::LinkRiotAccountResponse**](LinkRiotAccountResponse.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

