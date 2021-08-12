# agents_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
****](agents_api.md#) | **GET** /agents | 
****](agents_api.md#) | **DELETE** /agents/{id} | 
****](agents_api.md#) | **GET** /agents/{id} | 
****](agents_api.md#) | **PUT** /agents/{id} | 
****](agents_api.md#) | **POST** /agents | 


# ****
> Vec<models::Agent> (ctx, )


### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::Agent>**](Agent.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Agent (id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of the alert. | 

### Return type

[**models::Agent**](Agent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Agent (id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of the alert. | 

### Return type

[**models::Agent**](Agent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Agent (id, agent)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of the alert. | 
  **agent** | [**Agent**](Agent.md)| Update an existing agent. | 

### Return type

[**models::Agent**](Agent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::Agent (agent)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **agent** | [**Agent**](Agent.md)| Create a new agent and generate a new API key. | 

### Return type

[**models::Agent**](Agent.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

