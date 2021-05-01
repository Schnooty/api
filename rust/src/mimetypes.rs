/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit

    lazy_static! {
        /// Create Mime objects for the response content types for CreateAgentSession
        pub static ref CREATE_AGENT_SESSION_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateAgentSession
        pub static ref CREATE_AGENT_SESSION_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateAgentSession
        pub static ref CREATE_AGENT_SESSION_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateAgentSession
        pub static ref CREATE_AGENT_SESSION_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateAgentSession
        pub static ref CREATE_AGENT_SESSION_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAgentSessionState
        pub static ref GET_AGENT_SESSION_STATE_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAgentSessionState
        pub static ref GET_AGENT_SESSION_STATE_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAgentSessionState
        pub static ref GET_AGENT_SESSION_STATE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAgentSessionState
        pub static ref GET_AGENT_SESSION_STATE_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAgentSessionState
        pub static ref GET_AGENT_SESSION_STATE_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsGet
        pub static ref AGENTS_GET_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsGet
        pub static ref AGENTS_GET_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsGet
        pub static ref AGENTS_GET_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsGet
        pub static ref AGENTS_GET_FORBIDDEN: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsGet
        pub static ref AGENTS_GET_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdDelete
        pub static ref AGENTS_ID_DELETE_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdDelete
        pub static ref AGENTS_ID_DELETE_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdDelete
        pub static ref AGENTS_ID_DELETE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdDelete
        pub static ref AGENTS_ID_DELETE_FORBIDDEN: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdDelete
        pub static ref AGENTS_ID_DELETE_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdGet
        pub static ref AGENTS_ID_GET_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdGet
        pub static ref AGENTS_ID_GET_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdGet
        pub static ref AGENTS_ID_GET_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdGet
        pub static ref AGENTS_ID_GET_FORBIDDEN: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdGet
        pub static ref AGENTS_ID_GET_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdGet
        pub static ref AGENTS_ID_GET_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdPut
        pub static ref AGENTS_ID_PUT_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdPut
        pub static ref AGENTS_ID_PUT_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdPut
        pub static ref AGENTS_ID_PUT_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdPut
        pub static ref AGENTS_ID_PUT_FORBIDDEN: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdPut
        pub static ref AGENTS_ID_PUT_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsIdPut
        pub static ref AGENTS_ID_PUT_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsPost
        pub static ref AGENTS_POST_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsPost
        pub static ref AGENTS_POST_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsPost
        pub static ref AGENTS_POST_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsPost
        pub static ref AGENTS_POST_FORBIDDEN: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AgentsPost
        pub static ref AGENTS_POST_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsGet
        pub static ref ALERTS_GET_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsGet
        pub static ref ALERTS_GET_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsGet
        pub static ref ALERTS_GET_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsGet
        pub static ref ALERTS_GET_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsGet
        pub static ref ALERTS_GET_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdDelete
        pub static ref ALERTS_ID_DELETE_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdDelete
        pub static ref ALERTS_ID_DELETE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdDelete
        pub static ref ALERTS_ID_DELETE_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdDelete
        pub static ref ALERTS_ID_DELETE_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdGet
        pub static ref ALERTS_ID_GET_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdGet
        pub static ref ALERTS_ID_GET_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdGet
        pub static ref ALERTS_ID_GET_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdGet
        pub static ref ALERTS_ID_GET_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdPut
        pub static ref ALERTS_ID_PUT_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdPut
        pub static ref ALERTS_ID_PUT_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdPut
        pub static ref ALERTS_ID_PUT_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsIdPut
        pub static ref ALERTS_ID_PUT_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsPost
        pub static ref ALERTS_POST_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsPost
        pub static ref ALERTS_POST_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsPost
        pub static ref ALERTS_POST_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsPost
        pub static ref ALERTS_POST_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AlertsPost
        pub static ref ALERTS_POST_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AuthenticationJwtPost
        pub static ref AUTHENTICATION_JWT_POST_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AuthenticationJwtPost
        pub static ref AUTHENTICATION_JWT_POST_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AuthenticationJwtPost
        pub static ref AUTHENTICATION_JWT_POST_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorById
        pub static ref GET_MONITOR_BY_ID_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorById
        pub static ref GET_MONITOR_BY_ID_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorById
        pub static ref GET_MONITOR_BY_ID_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorById
        pub static ref GET_MONITOR_BY_ID_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorById
        pub static ref GET_MONITOR_BY_ID_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitors
        pub static ref GET_MONITORS_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitors
        pub static ref GET_MONITORS_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitors
        pub static ref GET_MONITORS_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitors
        pub static ref GET_MONITORS_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for MonitorsIdDelete
        pub static ref MONITORS_ID_DELETE_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for MonitorsIdDelete
        pub static ref MONITORS_ID_DELETE_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for MonitorsIdDelete
        pub static ref MONITORS_ID_DELETE_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for MonitorsIdDelete
        pub static ref MONITORS_ID_DELETE_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for PostMonitor
        pub static ref POST_MONITOR_CREATED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for PostMonitor
        pub static ref POST_MONITOR_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for PostMonitor
        pub static ref POST_MONITOR_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for PostMonitor
        pub static ref POST_MONITOR_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitor
        pub static ref UPDATE_MONITOR_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitor
        pub static ref UPDATE_MONITOR_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitor
        pub static ref UPDATE_MONITOR_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitor
        pub static ref UPDATE_MONITOR_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitor
        pub static ref UPDATE_MONITOR_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateRegistration
        pub static ref CREATE_REGISTRATION_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateRegistration
        pub static ref CREATE_REGISTRATION_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateRegistration
        pub static ref CREATE_REGISTRATION_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for RegistrationIdPost
        pub static ref REGISTRATION_ID_POST_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for RegistrationIdPost
        pub static ref REGISTRATION_ID_POST_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for RegistrationIdPost
        pub static ref REGISTRATION_ID_POST_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorStatuses
        pub static ref GET_MONITOR_STATUSES_OK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorStatuses
        pub static ref GET_MONITOR_STATUSES_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorStatuses
        pub static ref GET_MONITOR_STATUSES_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetMonitorStatuses
        pub static ref GET_MONITOR_STATUSES_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitorStatuses
        pub static ref UPDATE_MONITOR_STATUSES_BAD_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitorStatuses
        pub static ref UPDATE_MONITOR_STATUSES_UNAUTHORIZED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitorStatuses
        pub static ref UPDATE_MONITOR_STATUSES_UNPROCESSABLE_ENTITY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateMonitorStatuses
        pub static ref UPDATE_MONITOR_STATUSES_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

    lazy_static! {
        /// Create Mime objects for the request content types for CreateAgentSession
        pub static ref CREATE_AGENT_SESSION: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for AgentsIdPut
        pub static ref AGENTS_ID_PUT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for AgentsPost
        pub static ref AGENTS_POST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for AlertsIdPut
        pub static ref ALERTS_ID_PUT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for AlertsPost
        pub static ref ALERTS_POST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for PostMonitor
        pub static ref POST_MONITOR: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateMonitor
        pub static ref UPDATE_MONITOR: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateRegistration
        pub static ref CREATE_REGISTRATION: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for RegistrationIdPost
        pub static ref REGISTRATION_ID_POST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateMonitorStatuses
        pub static ref UPDATE_MONITOR_STATUSES: Mime = "application/json".parse().unwrap();
    }

}
