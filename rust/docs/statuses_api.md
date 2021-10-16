# statuses_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**clear_status**](statuses_api.md#clear_status) | **DELETE** /statuses/{statusId} | 
**get_monitor_status**](statuses_api.md#get_monitor_status) | **GET** /statuses/{statusId} | 
**get_monitor_statuses**](statuses_api.md#get_monitor_statuses) | **GET** /statuses | 
**set_status**](statuses_api.md#set_status) | **POST** /statuses/{statusId} | 


# **clear_status**
> models::MonitorStatusContainer clear_status(ctx, status_id)


Remove a status from being active

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **status_id** | **String**| Name of the monitor status | 

### Return type

[**models::MonitorStatusContainer**](MonitorStatusContainer.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_monitor_status**
> models::MonitorStatusContainer get_monitor_status(ctx, status_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **status_id** | **String**| Name of the monitor status | 

### Return type

[**models::MonitorStatusContainer**](MonitorStatusContainer.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

# **set_status**
> models::MonitorStatusContainer set_status(ctx, status_id, monitor_status)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **status_id** | **String**| Name of the monitor status | 
  **monitor_status** | [**MonitorStatus**](MonitorStatus.md)| Status you wish to set | 

### Return type

[**models::MonitorStatusContainer**](MonitorStatusContainer.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

