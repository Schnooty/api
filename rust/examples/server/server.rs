//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
    AccountsIdPutResponse,
    CreateAccountResponse,
    CreateSubscriptionResponse,
    GetAccountBalanceResponse,
    GetAccountByIdResponse,
    GetAccountsResponse,
    GetPlansResponse,
    GetSubscriptionByIdResponse,
    GetSubscriptionRecordsResponse,
    GetTransactionsResponse,
    ClearSessionResponse,
    GetSessionResponse,
    GetSessionsResponse,
    PutSessionResponse,
    AgentsGetResponse,
    AgentsIdDeleteResponse,
    AgentsIdGetResponse,
    AgentsIdPutResponse,
    AgentsPostResponse,
    AlertsGetResponse,
    AlertsIdDeleteResponse,
    AlertsIdGetResponse,
    AlertsIdPutResponse,
    AlertsPostResponse,
    JwtPostResponse,
    CreateChallengeResponse,
    UpdateChallengeResponse,
    GetInfoResponse,
    ConfirmRegistrationResponse,
    CreateRegistrationResponse,
    ClearStatusResponse,
    GetMonitorStatusResponse,
    GetMonitorStatusesResponse,
    SetStatusResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn accounts_id_put(
        &self,
        id: String,
        account: models::Account,
        context: &C) -> Result<AccountsIdPutResponse, ApiError>
    {
        let context = context.clone();
        info!("accounts_id_put(\"{}\", {:?}) - X-Span-ID: {:?}", id, account, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn create_account(
        &self,
        account: models::Account,
        context: &C) -> Result<CreateAccountResponse, ApiError>
    {
        let context = context.clone();
        info!("create_account({:?}) - X-Span-ID: {:?}", account, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn create_subscription(
        &self,
        subscription: models::Subscription,
        context: &C) -> Result<CreateSubscriptionResponse, ApiError>
    {
        let context = context.clone();
        info!("create_subscription({:?}) - X-Span-ID: {:?}", subscription, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_account_balance(
        &self,
        account_id: String,
        context: &C) -> Result<GetAccountBalanceResponse, ApiError>
    {
        let context = context.clone();
        info!("get_account_balance(\"{}\") - X-Span-ID: {:?}", account_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_account_by_id(
        &self,
        id: String,
        context: &C) -> Result<GetAccountByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_account_by_id(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_accounts(
        &self,
        context: &C) -> Result<GetAccountsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_accounts() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_plans(
        &self,
        context: &C) -> Result<GetPlansResponse, ApiError>
    {
        let context = context.clone();
        info!("get_plans() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_subscription_by_id(
        &self,
        id: String,
        context: &C) -> Result<GetSubscriptionByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_subscription_by_id(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_subscription_records(
        &self,
        context: &C) -> Result<GetSubscriptionRecordsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_subscription_records() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_transactions(
        &self,
        account_id: String,
        from_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        to_timestamp: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<GetTransactionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_transactions(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", account_id, from_timestamp, to_timestamp, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn clear_session(
        &self,
        identifier: String,
        context: &C) -> Result<ClearSessionResponse, ApiError>
    {
        let context = context.clone();
        info!("clear_session(\"{}\") - X-Span-ID: {:?}", identifier, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_session(
        &self,
        identifier: String,
        context: &C) -> Result<GetSessionResponse, ApiError>
    {
        let context = context.clone();
        info!("get_session(\"{}\") - X-Span-ID: {:?}", identifier, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_sessions(
        &self,
        context: &C) -> Result<GetSessionsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_sessions() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn put_session(
        &self,
        identifier: String,
        session: models::Session,
        context: &C) -> Result<PutSessionResponse, ApiError>
    {
        let context = context.clone();
        info!("put_session(\"{}\", {:?}) - X-Span-ID: {:?}", identifier, session, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn agents_get(
        &self,
        context: &C) -> Result<AgentsGetResponse, ApiError>
    {
        let context = context.clone();
        info!("agents_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn agents_id_delete(
        &self,
        id: String,
        context: &C) -> Result<AgentsIdDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("agents_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn agents_id_get(
        &self,
        id: String,
        context: &C) -> Result<AgentsIdGetResponse, ApiError>
    {
        let context = context.clone();
        info!("agents_id_get(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn agents_id_put(
        &self,
        id: String,
        agent: models::Agent,
        context: &C) -> Result<AgentsIdPutResponse, ApiError>
    {
        let context = context.clone();
        info!("agents_id_put(\"{}\", {:?}) - X-Span-ID: {:?}", id, agent, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn agents_post(
        &self,
        agent: models::Agent,
        context: &C) -> Result<AgentsPostResponse, ApiError>
    {
        let context = context.clone();
        info!("agents_post({:?}) - X-Span-ID: {:?}", agent, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn alerts_get(
        &self,
        context: &C) -> Result<AlertsGetResponse, ApiError>
    {
        let context = context.clone();
        info!("alerts_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn alerts_id_delete(
        &self,
        id: String,
        context: &C) -> Result<AlertsIdDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("alerts_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn alerts_id_get(
        &self,
        id: String,
        context: &C) -> Result<AlertsIdGetResponse, ApiError>
    {
        let context = context.clone();
        info!("alerts_id_get(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn alerts_id_put(
        &self,
        id: String,
        alert: models::Alert,
        context: &C) -> Result<AlertsIdPutResponse, ApiError>
    {
        let context = context.clone();
        info!("alerts_id_put(\"{}\", {:?}) - X-Span-ID: {:?}", id, alert, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn alerts_post(
        &self,
        alert: models::Alert,
        context: &C) -> Result<AlertsPostResponse, ApiError>
    {
        let context = context.clone();
        info!("alerts_post({:?}) - X-Span-ID: {:?}", alert, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create an API token in the form of a JWT.
    async fn jwt_post(
        &self,
        context: &C) -> Result<JwtPostResponse, ApiError>
    {
        let context = context.clone();
        info!("jwt_post() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create a challenge to prove you are human
    async fn create_challenge(
        &self,
        context: &C) -> Result<CreateChallengeResponse, ApiError>
    {
        let context = context.clone();
        info!("create_challenge() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Solve a challenge and prove you are human.
    async fn update_challenge(
        &self,
        id: String,
        context: &C) -> Result<UpdateChallengeResponse, ApiError>
    {
        let context = context.clone();
        info!("update_challenge(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_info(
        &self,
        context: &C) -> Result<GetInfoResponse, ApiError>
    {
        let context = context.clone();
        info!("get_info() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Confirm registration of account.
    async fn confirm_registration(
        &self,
        id: String,
        registration_confirmation: models::RegistrationConfirmation,
        context: &C) -> Result<ConfirmRegistrationResponse, ApiError>
    {
        let context = context.clone();
        info!("confirm_registration(\"{}\", {:?}) - X-Span-ID: {:?}", id, registration_confirmation, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Register your email address and password.
    async fn create_registration(
        &self,
        registration: models::Registration,
        context: &C) -> Result<CreateRegistrationResponse, ApiError>
    {
        let context = context.clone();
        info!("create_registration({:?}) - X-Span-ID: {:?}", registration, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn clear_status(
        &self,
        status_id: String,
        context: &C) -> Result<ClearStatusResponse, ApiError>
    {
        let context = context.clone();
        info!("clear_status(\"{}\") - X-Span-ID: {:?}", status_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_monitor_status(
        &self,
        status_id: String,
        context: &C) -> Result<GetMonitorStatusResponse, ApiError>
    {
        let context = context.clone();
        info!("get_monitor_status(\"{}\") - X-Span-ID: {:?}", status_id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_monitor_statuses(
        &self,
        context: &C) -> Result<GetMonitorStatusesResponse, ApiError>
    {
        let context = context.clone();
        info!("get_monitor_statuses() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn set_status(
        &self,
        status_id: String,
        monitor_status: models::MonitorStatus,
        context: &C) -> Result<SetStatusResponse, ApiError>
    {
        let context = context.clone();
        info!("set_status(\"{}\", {:?}) - X-Span-ID: {:?}", status_id, monitor_status, context.get().0.clone());
        Err("Generic failuare".into())
    }

}
