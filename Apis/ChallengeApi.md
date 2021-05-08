# ChallengeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createChallenge**](ChallengeApi.md#createChallenge) | **POST** /challenge | Create a challenge to prove you are human
[**updateChallenge**](ChallengeApi.md#updateChallenge) | **POST** /challenge/{id} | Solve a challenge and prove you are human.


<a name="createChallenge"></a>
# **createChallenge**
> RegistrationChallenge createChallenge()

Create a challenge to prove you are human

### Parameters
This endpoint does not need any parameter.

### Return type

[**RegistrationChallenge**](../Models/RegistrationChallenge.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="updateChallenge"></a>
# **updateChallenge**
> RegistrationChallenge updateChallenge(id)

Solve a challenge and prove you are human.

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of registration to confirm. | [default to null]

### Return type

[**RegistrationChallenge**](../Models/RegistrationChallenge.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

