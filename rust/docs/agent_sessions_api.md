# agent_sessions_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**clear_session**](agent_sessions_api.md#clear_session) | **GET** /sessions/{identifier} | 
**get_session**](agent_sessions_api.md#get_session) | **DELETE** /sessions/{identifier} | 
**get_sessions**](agent_sessions_api.md#get_sessions) | **GET** /sessions | 
**put_session**](agent_sessions_api.md#put_session) | **PUT** /sessions/{identifier} | 


# **clear_session**
> models::SessionContainer clear_session(identifier)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **identifier** | **String**| Unique name for this session. | 

### Return type

[**models::SessionContainer**](SessionContainer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_session**
> models::SessionContainer get_session(identifier)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **identifier** | **String**| Unique name for this session. | 

### Return type

[**models::SessionContainer**](SessionContainer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sessions**
> models::SessionArray get_sessions()


### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::SessionArray**](SessionArray.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_session**
> models::SessionContainer put_session(identifier, session)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **identifier** | **String**| Unique name for this session. | 
  **session** | [**Session**](Session.md)| Create a new agent and generate a new API key. | 

### Return type

[**models::SessionContainer**](SessionContainer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

