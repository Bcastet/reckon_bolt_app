# UploadScrimGameError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | **String** | Description of what went wrong | 
**unresolved_accounts** | Option<[**Vec<models::UnresolvedAccount>**](UnresolvedAccount.md)> | Present when live-client PUUIDs could not be translated via Account-v1 | [optional]
**unlinked_accounts** | Option<[**Vec<models::UnlinkedAccount>**](UnlinkedAccount.md)> | Present when SoloQAccounts are missing player_id links | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


