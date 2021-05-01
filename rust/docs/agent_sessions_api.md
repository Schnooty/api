# agent_sessions_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**create_agent_session**](agent_sessions_api.md#create_agent_session) | **POST** /session/{groupName} | 
**get_agent_session_state**](agent_sessions_api.md#get_agent_session_state) | **GET** /session/{groupName} | 


# **create_agent_session**
> models::AgentSessionState create_agent_session(group_name, agent_session_request)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_name** | **String**| Name for this group of agents. | 
  **agent_session_request** | [**AgentSessionRequest**](AgentSessionRequest.md)| Create a new agent and generate a new API key. | 

### Return type

[**models::AgentSessionState**](AgentSessionState.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_agent_session_state**
> models::AgentSessionState get_agent_session_state(group_name)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_name** | **String**| Name for this group of agents. | 

### Return type

[**models::AgentSessionState**](AgentSessionState.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

