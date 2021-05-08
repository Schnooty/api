# AlertsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alertsGet**](AlertsApi.md#alertsGet) | **GET** /alerts | 
[**alertsIdDelete**](AlertsApi.md#alertsIdDelete) | **DELETE** /alerts/{id} | 
[**alertsIdGet**](AlertsApi.md#alertsIdGet) | **GET** /alerts/{id} | 
[**alertsIdPut**](AlertsApi.md#alertsIdPut) | **PUT** /alerts/{id} | 
[**alertsPost**](AlertsApi.md#alertsPost) | **POST** /alerts | 


<a name="alertsGet"></a>
# **alertsGet**
> AlertArray alertsGet()



    Retrieve all of the alerts you have created in your account.

### Parameters
This endpoint does not need any parameter.

### Return type

[**AlertArray**](../Models/AlertArray.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="alertsIdDelete"></a>
# **alertsIdDelete**
> Alert alertsIdDelete(id)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the alert. | [default to null]

### Return type

[**Alert**](../Models/Alert.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="alertsIdGet"></a>
# **alertsIdGet**
> Alert alertsIdGet(id)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the alert. | [default to null]

### Return type

[**Alert**](../Models/Alert.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="alertsIdPut"></a>
# **alertsIdPut**
> Alert alertsIdPut(id, Alert)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the alert. | [default to null]
 **Alert** | [**Alert**](../Models/Alert.md)| Update an existing alert. |

### Return type

[**Alert**](../Models/Alert.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="alertsPost"></a>
# **alertsPost**
> Alert alertsPost(Alert)



    Create a new alert that can be activated when a monitor goes down. An alert can be a message (email, Microsoft teams), or it can call a web service (webhook). An alert is associated with zero or more of your monitors. If your alert has more than one monitor, the alert will be activated only when the number of monitors that goes down exceededs the &#x60;threshold&#x60;.

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **Alert** | [**Alert**](../Models/Alert.md)| Create a new alert. |

### Return type

[**Alert**](../Models/Alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

