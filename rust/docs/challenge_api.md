# challenge_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**create_challenge**](challenge_api.md#create_challenge) | **POST** /challenge | Create a challenge to prove you are human
**update_challenge**](challenge_api.md#update_challenge) | **POST** /challenge/{id} | Solve a challenge and prove you are human.


# **create_challenge**
> models::RegistrationChallenge create_challenge()
Create a challenge to prove you are human

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::RegistrationChallenge**](RegistrationChallenge.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_challenge**
> models::RegistrationChallenge update_challenge(id)
Solve a challenge and prove you are human.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of registration to confirm. | 

### Return type

[**models::RegistrationChallenge**](RegistrationChallenge.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

