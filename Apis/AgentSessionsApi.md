# AgentSessionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createAgentSession**](AgentSessionsApi.md#createAgentSession) | **POST** /session/{groupName} | 
[**getAgentSessionState**](AgentSessionsApi.md#getAgentSessionState) | **GET** /session/{groupName} | 


<a name="createAgentSession"></a>
# **createAgentSession**
> AgentSessionState createAgentSession(groupName, AgentSessionRequest)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupName** | **String**| Name for this group of agents. | [default to null]
 **AgentSessionRequest** | [**AgentSessionRequest**](../Models/AgentSessionRequest.md)| Create a new agent and generate a new API key. |

### Return type

[**AgentSessionState**](../Models/AgentSessionState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="getAgentSessionState"></a>
# **getAgentSessionState**
> AgentSessionState getAgentSessionState(groupName)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupName** | **String**| Name for this group of agents. | [default to null]

### Return type

[**AgentSessionState**](../Models/AgentSessionState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

