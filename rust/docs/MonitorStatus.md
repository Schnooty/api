# MonitorStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**monitor_id** | **String** |  | [readonly] 
**status** | [***models::MonitorStatusIndicator**](MonitorStatusIndicator.md) |  | 
**timestamp** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | UTC UNIX timestamp in with fractional offset. | [readonly] 
**last_result** | [***models::MonitorStatusResult**](MonitorStatusResult.md) |  | 
**description** | **String** |  | 
**log** | [**Vec<models::MonitorStatusLogEntry>**](MonitorStatusLogEntry.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


