# SoloQGameSummariesAggregate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **String** |  | [readonly]
**date** | **String** |  | 
**patch** | **String** |  | 
**kills** | **i32** |  | 
**deaths** | **i32** |  | 
**assists** | **i32** |  | 
**kpr** | Option<**f64**> |  | [optional]
**dpr** | Option<**f64**> |  | [optional]
**impact** | Option<**f64**> |  | [optional]
**adr** | Option<**f64**> |  | [optional]
**kast** | Option<**f64**> |  | [optional]
**rating_hltv** | Option<**f64**> |  | [optional]
**rating_vlr** | Option<**f64**> |  | [optional]
**combat_score** | Option<**f64**> |  | [optional]
**win** | **bool** |  | 
**acs** | Option<**f64**> |  | [optional]
**headshot_percent** | Option<**f64**> |  | [optional]
**headshots** | Option<**i32**> |  | [optional]
**first_kills** | Option<**f64**> |  | [optional]
**first_deaths** | Option<**f64**> |  | [optional]
**got_traded** | Option<**i32**> |  | [optional]
**rounds_played** | Option<**i32**> |  | [optional]
**rounds_won** | Option<**i32**> |  | [optional]
**rounds_lost** | Option<**i32**> |  | [optional]
**game_id** | **uuid::Uuid** |  | 
**participant_id** | **i32** |  | 
**rank_tier** | [**models::RankTierEnum**](RankTierEnum.md) |  | 
**server** | **String** |  | 
**ig_name** | **String** |  | 
**puuid** | **String** |  | 
**team** | [**models::TeamEnum**](TeamEnum.md) |  | 
**agent** | **String** |  | 
**competitive_player** | Option<**String**> |  | [optional]
**account** | Option<**i32**> |  | [optional]
**map** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


