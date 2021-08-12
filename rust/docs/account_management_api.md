# account_management_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
****](account_management_api.md#) | **PUT** /accounts/{id} | 
**create_account**](account_management_api.md#create_account) | **POST** /accounts | 
**create_subscription**](account_management_api.md#create_subscription) | **POST** /subscriptions | 
**get_account_balance**](account_management_api.md#get_account_balance) | **GET** /balances | 
**get_account_by_id**](account_management_api.md#get_account_by_id) | **GET** /accounts/{id} | 
**get_accounts**](account_management_api.md#get_accounts) | **GET** /accounts | 
**get_plans**](account_management_api.md#get_plans) | **GET** /plans | 
**get_subscription_by_id**](account_management_api.md#get_subscription_by_id) | **GET** /subscriptions/{id} | 
**get_subscription_records**](account_management_api.md#get_subscription_records) | **GET** /subscriptions | 
**get_transactions**](account_management_api.md#get_transactions) | **GET** /transactions | 


# ****
> models::InlineResponse2001 (id, account)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of the account. | 
  **account** | [**Account**](Account.md)| Account to be created | 

### Return type

[**models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_account**
> models::InlineResponse2001 create_account(account)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account** | [**Account**](Account.md)| Account to be created | 

### Return type

[**models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_subscription**
> models::SubscriptionContainer create_subscription(subscription)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **subscription** | [**Subscription**](Subscription.md)| Subscription to be created | 

### Return type

[**models::SubscriptionContainer**](SubscriptionContainer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_balance**
> models::InlineResponse2002 get_account_balance(account_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| The account whose current balance we want | 

### Return type

[**models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_by_id**
> models::InlineResponse2001 get_account_by_id(id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of the account. | 

### Return type

[**models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_accounts**
> models::InlineResponse200 get_accounts()


### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_plans**
> models::PlanArray get_plans()


### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::PlanArray**](PlanArray.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_subscription_by_id**
> models::SubscriptionContainer get_subscription_by_id(id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| ID of the subscription. | 

### Return type

[**models::SubscriptionContainer**](SubscriptionContainer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_subscription_records**
> Vec<models::Subscription> get_subscription_records()


### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::Subscription>**](Subscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transactions**
> models::TransactionList get_transactions(account_id, optional)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **account_id** | **String**| The account whose transactions we want | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **account_id** | **String**| The account whose transactions we want | 
 **from_timestamp** | **chrono::DateTime::<chrono::Utc>**| The timestamp from which to start searching | 
 **to_timestamp** | **chrono::DateTime::<chrono::Utc>**| The timestamp from which to end searching | 

### Return type

[**models::TransactionList**](TransactionList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

