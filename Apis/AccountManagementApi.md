# AccountManagementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accountsIdPut**](AccountManagementApi.md#accountsIdPut) | **PUT** /accounts/{id} | 
[**createAccount**](AccountManagementApi.md#createAccount) | **POST** /accounts | 
[**createSubscription**](AccountManagementApi.md#createSubscription) | **POST** /subscriptions | 
[**getAccountBalance**](AccountManagementApi.md#getAccountBalance) | **GET** /balances | 
[**getAccountById**](AccountManagementApi.md#getAccountById) | **GET** /accounts/{id} | 
[**getAccounts**](AccountManagementApi.md#getAccounts) | **GET** /accounts | 
[**getPlans**](AccountManagementApi.md#getPlans) | **GET** /plans | 
[**getSubscriptionById**](AccountManagementApi.md#getSubscriptionById) | **GET** /subscriptions/{id} | 
[**getSubscriptionRecords**](AccountManagementApi.md#getSubscriptionRecords) | **GET** /subscriptions | 
[**getTransactions**](AccountManagementApi.md#getTransactions) | **GET** /transactions | 


<a name="accountsIdPut"></a>
# **accountsIdPut**
> inline_response_200_1 accountsIdPut(id, Account)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the account. | [default to null]
 **Account** | [**Account**](../Models/Account.md)| Account to be created |

### Return type

[**inline_response_200_1**](../Models/inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="createAccount"></a>
# **createAccount**
> inline_response_200_1 createAccount(Account)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **Account** | [**Account**](../Models/Account.md)| Account to be created |

### Return type

[**inline_response_200_1**](../Models/inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="createSubscription"></a>
# **createSubscription**
> SubscriptionContainer createSubscription(Subscription)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **Subscription** | [**Subscription**](../Models/Subscription.md)| Subscription to be created |

### Return type

[**SubscriptionContainer**](../Models/SubscriptionContainer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="getAccountBalance"></a>
# **getAccountBalance**
> inline_response_200_2 getAccountBalance(accountId)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **accountId** | **String**| The account whose current balance we want | [default to null]

### Return type

[**inline_response_200_2**](../Models/inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getAccountById"></a>
# **getAccountById**
> inline_response_200_1 getAccountById(id)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the account. | [default to null]

### Return type

[**inline_response_200_1**](../Models/inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getAccounts"></a>
# **getAccounts**
> inline_response_200 getAccounts()



### Parameters
This endpoint does not need any parameter.

### Return type

[**inline_response_200**](../Models/inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getPlans"></a>
# **getPlans**
> PlanArray getPlans()



### Parameters
This endpoint does not need any parameter.

### Return type

[**PlanArray**](../Models/PlanArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getSubscriptionById"></a>
# **getSubscriptionById**
> SubscriptionContainer getSubscriptionById(id)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| ID of the subscription. | [default to null]

### Return type

[**SubscriptionContainer**](../Models/SubscriptionContainer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getSubscriptionRecords"></a>
# **getSubscriptionRecords**
> List getSubscriptionRecords()



### Parameters
This endpoint does not need any parameter.

### Return type

[**List**](../Models/Subscription.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getTransactions"></a>
# **getTransactions**
> TransactionList getTransactions(accountId, fromTimestamp, toTimestamp)



### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **accountId** | **String**| The account whose transactions we want | [default to null]
 **fromTimestamp** | **Date**| The timestamp from which to start searching | [optional] [default to null]
 **toTimestamp** | **Date**| The timestamp from which to end searching | [optional] [default to null]

### Return type

[**TransactionList**](../Models/TransactionList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

