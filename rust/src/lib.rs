#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "1.0.0";

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AccountsIdPutResponse {
    /// OK
    OK
    (models::InlineResponse2001)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateAccountResponse {
    /// OK
    OK
    (models::InlineResponse2001)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateSubscriptionResponse {
    /// OK
    OK
    (models::SubscriptionContainer)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetAccountBalanceResponse {
    /// OK
    OK
    (models::InlineResponse2002)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetAccountByIdResponse {
    /// OK
    OK
    (models::InlineResponse2001)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetAccountsResponse {
    /// OK
    OK
    (models::InlineResponse200)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetPlansResponse {
    /// OK
    OK
    (models::PlanArray)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetSubscriptionByIdResponse {
    /// OK
    OK
    (models::SubscriptionContainer)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetSubscriptionRecordsResponse {
    /// OK
    OK
    (Vec<models::Subscription>)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetTransactionsResponse {
    /// OK
    OK
    (models::TransactionList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateAgentSessionResponse {
    /// OK
    OK
    (models::AgentSessionState)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetAgentSessionStateResponse {
    /// OK
    OK
    (models::AgentSessionState)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AgentsGetResponse {
    /// OK
    OK
    (Vec<models::Agent>)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AgentsIdDeleteResponse {
    /// OK
    OK
    (models::Agent)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AgentsIdGetResponse {
    /// OK
    OK
    (models::Agent)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AgentsIdPutResponse {
    /// OK
    OK
    (models::Agent)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AgentsPostResponse {
    /// OK
    OK
    (models::Agent)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// Forbidden
    Forbidden
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AlertsGetResponse {
    /// OK
    OK
    (models::AlertArray)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AlertsIdDeleteResponse {
    /// OK
    OK
    (models::Alert)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AlertsIdGetResponse {
    /// OK
    OK
    (models::Alert)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AlertsIdPutResponse {
    /// OK
    OK
    (models::Alert)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AlertsPostResponse {
    /// OK
    OK
    (models::Alert)
    ,
    /// NoContent
    NoContent
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AuthenticationJwtPostResponse {
    /// OK
    OK
    (models::JwtObject)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateChallengeResponse {
    /// OK
    OK
    (models::RegistrationChallenge)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateChallengeResponse {
    /// OK
    OK
    (models::RegistrationChallenge)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetInfoResponse {
    /// OK
    OK
    (models::ServerInfo)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetMonitorByIdResponse {
    /// OK
    OK
    (models::Monitor)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetMonitorsResponse {
    /// OK
    OK
    (models::MonitorArray)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum MonitorsIdDeleteResponse {
    /// OK
    OK
    (models::Monitor)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum PostMonitorResponse {
    /// Created
    Created
    (models::Monitor)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateMonitorResponse {
    /// OK
    OK
    (models::Monitor)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum ConfirmRegistrationResponse {
    /// The registration was confirmed. You can now authenticate using the email address and password.
    TheRegistrationWasConfirmed
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateRegistrationResponse {
    /// OK
    OK
    (models::Registration)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetMonitorStatusesResponse {
    /// OK
    OK
    (models::MonitorStatusArray)
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateMonitorStatusesResponse {
    /// NoContent
    NoContent
    ,
    /// BadRequest
    BadRequest
    (models::ErrorList)
    ,
    /// Unauthorized
    Unauthorized
    (models::ErrorList)
    ,
    /// UnprocessableEntity
    UnprocessableEntity
    (models::ErrorList)
    ,
    /// NotFound
    NotFound
    (models::ErrorList)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    async fn accounts_id_put(
        &self,
        id: String,
        account: models::Account,
        context: &C) -> Result<AccountsIdPutResponse, ApiError>;

    async fn create_account(
        &self,
        account: models::Account,
        context: &C) -> Result<CreateAccountResponse, ApiError>;

    async fn create_subscription(
        &self,
        subscription: models::Subscription,
        context: &C) -> Result<CreateSubscriptionResponse, ApiError>;

    async fn get_account_balance(
        &self,
        account_id: String,
        context: &C) -> Result<GetAccountBalanceResponse, ApiError>;

    async fn get_account_by_id(
        &self,
        id: String,
        context: &C) -> Result<GetAccountByIdResponse, ApiError>;

    async fn get_accounts(
        &self,
        context: &C) -> Result<GetAccountsResponse, ApiError>;

    async fn get_plans(
        &self,
        context: &C) -> Result<GetPlansResponse, ApiError>;

    async fn get_subscription_by_id(
        &self,
        id: String,
        context: &C) -> Result<GetSubscriptionByIdResponse, ApiError>;

    async fn get_subscription_records(
        &self,
        context: &C) -> Result<GetSubscriptionRecordsResponse, ApiError>;

    async fn get_transactions(
        &self,
        account_id: String,
        from_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        to_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<GetTransactionsResponse, ApiError>;

    async fn create_agent_session(
        &self,
        group_name: String,
        agent_session_request: models::AgentSessionRequest,
        context: &C) -> Result<CreateAgentSessionResponse, ApiError>;

    async fn get_agent_session_state(
        &self,
        group_name: String,
        context: &C) -> Result<GetAgentSessionStateResponse, ApiError>;

    async fn agents_get(
        &self,
        context: &C) -> Result<AgentsGetResponse, ApiError>;

    async fn agents_id_delete(
        &self,
        id: String,
        context: &C) -> Result<AgentsIdDeleteResponse, ApiError>;

    async fn agents_id_get(
        &self,
        id: String,
        context: &C) -> Result<AgentsIdGetResponse, ApiError>;

    async fn agents_id_put(
        &self,
        id: String,
        agent: models::Agent,
        context: &C) -> Result<AgentsIdPutResponse, ApiError>;

    async fn agents_post(
        &self,
        agent: models::Agent,
        context: &C) -> Result<AgentsPostResponse, ApiError>;

    async fn alerts_get(
        &self,
        context: &C) -> Result<AlertsGetResponse, ApiError>;

    async fn alerts_id_delete(
        &self,
        id: String,
        context: &C) -> Result<AlertsIdDeleteResponse, ApiError>;

    async fn alerts_id_get(
        &self,
        id: String,
        context: &C) -> Result<AlertsIdGetResponse, ApiError>;

    async fn alerts_id_put(
        &self,
        id: String,
        alert: models::Alert,
        context: &C) -> Result<AlertsIdPutResponse, ApiError>;

    async fn alerts_post(
        &self,
        alert: models::Alert,
        context: &C) -> Result<AlertsPostResponse, ApiError>;

    /// Create an API token in the form of a JWT.
    async fn authentication_jwt_post(
        &self,
        context: &C) -> Result<AuthenticationJwtPostResponse, ApiError>;

    /// Create a challenge to prove you are human
    async fn create_challenge(
        &self,
        context: &C) -> Result<CreateChallengeResponse, ApiError>;

    /// Solve a challenge and prove you are human.
    async fn update_challenge(
        &self,
        id: String,
        context: &C) -> Result<UpdateChallengeResponse, ApiError>;

    async fn get_info(
        &self,
        context: &C) -> Result<GetInfoResponse, ApiError>;

    /// Get a monitor by ID
    async fn get_monitor_by_id(
        &self,
        id: String,
        context: &C) -> Result<GetMonitorByIdResponse, ApiError>;

    /// Get a list of all monitors in your account.
    async fn get_monitors(
        &self,
        context: &C) -> Result<GetMonitorsResponse, ApiError>;

    async fn monitors_id_delete(
        &self,
        id: String,
        context: &C) -> Result<MonitorsIdDeleteResponse, ApiError>;

    /// Create a new monitor
    async fn post_monitor(
        &self,
        monitor: models::Monitor,
        context: &C) -> Result<PostMonitorResponse, ApiError>;

    /// Update an existing monitor by ID
    async fn update_monitor(
        &self,
        id: String,
        monitor: models::Monitor,
        context: &C) -> Result<UpdateMonitorResponse, ApiError>;

    /// Confirm registration of Open Monitors account.
    async fn confirm_registration(
        &self,
        id: String,
        registration_confirmation: models::RegistrationConfirmation,
        context: &C) -> Result<ConfirmRegistrationResponse, ApiError>;

    /// Register your email address and password.
    async fn create_registration(
        &self,
        registration: models::Registration,
        context: &C) -> Result<CreateRegistrationResponse, ApiError>;

    async fn get_monitor_statuses(
        &self,
        context: &C) -> Result<GetMonitorStatusesResponse, ApiError>;

    async fn update_monitor_statuses(
        &self,
        monitor_status_array: models::MonitorStatusArray,
        context: &C) -> Result<UpdateMonitorStatusesResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    async fn accounts_id_put(
        &self,
        id: String,
        account: models::Account,
        ) -> Result<AccountsIdPutResponse, ApiError>;

    async fn create_account(
        &self,
        account: models::Account,
        ) -> Result<CreateAccountResponse, ApiError>;

    async fn create_subscription(
        &self,
        subscription: models::Subscription,
        ) -> Result<CreateSubscriptionResponse, ApiError>;

    async fn get_account_balance(
        &self,
        account_id: String,
        ) -> Result<GetAccountBalanceResponse, ApiError>;

    async fn get_account_by_id(
        &self,
        id: String,
        ) -> Result<GetAccountByIdResponse, ApiError>;

    async fn get_accounts(
        &self,
        ) -> Result<GetAccountsResponse, ApiError>;

    async fn get_plans(
        &self,
        ) -> Result<GetPlansResponse, ApiError>;

    async fn get_subscription_by_id(
        &self,
        id: String,
        ) -> Result<GetSubscriptionByIdResponse, ApiError>;

    async fn get_subscription_records(
        &self,
        ) -> Result<GetSubscriptionRecordsResponse, ApiError>;

    async fn get_transactions(
        &self,
        account_id: String,
        from_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        to_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        ) -> Result<GetTransactionsResponse, ApiError>;

    async fn create_agent_session(
        &self,
        group_name: String,
        agent_session_request: models::AgentSessionRequest,
        ) -> Result<CreateAgentSessionResponse, ApiError>;

    async fn get_agent_session_state(
        &self,
        group_name: String,
        ) -> Result<GetAgentSessionStateResponse, ApiError>;

    async fn agents_get(
        &self,
        ) -> Result<AgentsGetResponse, ApiError>;

    async fn agents_id_delete(
        &self,
        id: String,
        ) -> Result<AgentsIdDeleteResponse, ApiError>;

    async fn agents_id_get(
        &self,
        id: String,
        ) -> Result<AgentsIdGetResponse, ApiError>;

    async fn agents_id_put(
        &self,
        id: String,
        agent: models::Agent,
        ) -> Result<AgentsIdPutResponse, ApiError>;

    async fn agents_post(
        &self,
        agent: models::Agent,
        ) -> Result<AgentsPostResponse, ApiError>;

    async fn alerts_get(
        &self,
        ) -> Result<AlertsGetResponse, ApiError>;

    async fn alerts_id_delete(
        &self,
        id: String,
        ) -> Result<AlertsIdDeleteResponse, ApiError>;

    async fn alerts_id_get(
        &self,
        id: String,
        ) -> Result<AlertsIdGetResponse, ApiError>;

    async fn alerts_id_put(
        &self,
        id: String,
        alert: models::Alert,
        ) -> Result<AlertsIdPutResponse, ApiError>;

    async fn alerts_post(
        &self,
        alert: models::Alert,
        ) -> Result<AlertsPostResponse, ApiError>;

    /// Create an API token in the form of a JWT.
    async fn authentication_jwt_post(
        &self,
        ) -> Result<AuthenticationJwtPostResponse, ApiError>;

    /// Create a challenge to prove you are human
    async fn create_challenge(
        &self,
        ) -> Result<CreateChallengeResponse, ApiError>;

    /// Solve a challenge and prove you are human.
    async fn update_challenge(
        &self,
        id: String,
        ) -> Result<UpdateChallengeResponse, ApiError>;

    async fn get_info(
        &self,
        ) -> Result<GetInfoResponse, ApiError>;

    /// Get a monitor by ID
    async fn get_monitor_by_id(
        &self,
        id: String,
        ) -> Result<GetMonitorByIdResponse, ApiError>;

    /// Get a list of all monitors in your account.
    async fn get_monitors(
        &self,
        ) -> Result<GetMonitorsResponse, ApiError>;

    async fn monitors_id_delete(
        &self,
        id: String,
        ) -> Result<MonitorsIdDeleteResponse, ApiError>;

    /// Create a new monitor
    async fn post_monitor(
        &self,
        monitor: models::Monitor,
        ) -> Result<PostMonitorResponse, ApiError>;

    /// Update an existing monitor by ID
    async fn update_monitor(
        &self,
        id: String,
        monitor: models::Monitor,
        ) -> Result<UpdateMonitorResponse, ApiError>;

    /// Confirm registration of Open Monitors account.
    async fn confirm_registration(
        &self,
        id: String,
        registration_confirmation: models::RegistrationConfirmation,
        ) -> Result<ConfirmRegistrationResponse, ApiError>;

    /// Register your email address and password.
    async fn create_registration(
        &self,
        registration: models::Registration,
        ) -> Result<CreateRegistrationResponse, ApiError>;

    async fn get_monitor_statuses(
        &self,
        ) -> Result<GetMonitorStatusesResponse, ApiError>;

    async fn update_monitor_statuses(
        &self,
        monitor_status_array: models::MonitorStatusArray,
        ) -> Result<UpdateMonitorStatusesResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    async fn accounts_id_put(
        &self,
        id: String,
        account: models::Account,
        ) -> Result<AccountsIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().accounts_id_put(id, account, &context).await
    }

    async fn create_account(
        &self,
        account: models::Account,
        ) -> Result<CreateAccountResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_account(account, &context).await
    }

    async fn create_subscription(
        &self,
        subscription: models::Subscription,
        ) -> Result<CreateSubscriptionResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_subscription(subscription, &context).await
    }

    async fn get_account_balance(
        &self,
        account_id: String,
        ) -> Result<GetAccountBalanceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_account_balance(account_id, &context).await
    }

    async fn get_account_by_id(
        &self,
        id: String,
        ) -> Result<GetAccountByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_account_by_id(id, &context).await
    }

    async fn get_accounts(
        &self,
        ) -> Result<GetAccountsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_accounts(&context).await
    }

    async fn get_plans(
        &self,
        ) -> Result<GetPlansResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_plans(&context).await
    }

    async fn get_subscription_by_id(
        &self,
        id: String,
        ) -> Result<GetSubscriptionByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_subscription_by_id(id, &context).await
    }

    async fn get_subscription_records(
        &self,
        ) -> Result<GetSubscriptionRecordsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_subscription_records(&context).await
    }

    async fn get_transactions(
        &self,
        account_id: String,
        from_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        to_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        ) -> Result<GetTransactionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_transactions(account_id, from_timestamp, to_timestamp, &context).await
    }

    async fn create_agent_session(
        &self,
        group_name: String,
        agent_session_request: models::AgentSessionRequest,
        ) -> Result<CreateAgentSessionResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_agent_session(group_name, agent_session_request, &context).await
    }

    async fn get_agent_session_state(
        &self,
        group_name: String,
        ) -> Result<GetAgentSessionStateResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_agent_session_state(group_name, &context).await
    }

    async fn agents_get(
        &self,
        ) -> Result<AgentsGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().agents_get(&context).await
    }

    async fn agents_id_delete(
        &self,
        id: String,
        ) -> Result<AgentsIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().agents_id_delete(id, &context).await
    }

    async fn agents_id_get(
        &self,
        id: String,
        ) -> Result<AgentsIdGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().agents_id_get(id, &context).await
    }

    async fn agents_id_put(
        &self,
        id: String,
        agent: models::Agent,
        ) -> Result<AgentsIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().agents_id_put(id, agent, &context).await
    }

    async fn agents_post(
        &self,
        agent: models::Agent,
        ) -> Result<AgentsPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().agents_post(agent, &context).await
    }

    async fn alerts_get(
        &self,
        ) -> Result<AlertsGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().alerts_get(&context).await
    }

    async fn alerts_id_delete(
        &self,
        id: String,
        ) -> Result<AlertsIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().alerts_id_delete(id, &context).await
    }

    async fn alerts_id_get(
        &self,
        id: String,
        ) -> Result<AlertsIdGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().alerts_id_get(id, &context).await
    }

    async fn alerts_id_put(
        &self,
        id: String,
        alert: models::Alert,
        ) -> Result<AlertsIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().alerts_id_put(id, alert, &context).await
    }

    async fn alerts_post(
        &self,
        alert: models::Alert,
        ) -> Result<AlertsPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().alerts_post(alert, &context).await
    }

    /// Create an API token in the form of a JWT.
    async fn authentication_jwt_post(
        &self,
        ) -> Result<AuthenticationJwtPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().authentication_jwt_post(&context).await
    }

    /// Create a challenge to prove you are human
    async fn create_challenge(
        &self,
        ) -> Result<CreateChallengeResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_challenge(&context).await
    }

    /// Solve a challenge and prove you are human.
    async fn update_challenge(
        &self,
        id: String,
        ) -> Result<UpdateChallengeResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_challenge(id, &context).await
    }

    async fn get_info(
        &self,
        ) -> Result<GetInfoResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_info(&context).await
    }

    /// Get a monitor by ID
    async fn get_monitor_by_id(
        &self,
        id: String,
        ) -> Result<GetMonitorByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_monitor_by_id(id, &context).await
    }

    /// Get a list of all monitors in your account.
    async fn get_monitors(
        &self,
        ) -> Result<GetMonitorsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_monitors(&context).await
    }

    async fn monitors_id_delete(
        &self,
        id: String,
        ) -> Result<MonitorsIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().monitors_id_delete(id, &context).await
    }

    /// Create a new monitor
    async fn post_monitor(
        &self,
        monitor: models::Monitor,
        ) -> Result<PostMonitorResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_monitor(monitor, &context).await
    }

    /// Update an existing monitor by ID
    async fn update_monitor(
        &self,
        id: String,
        monitor: models::Monitor,
        ) -> Result<UpdateMonitorResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_monitor(id, monitor, &context).await
    }

    /// Confirm registration of Open Monitors account.
    async fn confirm_registration(
        &self,
        id: String,
        registration_confirmation: models::RegistrationConfirmation,
        ) -> Result<ConfirmRegistrationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().confirm_registration(id, registration_confirmation, &context).await
    }

    /// Register your email address and password.
    async fn create_registration(
        &self,
        registration: models::Registration,
        ) -> Result<CreateRegistrationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_registration(registration, &context).await
    }

    async fn get_monitor_statuses(
        &self,
        ) -> Result<GetMonitorStatusesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_monitor_statuses(&context).await
    }

    async fn update_monitor_statuses(
        &self,
        monitor_status_array: models::MonitorStatusArray,
        ) -> Result<UpdateMonitorStatusesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_monitor_statuses(monitor_status_array, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
