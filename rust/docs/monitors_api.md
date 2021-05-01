# monitors_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**get_monitor_by_id**](monitors_api.md#get_monitor_by_id) | **GET** /monitors/{id} | Get a monitor by ID
**get_monitors**](monitors_api.md#get_monitors) | **GET** /monitors | Get a list of all monitors in your account.
****](monitors_api.md#) | **DELETE** /monitors/{id} | 
**post_monitor**](monitors_api.md#post_monitor) | **POST** /monitors | Create a new monitor
**update_monitor**](monitors_api.md#update_monitor) | **PUT** /monitors/{id} | Update an existing monitor by ID


# **get_monitor_by_id**
> models::Monitor get_monitor_by_id(ctx, id)
Get a monitor by ID

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| ID of monitor to retrieve. | 

### Return type

[**models::Monitor**](Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_monitors**
> models::MonitorArray get_monitors(ctx, )
Get a list of all monitors in your account.

Returns a collection of all monitors created.  

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::MonitorArray**](MonitorArray.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Monitor (ctx, id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| ID of monitor to delete. | 

### Return type

[**models::Monitor**](Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_monitor**
> models::Monitor post_monitor(ctx, monitor)
Create a new monitor

Create a new monitor from the details in the request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **monitor** | [**Monitor**](Monitor.md)| Specify what should be monitored and when. | 

### Return type

[**models::Monitor**](Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_monitor**
> models::Monitor update_monitor(ctx, id, monitor)
Update an existing monitor by ID

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| ID of monitor | 
  **monitor** | [**Monitor**](Monitor.md)| Specify what should be monitored and when. | 

### Return type

[**models::Monitor**](Monitor.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

