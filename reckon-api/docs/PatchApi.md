# \PatchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**client_organization_patch**](PatchApi.md#client_organization_patch) | **PATCH** /ClientOrganization/item/{id} | 
[**link_riot_account_create**](PatchApi.md#link_riot_account_create) | **POST** /LinkRiotAccount | 
[**player_patch**](PatchApi.md#player_patch) | **PATCH** /Player/item/{id} | 
[**team_patch**](PatchApi.md#team_patch) | **PATCH** /Team/item/{id} | 
[**user_patch**](PatchApi.md#user_patch) | **PATCH** /User/item/{id} | 



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


## team_patch

> models::Team team_patch(id, patched_team)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_team** | Option<[**PatchedTeam**](PatchedTeam.md)> |  |  |

### Return type

[**models::Team**](Team.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_patch

> models::User user_patch(id, patched_user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patched_user** | Option<[**PatchedUser**](PatchedUser.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

