# \ScrimsDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**upload_scrim_game**](ScrimsDataApi.md#upload_scrim_game) | **POST** /ScrimsData/item/{id}/upload | Manually upload a scrim game



## upload_scrim_game

> models::UploadScrimGameSuccess upload_scrim_game(id, upload_scrim_game_request)
Manually upload a scrim game

Upload a scrim game via **multipart/form-data**.  **Fields**: `team1` (team ID), `team2` (team ID)  **Files**: `agnostic_match_history` (Riot match history JSON)  SoloQAccount entries are auto-created for unknown PUUIDs, but each account must have a `player_id` linked before the upload can proceed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**upload_scrim_game_request** | [**UploadScrimGameRequest**](UploadScrimGameRequest.md) |  | [required] |

### Return type

[**models::UploadScrimGameSuccess**](UploadScrimGameSuccess.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

