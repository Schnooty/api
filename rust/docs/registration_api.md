# registration_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**confirm_registration**](registration_api.md#confirm_registration) | **POST** /registration/{id} | Confirm registration of account.
**create_registration**](registration_api.md#create_registration) | **POST** /registration | Register your email address and password.


# **confirm_registration**
> confirm_registration(id, registration_confirmation)
Confirm registration of account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of registration to confirm. | 
  **registration_confirmation** | [**RegistrationConfirmation**](RegistrationConfirmation.md)| Details including confirmation code that prove the user's email address. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_registration**
> models::Registration create_registration(registration)
Register your email address and password.

This is the endpoint you will use to create an account. This will enable you to use the API. By submitting your email address, password, and a name for your account, you will be emailed a confirmation code. The confirmation code will be valid for 24 hours or less, and may only be used once. Note that this API will respond with a 200 response code even if your email address is in Open Monitor's system. This is for privacy reasons.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **registration** | [**Registration**](Registration.md)| Details for registering an account. | 

### Return type

[**models::Registration**](Registration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

