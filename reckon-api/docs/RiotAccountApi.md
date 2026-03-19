# \RiotAccountApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**link_riot_account_create**](RiotAccountApi.md#link_riot_account_create) | **POST** /LinkRiotAccount | 



## link_riot_account_create

> models::LinkRiotAccountResponse link_riot_account_create(link_riot_account_request)


Link the authenticated user's account to their Riot Games identity.  The frontend redirects the user to Riot's authorization page. After the user authorizes, the frontend receives an authorization code and sends it here. The backend then:  1. Exchanges the authorization code for access / refresh tokens via the    Riot RSO token endpoint. 2. Uses the access token to retrieve the player's PUUID, game name and    tagline from the Riot Account API. 3. Persists everything on the ``AuthUser`` model for later use (e.g. fetching    match history, including private games).

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

