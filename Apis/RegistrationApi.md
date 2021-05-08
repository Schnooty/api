# RegistrationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirmRegistration**](RegistrationApi.md#confirmRegistration) | **POST** /registration/{id} | Confirm registration of Open Monitors account.
[**createRegistration**](RegistrationApi.md#createRegistration) | **POST** /registration | Register your email address and password.


<a name="confirmRegistration"></a>
# **confirmRegistration**
> confirmRegistration(id, RegistrationConfirmation)

Confirm registration of Open Monitors account.

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of registration to confirm. | [default to null]
 **RegistrationConfirmation** | [**RegistrationConfirmation**](../Models/RegistrationConfirmation.md)| Details including confirmation code that prove the user&#39;s email address. |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="createRegistration"></a>
# **createRegistration**
> Registration createRegistration(Registration)

Register your email address and password.

    This is the endpoint you will use to create an account. This will enable you to use the API. By submitting your email address, password, and a name for your account, Open Monitors will email you a confirmation code. The confirmation code will be valid for 24 hours or less, and may only be used once. Note that this API will respond with a 200 response code even if your email address is in Open Monitor&#39;s system. This is for privacy reasons.

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **Registration** | [**Registration**](../Models/Registration.md)| Details for registering for an Open Monitors account. |

### Return type

[**Registration**](../Models/Registration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

