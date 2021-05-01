# alerts_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
****](alerts_api.md#) | **GET** /alerts | 
****](alerts_api.md#) | **DELETE** /alerts/{id} | 
****](alerts_api.md#) | **GET** /alerts/{id} | 
****](alerts_api.md#) | **PUT** /alerts/{id} | 
****](alerts_api.md#) | **POST** /alerts | 


# ****
> models::AlertArray (ctx, )


Retrieve all of the alerts you have created in your account.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::AlertArray**](AlertArray.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Alert (ctx, id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| ID of the alert. | 

### Return type

[**models::Alert**](Alert.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Alert (ctx, id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| ID of the alert. | 

### Return type

[**models::Alert**](Alert.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Alert (ctx, id, alert)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| ID of the alert. | 
  **alert** | [**Alert**](Alert.md)| Update an existing alert. | 

### Return type

[**models::Alert**](Alert.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Alert (alert)


Create a new alert that can be activated when a monitor goes down. An alert can be a message (email, Microsoft teams), or it can call a web service (webhook). An alert is associated with zero or more of your monitors. If your alert has more than one monitor, the alert will be activated only when the number of monitors that goes down exceededs the `threshold`.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **alert** | [**Alert**](Alert.md)| Create a new alert. | 

### Return type

[**models::Alert**](Alert.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

