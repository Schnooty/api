# MonitorBody

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | [optional] [default to None]
**method** | **String** |  | [optional] [default to None]
**headers** | [**Vec<models::HttpHeader>**](HttpHeader.md) |  | [optional] [default to None]
**body** | **String** |  | [optional] [default to None]
**executable** | **String** | The name of the executable process to be monitored. | [optional] [default to None]
**is_path_absolute** | **bool** | If true, the process(s) will be located by the full path of the executable e.g. /usr/bin/node | [optional] [default to Some(false)]
**minimum_count** | **isize** | The minimum number of processes that match the executable. | [optional] [default to None]
**maximum_count** | **isize** | The maximum number of processes that match the executable.  | [optional] [default to None]
**maximum_ram_individual** | **String** |  | [optional] [default to None]
**maximum_ram_total** | **String** |  | [optional] [default to None]
**hostname** | **String** |  | [optional] [default to None]
**port** | **u16** |  | [optional] [default to None]
**db** | **isize** |  | [optional] [default to None]
**username** | **String** |  | [optional] [default to None]
**password** | **String** |  | [optional] [default to None]
**expression** | [***models::BoolExpr**](BoolExpr.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


