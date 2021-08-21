# Documentation for Schnooty API

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountManagementApi* | [**accountsIdPut**](Apis/AccountManagementApi.md#accountsidput) | **PUT** /accounts/{id} | 
*AccountManagementApi* | [**createAccount**](Apis/AccountManagementApi.md#createaccount) | **POST** /accounts | 
*AccountManagementApi* | [**createSubscription**](Apis/AccountManagementApi.md#createsubscription) | **POST** /subscriptions | 
*AccountManagementApi* | [**getAccountBalance**](Apis/AccountManagementApi.md#getaccountbalance) | **GET** /balances | 
*AccountManagementApi* | [**getAccountById**](Apis/AccountManagementApi.md#getaccountbyid) | **GET** /accounts/{id} | 
*AccountManagementApi* | [**getAccounts**](Apis/AccountManagementApi.md#getaccounts) | **GET** /accounts | 
*AccountManagementApi* | [**getPlans**](Apis/AccountManagementApi.md#getplans) | **GET** /plans | 
*AccountManagementApi* | [**getSubscriptionById**](Apis/AccountManagementApi.md#getsubscriptionbyid) | **GET** /subscriptions/{id} | 
*AccountManagementApi* | [**getSubscriptionRecords**](Apis/AccountManagementApi.md#getsubscriptionrecords) | **GET** /subscriptions | 
*AccountManagementApi* | [**getTransactions**](Apis/AccountManagementApi.md#gettransactions) | **GET** /transactions | 
*AgentSessionsApi* | [**createAgentSession**](Apis/AgentSessionsApi.md#createagentsession) | **POST** /session/{groupName} | 
*AgentSessionsApi* | [**getAgentSessionState**](Apis/AgentSessionsApi.md#getagentsessionstate) | **GET** /session/{groupName} | 
*AgentsApi* | [**agentsGet**](Apis/AgentsApi.md#agentsget) | **GET** /agents | 
*AgentsApi* | [**agentsIdDelete**](Apis/AgentsApi.md#agentsiddelete) | **DELETE** /agents/{id} | 
*AgentsApi* | [**agentsIdGet**](Apis/AgentsApi.md#agentsidget) | **GET** /agents/{id} | 
*AgentsApi* | [**agentsIdPut**](Apis/AgentsApi.md#agentsidput) | **PUT** /agents/{id} | 
*AgentsApi* | [**agentsPost**](Apis/AgentsApi.md#agentspost) | **POST** /agents | 
*AlertsApi* | [**alertsGet**](Apis/AlertsApi.md#alertsget) | **GET** /alerts | Retrieve all of the alerts you have created in your account.
*AlertsApi* | [**alertsIdDelete**](Apis/AlertsApi.md#alertsiddelete) | **DELETE** /alerts/{id} | 
*AlertsApi* | [**alertsIdGet**](Apis/AlertsApi.md#alertsidget) | **GET** /alerts/{id} | 
*AlertsApi* | [**alertsIdPut**](Apis/AlertsApi.md#alertsidput) | **PUT** /alerts/{id} | 
*AlertsApi* | [**alertsPost**](Apis/AlertsApi.md#alertspost) | **POST** /alerts | Create a new alert that can be activated when a monitor goes down. An alert can be a message (email, Microsoft teams), or it can call a web service (webhook). An alert is associated with zero or more of your monitors. If your alert has more than one monitor, the alert will be activated only when the number of monitors that goes down exceededs the `threshold`.
*AuthenticationApi* | [**authenticationJwtPost**](Apis/AuthenticationApi.md#authenticationjwtpost) | **POST** /authentication/jwt | Create an API token in the form of a JWT.
*ChallengeApi* | [**createChallenge**](Apis/ChallengeApi.md#createchallenge) | **POST** /challenge | Create a challenge to prove you are human
*ChallengeApi* | [**updateChallenge**](Apis/ChallengeApi.md#updatechallenge) | **POST** /challenge/{id} | Solve a challenge and prove you are human.
*InfoApi* | [**getInfo**](Apis/InfoApi.md#getinfo) | **GET** /info | 
*MonitorsApi* | [**getMonitorById**](Apis/MonitorsApi.md#getmonitorbyid) | **GET** /monitors/{id} | Get a monitor by ID
*MonitorsApi* | [**getMonitors**](Apis/MonitorsApi.md#getmonitors) | **GET** /monitors | Get a list of all monitors in your account.
*MonitorsApi* | [**monitorsIdDelete**](Apis/MonitorsApi.md#monitorsiddelete) | **DELETE** /monitors/{id} | 
*MonitorsApi* | [**postMonitor**](Apis/MonitorsApi.md#postmonitor) | **POST** /monitors | Create a new monitor
*MonitorsApi* | [**updateMonitor**](Apis/MonitorsApi.md#updatemonitor) | **PUT** /monitors/{id} | Update an existing monitor by ID
*RegistrationApi* | [**confirmRegistration**](Apis/RegistrationApi.md#confirmregistration) | **POST** /registration/{id} | Confirm registration of account.
*RegistrationApi* | [**createRegistration**](Apis/RegistrationApi.md#createregistration) | **POST** /registration | Register your email address and password.
*StatusesApi* | [**getMonitorStatuses**](Apis/StatusesApi.md#getmonitorstatuses) | **GET** /statuses | 
*StatusesApi* | [**updateMonitorStatuses**](Apis/StatusesApi.md#updatemonitorstatuses) | **POST** /statuses | 


<a name="documentation-for-models"></a>
## Documentation for Models

 - [Account](./Models/Account.md)
 - [AccountType](./Models/AccountType.md)
 - [Agent](./Models/Agent.md)
 - [AgentSessionAssignment](./Models/AgentSessionAssignment.md)
 - [AgentSessionRequest](./Models/AgentSessionRequest.md)
 - [AgentSessionState](./Models/AgentSessionState.md)
 - [Alert](./Models/Alert.md)
 - [AlertArray](./Models/AlertArray.md)
 - [AlertBody](./Models/AlertBody.md)
 - [Balance](./Models/Balance.md)
 - [Balances](./Models/Balances.md)
 - [ChargeType](./Models/ChargeType.md)
 - [CmpOperator](./Models/CmpOperator.md)
 - [CurrencyCode](./Models/CurrencyCode.md)
 - [EmailAlertBody](./Models/EmailAlertBody.md)
 - [ErrorCode](./Models/ErrorCode.md)
 - [ErrorList](./Models/ErrorList.md)
 - [FieldConstraint](./Models/FieldConstraint.md)
 - [HttpHeader](./Models/HttpHeader.md)
 - [HttpMonitorBody](./Models/HttpMonitorBody.md)
 - [InlineResponse200](./Models/InlineResponse200.md)
 - [InlineResponse2001](./Models/InlineResponse2001.md)
 - [InlineResponse2002](./Models/InlineResponse2002.md)
 - [JwtObject](./Models/JwtObject.md)
 - [Money](./Models/Money.md)
 - [Monitor](./Models/Monitor.md)
 - [MonitorArray](./Models/MonitorArray.md)
 - [MonitorBody](./Models/MonitorBody.md)
 - [MonitorStatus](./Models/MonitorStatus.md)
 - [MonitorStatusArray](./Models/MonitorStatusArray.md)
 - [MonitorStatusIndicator](./Models/MonitorStatusIndicator.md)
 - [MonitorStatusLogEntry](./Models/MonitorStatusLogEntry.md)
 - [MonitorStatusResult](./Models/MonitorStatusResult.md)
 - [MonitorType](./Models/MonitorType.md)
 - [MsTeamsAlertBody](./Models/MsTeamsAlertBody.md)
 - [NewAgent](./Models/NewAgent.md)
 - [Plan](./Models/Plan.md)
 - [PlanArray](./Models/PlanArray.md)
 - [ProcessMonitorBody](./Models/ProcessMonitorBody.md)
 - [Product](./Models/Product.md)
 - [ProductEntity](./Models/ProductEntity.md)
 - [RedisMonitorBody](./Models/RedisMonitorBody.md)
 - [Registration](./Models/Registration.md)
 - [RegistrationChallenge](./Models/RegistrationChallenge.md)
 - [RegistrationConfirmation](./Models/RegistrationConfirmation.md)
 - [ResponseError](./Models/ResponseError.md)
 - [ServerInfo](./Models/ServerInfo.md)
 - [Subscription](./Models/Subscription.md)
 - [SubscriptionContainer](./Models/SubscriptionContainer.md)
 - [SubscriptionStatus](./Models/SubscriptionStatus.md)
 - [TcpMonitorBody](./Models/TcpMonitorBody.md)
 - [TlsMode](./Models/TlsMode.md)
 - [Transaction](./Models/Transaction.md)
 - [TransactionCode](./Models/TransactionCode.md)
 - [TransactionList](./Models/TransactionList.md)
 - [WebhookAlertBody](./Models/WebhookAlertBody.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

<a name="BasicAuth"></a>
### BasicAuth

- **Type**: HTTP basic authentication

<a name="BearerAuth"></a>
### BearerAuth

- **Type**: HTTP basic authentication

