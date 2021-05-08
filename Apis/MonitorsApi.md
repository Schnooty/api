# MonitorsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getMonitorById**](MonitorsApi.md#getMonitorById) | **GET** /monitors/{id} | Get a monitor by ID
[**getMonitors**](MonitorsApi.md#getMonitors) | **GET** /monitors | Get a list of all monitors in your account.
[**monitorsIdDelete**](MonitorsApi.md#monitorsIdDelete) | **DELETE** /monitors/{id} | 
[**postMonitor**](MonitorsApi.md#postMonitor) | **POST** /monitors | Create a new monitor
[**updateMonitor**](MonitorsApi.md#updateMonitor) | **PUT** /monitors/{id} | Update an existing monitor by ID


<a name="getMonitorById"></a>
# **getMonitorById**
> Monitor getMonitorById(id)

Get a monitor by ID

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of monitor to retrieve. | [default to null]

### Return type

[**Monitor**](../Models/Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getMonitors"></a>
# **getMonitors**
> MonitorArray getMonitors()

Get a list of all monitors in your account.

    Returns a collection of all monitors created.  

### Parameters
This endpoint does not need any parameter.

### Return type

[**MonitorArray**](../Models/MonitorArray.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="monitorsIdDelete"></a>
# **monitorsIdDelete**
> Monitor monitorsIdDelete(id)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of monitor to delete. | [default to null]

### Return type

[**Monitor**](../Models/Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="postMonitor"></a>
# **postMonitor**
> Monitor postMonitor(Monitor)

Create a new monitor

    Create a new monitor from the details in the request.

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **Monitor** | [**Monitor**](../Models/Monitor.md)| Specify what should be monitored and when. |

### Return type

[**Monitor**](../Models/Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="updateMonitor"></a>
# **updateMonitor**
> Monitor updateMonitor(id, Monitor)

Update an existing monitor by ID

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of monitor | [default to null]
 **Monitor** | [**Monitor**](../Models/Monitor.md)| Specify what should be monitored and when. |

### Return type

[**Monitor**](../Models/Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

