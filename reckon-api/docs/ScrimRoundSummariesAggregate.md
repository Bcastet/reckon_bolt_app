# ScrimRoundSummariesAggregate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**round_id** | **String** |  | 
**death_event** | [**Vec<models::CompetitiveRoundSummariesDeathEventInner>**](CompetitiveRoundSummariesDeathEventInner.md) |  | 
**smoke_abilities** | [**Vec<models::CompetitiveRoundSummariesSmokeAbilitiesInner>**](CompetitiveRoundSummariesSmokeAbilitiesInner.md) |  | 
**utility_abilities** | [**Vec<models::CompetitiveRoundSummariesUtilityAbilitiesInner>**](CompetitiveRoundSummariesUtilityAbilitiesInner.md) |  | 
**other_abilities** | [**Vec<models::CompetitiveRoundSummariesOtherAbilitiesInner>**](CompetitiveRoundSummariesOtherAbilitiesInner.md) |  | 
**player_snapshots** | [**Vec<models::CompetitiveRoundSummariesPlayerSnapshotsInner>**](CompetitiveRoundSummariesPlayerSnapshotsInner.md) |  | 
**kill_events** | [**Vec<models::CompetitiveRoundSummariesDeathEventInner>**](CompetitiveRoundSummariesDeathEventInner.md) |  | 
**positions** | [**Vec<models::CompetitiveRoundSummariesSmokeAbilitiesInnerCastPosition>**](CompetitiveRoundSummariesSmokeAbilitiesInnerCastPosition.md) |  | 
**death_event_agnostic** | Option<**String**> |  | [optional]
**kills** | **i32** |  | 
**deaths** | **i32** |  | 
**assists** | **i32** |  | 
**was_traded** | Option<**bool**> |  | [optional]
**agent** | **String** |  | 
**patch** | **String** |  | 
**side** | **String** |  | 
**round_eco_type** | **String** |  | 
**round_eco_type_no_bonus** | **String** |  | 
**round_strat_type** | **String** |  | 
**round_index** | **i32** |  | 
**main_weapon** | **String** |  | 
**second_weapon** | Option<**String**> |  | [optional]
**loadout_value** | Option<**i32**> |  | [optional]
**ability1_charges** | Option<**i32**> |  | [optional]
**ability2_charges** | Option<**i32**> |  | [optional]
**ability3_charges** | Option<**i32**> |  | [optional]
**grenade_charges** | Option<**i32**> |  | [optional]
**ultimate_charges** | Option<**i32**> |  | [optional]
**shield** | Option<**String**> |  | [optional]
**damages** | **i32** |  | 
**damage_instances** | **i32** |  | 
**took_spike** | Option<**bool**> |  | [optional]
**planted_spike** | Option<**bool**> |  | [optional]
**combat_score** | **f64** |  | 
**win** | **bool** |  | 
**round_end** | Option<**String**> |  | [optional]
**first_death** | **bool** |  | 
**first_blood** | **bool** |  | 
**first_true_death** | **bool** |  | 
**first_true_blood** | **bool** |  | 
**investment** | **i32** |  | 
**bomb_site** | Option<**String**> |  | [optional]
**first_attack_site** | Option<**String**> |  | [optional]
**attack_pattern_full** | Option<**String**> |  | [optional]
**defense_pattern_full** | Option<**String**> |  | [optional]
**attack_pattern_short** | Option<**String**> |  | [optional]
**defense_pattern_short** | Option<**String**> |  | [optional]
**plant_location** | Option<**String**> |  | [optional]
**plant_time** | Option<**i32**> |  | [optional]
**start_time_seconds** | Option<**i32**> |  | [optional]
**freeze_time_end_timestamp** | Option<**i32**> |  | [optional]
**callout_15s** | Option<**String**> |  | [optional]
**kast** | **bool** |  | 
**date** | **String** |  | 
**summary** | **String** |  | 
**player** | **String** |  | 
**team1** | **String** |  | 
**team2** | **String** |  | 
**game** | **String** |  | 
**map** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


