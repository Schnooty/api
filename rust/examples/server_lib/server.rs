//! Server implementation of openapi_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;
use swagger;
use swagger::{Has, XSpanIdString};

use openapi_client::{Api, ApiError,
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
use openapi_client::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{


    fn create_agent_session(&self, group_name: String, agent_session_request: models::AgentSessionRequest, context: &C) -> Box<dyn Future<Item=CreateAgentSessionResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_agent_session(\"{}\", {:?}) - X-Span-ID: {:?}", group_name, agent_session_request, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_agent_session_state(&self, group_name: String, context: &C) -> Box<dyn Future<Item=GetAgentSessionStateResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_agent_session_state(\"{}\") - X-Span-ID: {:?}", group_name, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn agents_get(&self, context: &C) -> Box<dyn Future<Item=AgentsGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("agents_get() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn agents_id_delete(&self, id: String, context: &C) -> Box<dyn Future<Item=AgentsIdDeleteResponse, Error=ApiError>> {
        let context = context.clone();
        println!("agents_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn agents_id_get(&self, id: String, context: &C) -> Box<dyn Future<Item=AgentsIdGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("agents_id_get(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn agents_id_put(&self, id: String, agent: models::Agent, context: &C) -> Box<dyn Future<Item=AgentsIdPutResponse, Error=ApiError>> {
        let context = context.clone();
        println!("agents_id_put(\"{}\", {:?}) - X-Span-ID: {:?}", id, agent, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn agents_post(&self, agent: models::Agent, context: &C) -> Box<dyn Future<Item=AgentsPostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("agents_post({:?}) - X-Span-ID: {:?}", agent, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn alerts_get(&self, context: &C) -> Box<dyn Future<Item=AlertsGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("alerts_get() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn alerts_id_delete(&self, id: String, context: &C) -> Box<dyn Future<Item=AlertsIdDeleteResponse, Error=ApiError>> {
        let context = context.clone();
        println!("alerts_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn alerts_id_get(&self, id: String, context: &C) -> Box<dyn Future<Item=AlertsIdGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("alerts_id_get(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn alerts_id_put(&self, id: String, alert: models::Alert, context: &C) -> Box<dyn Future<Item=AlertsIdPutResponse, Error=ApiError>> {
        let context = context.clone();
        println!("alerts_id_put(\"{}\", {:?}) - X-Span-ID: {:?}", id, alert, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn alerts_post(&self, alert: models::Alert, context: &C) -> Box<dyn Future<Item=AlertsPostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("alerts_post({:?}) - X-Span-ID: {:?}", alert, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create an API token in the form of a JWT.
    fn authentication_jwt_post(&self, context: &C) -> Box<dyn Future<Item=AuthenticationJwtPostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("authentication_jwt_post() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Get a monitor by ID
    fn get_monitor_by_id(&self, id: String, context: &C) -> Box<dyn Future<Item=GetMonitorByIdResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_monitor_by_id(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Get a list of all monitors in your account.
    fn get_monitors(&self, context: &C) -> Box<dyn Future<Item=GetMonitorsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_monitors() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn monitors_id_delete(&self, id: String, context: &C) -> Box<dyn Future<Item=MonitorsIdDeleteResponse, Error=ApiError>> {
        let context = context.clone();
        println!("monitors_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create a new monitor
    fn post_monitor(&self, monitor: models::Monitor, context: &C) -> Box<dyn Future<Item=PostMonitorResponse, Error=ApiError>> {
        let context = context.clone();
        println!("post_monitor({:?}) - X-Span-ID: {:?}", monitor, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Update an existing monitor by ID
    fn update_monitor(&self, id: String, monitor: models::Monitor, context: &C) -> Box<dyn Future<Item=UpdateMonitorResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_monitor(\"{}\", {:?}) - X-Span-ID: {:?}", id, monitor, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Register your email address and password.
    fn create_registration(&self, registration: models::Registration, context: &C) -> Box<dyn Future<Item=CreateRegistrationResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_registration({:?}) - X-Span-ID: {:?}", registration, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Confirm registration of Open Monitors account.
    fn registration_id_post(&self, id: String, registration_confirmation: models::RegistrationConfirmation, context: &C) -> Box<dyn Future<Item=RegistrationIdPostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("registration_id_post(\"{}\", {:?}) - X-Span-ID: {:?}", id, registration_confirmation, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn get_monitor_statuses(&self, context: &C) -> Box<dyn Future<Item=GetMonitorStatusesResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_monitor_statuses() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn update_monitor_statuses(&self, monitor_status_array: models::MonitorStatusArray, context: &C) -> Box<dyn Future<Item=UpdateMonitorStatusesResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_monitor_statuses({:?}) - X-Span-ID: {:?}", monitor_status_array, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
