# MonitorStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status_id** | **String** |  | [readonly] 
**monitor_type** | [***models::MonitorType**](MonitorType.md) |  | 
**monitor_name** | **String** |  | 
**status** | [***models::MonitorStatusIndicator**](MonitorStatusIndicator.md) |  | 
**timestamp** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | UTC UNIX timestamp in with fractional offset. | [readonly] 
**expires_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | UTC UNIX timestamp in with fractional offset. | [readonly] 
**expected_result** | **String** |  | 
**actual_result** | **String** |  | 
**description** | **String** |  | 
**session** | [***models::Session**](Session.md) |  | [optional] [default to None]
**log** | [**Vec<models::MonitorStatusLogEntry>**](MonitorStatusLogEntry.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


