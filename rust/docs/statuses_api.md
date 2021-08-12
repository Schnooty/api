# statuses_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**get_monitor_statuses**](statuses_api.md#get_monitor_statuses) | **GET** /statuses | 
**update_monitor_statuses**](statuses_api.md#update_monitor_statuses) | **POST** /statuses | 


# **get_monitor_statuses**
> models::MonitorStatusArray get_monitor_statuses(ctx, )


### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::MonitorStatusArray**](MonitorStatusArray.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_monitor_statuses**
> update_monitor_statuses(ctx, monitor_status_array)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **monitor_status_array** | [**MonitorStatusArray**](MonitorStatusArray.md)| Specify what should be monitored and when. | 

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

