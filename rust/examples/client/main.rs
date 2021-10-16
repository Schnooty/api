#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use openapi_client::{Api, ApiNoContext, Client, ContextWrapperExt, models,
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
                      JwtGetResponse,
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
use clap::{App, Arg};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "GetAccountBalance",
                "GetAccountById",
                "GetAccounts",
                "GetPlans",
                "GetSubscriptionById",
                "GetSubscriptionRecords",
                "GetTransactions",
                "ClearSession",
                "GetSession",
                "GetSessions",
                "AgentsGet",
                "AgentsIdDelete",
                "AgentsIdGet",
                "AlertsGet",
                "AlertsIdDelete",
                "AlertsIdGet",
                "JwtGet",
                "CreateChallenge",
                "UpdateChallenge",
                "GetInfo",
                "ClearStatus",
                "GetMonitorStatus",
                "GetMonitorStatuses",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        /* Disabled because there's no example.
        Some("AccountsIdPut") => {
            let result = rt.block_on(client.accounts_id_put(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateAccount") => {
            let result = rt.block_on(client.create_account(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateSubscription") => {
            let result = rt.block_on(client.create_subscription(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("GetAccountBalance") => {
            let result = rt.block_on(client.get_account_balance(
                  "account_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAccountById") => {
            let result = rt.block_on(client.get_account_by_id(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetAccounts") => {
            let result = rt.block_on(client.get_accounts(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetPlans") => {
            let result = rt.block_on(client.get_plans(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSubscriptionById") => {
            let result = rt.block_on(client.get_subscription_by_id(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSubscriptionRecords") => {
            let result = rt.block_on(client.get_subscription_records(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetTransactions") => {
            let result = rt.block_on(client.get_transactions(
                  "account_id_example".to_string(),
                  None,
                  None
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("ClearSession") => {
            let result = rt.block_on(client.clear_session(
                  "identifier_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSession") => {
            let result = rt.block_on(client.get_session(
                  "identifier_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetSessions") => {
            let result = rt.block_on(client.get_sessions(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("PutSession") => {
            let result = rt.block_on(client.put_session(
                  "identifier_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("AgentsGet") => {
            let result = rt.block_on(client.agents_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AgentsIdDelete") => {
            let result = rt.block_on(client.agents_id_delete(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AgentsIdGet") => {
            let result = rt.block_on(client.agents_id_get(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("AgentsIdPut") => {
            let result = rt.block_on(client.agents_id_put(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("AgentsPost") => {
            let result = rt.block_on(client.agents_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("AlertsGet") => {
            let result = rt.block_on(client.alerts_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AlertsIdDelete") => {
            let result = rt.block_on(client.alerts_id_delete(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AlertsIdGet") => {
            let result = rt.block_on(client.alerts_id_get(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("AlertsIdPut") => {
            let result = rt.block_on(client.alerts_id_put(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("AlertsPost") => {
            let result = rt.block_on(client.alerts_post(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("JwtGet") => {
            let result = rt.block_on(client.jwt_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("CreateChallenge") => {
            let result = rt.block_on(client.create_challenge(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("UpdateChallenge") => {
            let result = rt.block_on(client.update_challenge(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetInfo") => {
            let result = rt.block_on(client.get_info(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("ConfirmRegistration") => {
            let result = rt.block_on(client.confirm_registration(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateRegistration") => {
            let result = rt.block_on(client.create_registration(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("ClearStatus") => {
            let result = rt.block_on(client.clear_status(
                  "status_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetMonitorStatus") => {
            let result = rt.block_on(client.get_monitor_status(
                  "status_id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GetMonitorStatuses") => {
            let result = rt.block_on(client.get_monitor_statuses(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("SetStatus") => {
            let result = rt.block_on(client.set_status(
                  "status_id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
