# MonitorBody
## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | [**String**](string.md) |  | [optional] [default to null]
**method** | [**String**](string.md) |  | [optional] [default to null]
**headers** | [**List**](HttpHeader.md) |  | [optional] [default to null]
**body** | [**String**](string.md) |  | [optional] [default to null]
**executable** | [**String**](string.md) | The name of the executable process to be monitored. | [optional] [default to null]
**isPathAbsolute** | [**Boolean**](boolean.md) | If true, the process(s) will be located by the full path of the executable e.g. /usr/bin/node | [optional] [default to false]
**minimumCount** | [**Integer**](integer.md) | The minimum number of processes that match the executable. | [optional] [default to null]
**maximumCount** | [**Integer**](integer.md) | The maximum number of processes that match the executable.  | [optional] [default to null]
**maximumRamIndividual** | [**String**](string.md) |  | [optional] [default to null]
**maximumRamTotal** | [**String**](string.md) |  | [optional] [default to null]
**hostname** | [**String**](string.md) |  | [optional] [default to null]
**port** | [**Integer**](integer.md) |  | [optional] [default to null]
**db** | [**Integer**](integer.md) |  | [optional] [default to null]
**username** | [**String**](string.md) |  | [optional] [default to null]
**password** | [**String**](string.md) |  | [optional] [default to null]
**constraints** | [**List**](FieldConstraint.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

