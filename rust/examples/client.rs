#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate openapi_client;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
#[macro_use]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate clap;
extern crate tokio_core;
extern crate uuid;

use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use openapi_client::{ApiNoContext, ContextWrapperExt,
                      ApiError,
                      CreateAgentSessionResponse,
                      GetAgentSessionStateResponse,
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
                      AuthenticationJwtPostResponse,
                      GetMonitorByIdResponse,
                      GetMonitorsResponse,
                      MonitorsIdDeleteResponse,
                      PostMonitorResponse,
                      UpdateMonitorResponse,
                      CreateRegistrationResponse,
                      RegistrationIdPostResponse,
                      GetMonitorStatusesResponse,
                      UpdateMonitorStatusesResponse
                     };
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "GetAgentSessionState",
    "AgentsGet",
    "AgentsIdDelete",
    "AgentsIdGet",
    "AlertsGet",
    "AlertsIdDelete",
    "AlertsIdGet",
    "AuthenticationJwtPost",
    "GetMonitorById",
    "GetMonitors",
    "MonitorsIdDelete",
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
            .default_value("80")
            .help("Port to contact"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        openapi_client::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        openapi_client::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString(self::uuid::Uuid::new_v4().to_string()));
    let client = client.with_context(context);

    match matches.value_of("operation") {

        // Disabled because there's no example.
        // Some("CreateAgentSession") => {
        //     let result = core.run(client.create_agent_session("group_name_example".to_string(), ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        Some("GetAgentSessionState") => {
            let result = core.run(client.get_agent_session_state("group_name_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("AgentsGet") => {
            let result = core.run(client.agents_get());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("AgentsIdDelete") => {
            let result = core.run(client.agents_id_delete("id_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("AgentsIdGet") => {
            let result = core.run(client.agents_id_get("id_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("AgentsIdPut") => {
        //     let result = core.run(client.agents_id_put("id_example".to_string(), ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("AgentsPost") => {
        //     let result = core.run(client.agents_post(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        Some("AlertsGet") => {
            let result = core.run(client.alerts_get());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("AlertsIdDelete") => {
            let result = core.run(client.alerts_id_delete("id_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("AlertsIdGet") => {
            let result = core.run(client.alerts_id_get("id_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("AlertsIdPut") => {
        //     let result = core.run(client.alerts_id_put("id_example".to_string(), ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("AlertsPost") => {
        //     let result = core.run(client.alerts_post(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        Some("AuthenticationJwtPost") => {
            let result = core.run(client.authentication_jwt_post());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("GetMonitorById") => {
            let result = core.run(client.get_monitor_by_id("id_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("GetMonitors") => {
            let result = core.run(client.get_monitors());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        Some("MonitorsIdDelete") => {
            let result = core.run(client.monitors_id_delete("id_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("PostMonitor") => {
        //     let result = core.run(client.post_monitor(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("UpdateMonitor") => {
        //     let result = core.run(client.update_monitor("id_example".to_string(), ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("CreateRegistration") => {
        //     let result = core.run(client.create_registration(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("RegistrationIdPost") => {
        //     let result = core.run(client.registration_id_post("id_example".to_string(), ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        Some("GetMonitorStatuses") => {
            let result = core.run(client.get_monitor_statuses());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("UpdateMonitorStatuses") => {
        //     let result = core.run(client.update_monitor_statuses(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        //  },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

