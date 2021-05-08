# StatusesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getMonitorStatuses**](StatusesApi.md#getMonitorStatuses) | **GET** /statuses | 
[**updateMonitorStatuses**](StatusesApi.md#updateMonitorStatuses) | **POST** /statuses | 


<a name="getMonitorStatuses"></a>
# **getMonitorStatuses**
> MonitorStatusArray getMonitorStatuses()



### Parameters
This endpoint does not need any parameter.

### Return type

[**MonitorStatusArray**](../Models/MonitorStatusArray.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="updateMonitorStatuses"></a>
# **updateMonitorStatuses**
> updateMonitorStatuses(MonitorStatusArray)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **MonitorStatusArray** | [**MonitorStatusArray**](../Models/MonitorStatusArray.md)| Specify what should be monitored and when. |

### Return type

null (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

