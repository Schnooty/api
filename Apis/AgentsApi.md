# AgentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**agentsGet**](AgentsApi.md#agentsGet) | **GET** /agents | 
[**agentsIdDelete**](AgentsApi.md#agentsIdDelete) | **DELETE** /agents/{id} | 
[**agentsIdGet**](AgentsApi.md#agentsIdGet) | **GET** /agents/{id} | 
[**agentsIdPut**](AgentsApi.md#agentsIdPut) | **PUT** /agents/{id} | 
[**agentsPost**](AgentsApi.md#agentsPost) | **POST** /agents | 


<a name="agentsGet"></a>
# **agentsGet**
> List agentsGet()



### Parameters
This endpoint does not need any parameter.

### Return type

[**List**](../Models/Agent.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="agentsIdDelete"></a>
# **agentsIdDelete**
> Agent agentsIdDelete(id)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the alert. | [default to null]

### Return type

[**Agent**](../Models/Agent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="agentsIdGet"></a>
# **agentsIdGet**
> Agent agentsIdGet(id)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the alert. | [default to null]

### Return type

[**Agent**](../Models/Agent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="agentsIdPut"></a>
# **agentsIdPut**
> Agent agentsIdPut(id, Agent)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the alert. | [default to null]
 **Agent** | [**Agent**](../Models/Agent.md)| Update an existing agent. |

### Return type

[**Agent**](../Models/Agent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="agentsPost"></a>
# **agentsPost**
> Agent agentsPost(Agent)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **Agent** | [**Agent**](../Models/Agent.md)| Create a new agent and generate a new API key. |

### Return type

[**Agent**](../Models/Agent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

