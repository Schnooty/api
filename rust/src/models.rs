#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;


// Methods for converting between header::IntoHeaderValue<Account> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Account>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Account>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Account - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Account> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Account as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Account - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Account {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub type_: models::AccountType,

    #[serde(rename = "currencyCode")]
    pub currency_code: models::CurrencyCode,

}

impl Account {
    pub fn new(name: String, type_: models::AccountType, currency_code: models::CurrencyCode, ) -> Account {
        Account {
            id: None,
            name: name,
            type_: type_,
            currency_code: currency_code,
        }
    }
}

/// Converts the Account value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Account {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        params.push("name".to_string());
        params.push(self.name.to_string());

        // Skipping type in query parameter serialization

        // Skipping currencyCode in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Account value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub type_: Vec<models::AccountType>,
            pub currency_code: Vec<models::CurrencyCode>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Account".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "type" => intermediate_rep.type_.push(models::AccountType::from_str(val).map_err(|x| format!("{}", x))?),
                    "currencyCode" => intermediate_rep.currency_code.push(models::CurrencyCode::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Account".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Account {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in Account".to_string())?,
            type_: intermediate_rep.type_.into_iter().next().ok_or("type missing in Account".to_string())?,
            currency_code: intermediate_rep.currency_code.into_iter().next().ok_or("currencyCode missing in Account".to_string())?,
        })
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum AccountType { 
    #[serde(rename = "subscription_charges")]
    CHARGES,
    #[serde(rename = "subscription_credits")]
    CREDITS,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            AccountType::CHARGES => write!(f, "{}", "subscription_charges"),
            AccountType::CREDITS => write!(f, "{}", "subscription_credits"),
        }
    }
}

impl std::str::FromStr for AccountType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "subscription_charges" => std::result::Result::Ok(AccountType::CHARGES),
            "subscription_credits" => std::result::Result::Ok(AccountType::CREDITS),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<Agent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Agent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Agent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Agent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Agent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Agent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Agent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Agent {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    /// Describes what this agent is or where it will run.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<String>,

    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub api_key: Option<String>,

}

impl Agent {
    pub fn new(name: String, ) -> Agent {
        Agent {
            id: None,
            name: name,
            description: None,
            group: None,
            api_key: None,
        }
    }
}

/// Converts the Agent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Agent {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        params.push("name".to_string());
        params.push(self.name.to_string());


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        if let Some(ref group) = self.group {
            params.push("group".to_string());
            params.push(group.to_string());
        }


        if let Some(ref api_key) = self.api_key {
            params.push("apiKey".to_string());
            params.push(api_key.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Agent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Agent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub group: Vec<String>,
            pub api_key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Agent".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "group" => intermediate_rep.group.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "apiKey" => intermediate_rep.api_key.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Agent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Agent {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in Agent".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            group: intermediate_rep.group.into_iter().next(),
            api_key: intermediate_rep.api_key.into_iter().next(),
        })
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AgentApiKey(String);

impl std::convert::From<String> for AgentApiKey {
    fn from(x: String) -> Self {
        AgentApiKey(x)
    }
}

impl std::string::ToString for AgentApiKey {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for AgentApiKey {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(AgentApiKey(x.to_string()))
    }
}

impl std::convert::From<AgentApiKey> for String {
    fn from(x: AgentApiKey) -> Self {
        x.0
    }
}

impl std::ops::Deref for AgentApiKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for AgentApiKey {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



// Methods for converting between header::IntoHeaderValue<AgentSessionAssignment> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AgentSessionAssignment>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AgentSessionAssignment>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AgentSessionAssignment - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AgentSessionAssignment> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AgentSessionAssignment as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AgentSessionAssignment - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AgentSessionAssignment {
    #[serde(rename = "sessionId")]
    pub session_id: String,

    #[serde(rename = "agentId")]
    pub agent_id: String,

    #[serde(rename = "monitorIds")]
    pub monitor_ids: Vec<models::IdString>,

    #[serde(rename = "lastHeartbeatTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_heartbeat_timestamp: Option<isize>,

}

impl AgentSessionAssignment {
    pub fn new(session_id: String, agent_id: String, monitor_ids: Vec<models::IdString>, ) -> AgentSessionAssignment {
        AgentSessionAssignment {
            session_id: session_id,
            agent_id: agent_id,
            monitor_ids: monitor_ids,
            last_heartbeat_timestamp: None,
        }
    }
}

/// Converts the AgentSessionAssignment value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AgentSessionAssignment {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("sessionId".to_string());
        params.push(self.session_id.to_string());


        params.push("agentId".to_string());
        params.push(self.agent_id.to_string());


        params.push("monitorIds".to_string());
        params.push(self.monitor_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());


        if let Some(ref last_heartbeat_timestamp) = self.last_heartbeat_timestamp {
            params.push("lastHeartbeatTimestamp".to_string());
            params.push(last_heartbeat_timestamp.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AgentSessionAssignment value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AgentSessionAssignment {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub session_id: Vec<String>,
            pub agent_id: Vec<String>,
            pub monitor_ids: Vec<Vec<models::IdString>>,
            pub last_heartbeat_timestamp: Vec<isize>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AgentSessionAssignment".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "sessionId" => intermediate_rep.session_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "agentId" => intermediate_rep.agent_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "monitorIds" => return std::result::Result::Err("Parsing a container in this style is not supported in AgentSessionAssignment".to_string()),
                    "lastHeartbeatTimestamp" => intermediate_rep.last_heartbeat_timestamp.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AgentSessionAssignment".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AgentSessionAssignment {
            session_id: intermediate_rep.session_id.into_iter().next().ok_or("sessionId missing in AgentSessionAssignment".to_string())?,
            agent_id: intermediate_rep.agent_id.into_iter().next().ok_or("agentId missing in AgentSessionAssignment".to_string())?,
            monitor_ids: intermediate_rep.monitor_ids.into_iter().next().ok_or("monitorIds missing in AgentSessionAssignment".to_string())?,
            last_heartbeat_timestamp: intermediate_rep.last_heartbeat_timestamp.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AgentSessionRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AgentSessionRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AgentSessionRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AgentSessionRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AgentSessionRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AgentSessionRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AgentSessionRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AgentSessionRequest {
    #[serde(rename = "sessionId")]
    pub session_id: String,

    /// Optional indicator that tells the server that this is a new session.
    #[serde(rename = "isNew")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_new: Option<bool>,

}

impl AgentSessionRequest {
    pub fn new(session_id: String, ) -> AgentSessionRequest {
        AgentSessionRequest {
            session_id: session_id,
            is_new: None,
        }
    }
}

/// Converts the AgentSessionRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AgentSessionRequest {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("sessionId".to_string());
        params.push(self.session_id.to_string());


        if let Some(ref is_new) = self.is_new {
            params.push("isNew".to_string());
            params.push(is_new.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AgentSessionRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AgentSessionRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub session_id: Vec<String>,
            pub is_new: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AgentSessionRequest".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "sessionId" => intermediate_rep.session_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "isNew" => intermediate_rep.is_new.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AgentSessionRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AgentSessionRequest {
            session_id: intermediate_rep.session_id.into_iter().next().ok_or("sessionId missing in AgentSessionRequest".to_string())?,
            is_new: intermediate_rep.is_new.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AgentSessionState> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AgentSessionState>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AgentSessionState>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AgentSessionState - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AgentSessionState> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AgentSessionState as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AgentSessionState - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AgentSessionState {
    #[serde(rename = "group")]
    pub group: String,

    #[serde(rename = "agents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub agents: Option<Vec<models::AgentSessionAssignment>>,

}

impl AgentSessionState {
    pub fn new(group: String, ) -> AgentSessionState {
        AgentSessionState {
            group: group,
            agents: None,
        }
    }
}

/// Converts the AgentSessionState value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AgentSessionState {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("group".to_string());
        params.push(self.group.to_string());

        // Skipping agents in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AgentSessionState value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AgentSessionState {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub group: Vec<String>,
            pub agents: Vec<Vec<models::AgentSessionAssignment>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AgentSessionState".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "group" => intermediate_rep.group.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "agents" => return std::result::Result::Err("Parsing a container in this style is not supported in AgentSessionState".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing AgentSessionState".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AgentSessionState {
            group: intermediate_rep.group.into_iter().next().ok_or("group missing in AgentSessionState".to_string())?,
            agents: intermediate_rep.agents.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Alert> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Alert>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Alert>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Alert - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Alert> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Alert as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Alert - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Alert {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    /// Describes what your alert will do.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "monitors")]
    pub monitors: Vec<models::IdString>,

    #[serde(rename = "threshold")]
    pub threshold: isize,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "body")]
    pub body: models::AlertBody,

}

impl Alert {
    pub fn new(type_: String, monitors: Vec<models::IdString>, threshold: isize, enabled: bool, body: models::AlertBody, ) -> Alert {
        Alert {
            type_: type_,
            id: None,
            description: None,
            monitors: monitors,
            threshold: threshold,
            enabled: enabled,
            body: body,
        }
    }
}

/// Converts the Alert value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Alert {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("type".to_string());
        params.push(self.type_.to_string());


        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("monitors".to_string());
        params.push(self.monitors.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());


        params.push("threshold".to_string());
        params.push(self.threshold.to_string());


        params.push("enabled".to_string());
        params.push(self.enabled.to_string());

        // Skipping body in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Alert value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Alert {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub type_: Vec<String>,
            pub id: Vec<String>,
            pub description: Vec<String>,
            pub monitors: Vec<Vec<models::IdString>>,
            pub threshold: Vec<isize>,
            pub enabled: Vec<bool>,
            pub body: Vec<models::AlertBody>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Alert".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "type" => intermediate_rep.type_.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "monitors" => return std::result::Result::Err("Parsing a container in this style is not supported in Alert".to_string()),
                    "threshold" => intermediate_rep.threshold.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "enabled" => intermediate_rep.enabled.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "body" => intermediate_rep.body.push(models::AlertBody::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Alert".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Alert {
            type_: intermediate_rep.type_.into_iter().next().ok_or("type missing in Alert".to_string())?,
            id: intermediate_rep.id.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            monitors: intermediate_rep.monitors.into_iter().next().ok_or("monitors missing in Alert".to_string())?,
            threshold: intermediate_rep.threshold.into_iter().next().ok_or("threshold missing in Alert".to_string())?,
            enabled: intermediate_rep.enabled.into_iter().next().ok_or("enabled missing in Alert".to_string())?,
            body: intermediate_rep.body.into_iter().next().ok_or("body missing in Alert".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AlertArray> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AlertArray>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AlertArray>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AlertArray - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AlertArray> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AlertArray as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AlertArray - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AlertArray {
    #[serde(rename = "alerts")]
    pub alerts: Vec<models::Alert>,

}

impl AlertArray {
    pub fn new(alerts: Vec<models::Alert>, ) -> AlertArray {
        AlertArray {
            alerts: alerts,
        }
    }
}

/// Converts the AlertArray value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AlertArray {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping alerts in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AlertArray value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AlertArray {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub alerts: Vec<Vec<models::Alert>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AlertArray".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "alerts" => return std::result::Result::Err("Parsing a container in this style is not supported in AlertArray".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing AlertArray".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AlertArray {
            alerts: intermediate_rep.alerts.into_iter().next().ok_or("alerts missing in AlertArray".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AlertBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AlertBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AlertBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AlertBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AlertBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AlertBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AlertBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AlertBody {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "headers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub headers: Option<Vec<models::HttpHeader>>,

    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<String>,

    /// The addresses to which the alert email will be sent.
    #[serde(rename = "recipients")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recipients: Option<Vec<String>>,

    /// Your SMTP server's hostname
    #[serde(rename = "host")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host: Option<String>,

    /// The port number for your SMTP server
    #[serde(rename = "port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<f64>,

    #[serde(rename = "tlsMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tls_mode: Option<models::TlsMode>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

}

impl AlertBody {
    pub fn new() -> AlertBody {
        AlertBody {
            url: None,
            headers: None,
            from: None,
            recipients: None,
            host: None,
            port: None,
            tls_mode: None,
            username: None,
            password: None,
        }
    }
}

/// Converts the AlertBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AlertBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref url) = self.url {
            params.push("url".to_string());
            params.push(url.to_string());
        }

        // Skipping headers in query parameter serialization


        if let Some(ref from) = self.from {
            params.push("from".to_string());
            params.push(from.to_string());
        }


        if let Some(ref recipients) = self.recipients {
            params.push("recipients".to_string());
            params.push(recipients.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }


        if let Some(ref host) = self.host {
            params.push("host".to_string());
            params.push(host.to_string());
        }


        if let Some(ref port) = self.port {
            params.push("port".to_string());
            params.push(port.to_string());
        }

        // Skipping tlsMode in query parameter serialization


        if let Some(ref username) = self.username {
            params.push("username".to_string());
            params.push(username.to_string());
        }


        if let Some(ref password) = self.password {
            params.push("password".to_string());
            params.push(password.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AlertBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AlertBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub url: Vec<String>,
            pub headers: Vec<Vec<models::HttpHeader>>,
            pub from: Vec<String>,
            pub recipients: Vec<Vec<String>>,
            pub host: Vec<String>,
            pub port: Vec<f64>,
            pub tls_mode: Vec<models::TlsMode>,
            pub username: Vec<String>,
            pub password: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AlertBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "url" => intermediate_rep.url.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "headers" => return std::result::Result::Err("Parsing a container in this style is not supported in AlertBody".to_string()),
                    "from" => intermediate_rep.from.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "recipients" => return std::result::Result::Err("Parsing a container in this style is not supported in AlertBody".to_string()),
                    "host" => intermediate_rep.host.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "port" => intermediate_rep.port.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    "tlsMode" => intermediate_rep.tls_mode.push(models::TlsMode::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AlertBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AlertBody {
            url: intermediate_rep.url.into_iter().next(),
            headers: intermediate_rep.headers.into_iter().next(),
            from: intermediate_rep.from.into_iter().next(),
            recipients: intermediate_rep.recipients.into_iter().next(),
            host: intermediate_rep.host.into_iter().next(),
            port: intermediate_rep.port.into_iter().next(),
            tls_mode: intermediate_rep.tls_mode.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
            password: intermediate_rep.password.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AndExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AndExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AndExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AndExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AndExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AndExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AndExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AndExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::BoolExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::BoolExpr,

}

impl AndExpr {
    pub fn new(lhs: models::BoolExpr, rhs: models::BoolExpr, ) -> AndExpr {
        AndExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the AndExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AndExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AndExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AndExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::BoolExpr>,
            pub rhs: Vec<models::BoolExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AndExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AndExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AndExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in AndExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in AndExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<ArthExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ArthExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ArthExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ArthExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ArthExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ArthExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ArthExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ArthExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "constant")]
    pub constant: f64,

}

impl ArthExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, name: String, constant: f64, ) -> ArthExpr {
        ArthExpr {
            lhs: lhs,
            rhs: rhs,
            name: name,
            constant: constant,
        }
    }
}

/// Converts the ArthExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ArthExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization


        params.push("name".to_string());
        params.push(self.name.to_string());


        params.push("constant".to_string());
        params.push(self.constant.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ArthExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ArthExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
            pub name: Vec<String>,
            pub constant: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ArthExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "constant" => intermediate_rep.constant.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ArthExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ArthExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in ArthExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in ArthExpr".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in ArthExpr".to_string())?,
            constant: intermediate_rep.constant.into_iter().next().ok_or("constant missing in ArthExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Balance> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Balance>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Balance>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Balance - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Balance> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Balance as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Balance - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Balance {
    #[serde(rename = "accountId")]
    pub account_id: String,

    /// UTC UNIX timestamp in with fractional offset.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "balance")]
    pub balance: models::Money,

    #[serde(rename = "availableBalance")]
    pub available_balance: models::Money,

}

impl Balance {
    pub fn new(account_id: String, timestamp: chrono::DateTime::<chrono::Utc>, balance: models::Money, available_balance: models::Money, ) -> Balance {
        Balance {
            account_id: account_id,
            timestamp: timestamp,
            balance: balance,
            available_balance: available_balance,
        }
    }
}

/// Converts the Balance value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Balance {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("accountId".to_string());
        params.push(self.account_id.to_string());

        // Skipping timestamp in query parameter serialization

        // Skipping balance in query parameter serialization

        // Skipping availableBalance in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Balance value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Balance {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub account_id: Vec<String>,
            pub timestamp: Vec<chrono::DateTime::<chrono::Utc>>,
            pub balance: Vec<models::Money>,
            pub available_balance: Vec<models::Money>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Balance".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "accountId" => intermediate_rep.account_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "timestamp" => intermediate_rep.timestamp.push(chrono::DateTime::<chrono::Utc>::from_str(val).map_err(|x| format!("{}", x))?),
                    "balance" => intermediate_rep.balance.push(models::Money::from_str(val).map_err(|x| format!("{}", x))?),
                    "availableBalance" => intermediate_rep.available_balance.push(models::Money::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Balance".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Balance {
            account_id: intermediate_rep.account_id.into_iter().next().ok_or("accountId missing in Balance".to_string())?,
            timestamp: intermediate_rep.timestamp.into_iter().next().ok_or("timestamp missing in Balance".to_string())?,
            balance: intermediate_rep.balance.into_iter().next().ok_or("balance missing in Balance".to_string())?,
            available_balance: intermediate_rep.available_balance.into_iter().next().ok_or("availableBalance missing in Balance".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Balances> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Balances>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Balances>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Balances - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Balances> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Balances as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Balances - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Balances {
    #[serde(rename = "balances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub balances: Option<Vec<models::Balance>>,

}

impl Balances {
    pub fn new() -> Balances {
        Balances {
            balances: None,
        }
    }
}

/// Converts the Balances value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Balances {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping balances in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Balances value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Balances {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub balances: Vec<Vec<models::Balance>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Balances".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "balances" => return std::result::Result::Err("Parsing a container in this style is not supported in Balances".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Balances".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Balances {
            balances: intermediate_rep.balances.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<BoolExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BoolExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BoolExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BoolExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BoolExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BoolExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BoolExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BoolExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

    #[serde(rename = "expr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expr: Option<models::BoolExpr>,

}

impl BoolExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> BoolExpr {
        BoolExpr {
            lhs: lhs,
            rhs: rhs,
            expr: None,
        }
    }
}

/// Converts the BoolExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BoolExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        // Skipping expr in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BoolExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BoolExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
            pub expr: Vec<models::BoolExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BoolExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "expr" => intermediate_rep.expr.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BoolExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BoolExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in BoolExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in BoolExpr".to_string())?,
            expr: intermediate_rep.expr.into_iter().next(),
        })
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ByteUnit(String);

impl std::convert::From<String> for ByteUnit {
    fn from(x: String) -> Self {
        ByteUnit(x)
    }
}

impl std::string::ToString for ByteUnit {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for ByteUnit {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(ByteUnit(x.to_string()))
    }
}

impl std::convert::From<ByteUnit> for String {
    fn from(x: ByteUnit) -> Self {
        x.0
    }
}

impl std::ops::Deref for ByteUnit {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for ByteUnit {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ChargeType { 
    #[serde(rename = "daily")]
    DAILY,
    #[serde(rename = "weekly")]
    WEEKLY,
    #[serde(rename = "monthly")]
    MONTHLY,
}

impl std::fmt::Display for ChargeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            ChargeType::DAILY => write!(f, "{}", "daily"),
            ChargeType::WEEKLY => write!(f, "{}", "weekly"),
            ChargeType::MONTHLY => write!(f, "{}", "monthly"),
        }
    }
}

impl std::str::FromStr for ChargeType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "daily" => std::result::Result::Ok(ChargeType::DAILY),
            "weekly" => std::result::Result::Ok(ChargeType::WEEKLY),
            "monthly" => std::result::Result::Ok(ChargeType::MONTHLY),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<ConstExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConstExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConstExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConstExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConstExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConstExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConstExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConstExpr {
    #[serde(rename = "constant")]
    pub constant: f64,

}

impl ConstExpr {
    pub fn new(constant: f64, ) -> ConstExpr {
        ConstExpr {
            constant: constant,
        }
    }
}

/// Converts the ConstExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConstExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("constant".to_string());
        params.push(self.constant.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConstExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConstExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub constant: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConstExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "constant" => intermediate_rep.constant.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConstExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConstExpr {
            constant: intermediate_rep.constant.into_iter().next().ok_or("constant missing in ConstExpr".to_string())?,
        })
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum CurrencyCode { 
    #[serde(rename = "NZD")]
    NZD,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "EUR")]
    EUR,
}

impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            CurrencyCode::NZD => write!(f, "{}", "NZD"),
            CurrencyCode::USD => write!(f, "{}", "USD"),
            CurrencyCode::EUR => write!(f, "{}", "EUR"),
        }
    }
}

impl std::str::FromStr for CurrencyCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NZD" => std::result::Result::Ok(CurrencyCode::NZD),
            "USD" => std::result::Result::Ok(CurrencyCode::USD),
            "EUR" => std::result::Result::Ok(CurrencyCode::EUR),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


/// UTC UNIX timestamp in with fractional offset.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DateTime(chrono::DateTime::<chrono::Utc>);

impl std::convert::From<chrono::DateTime::<chrono::Utc>> for DateTime {
    fn from(x: chrono::DateTime::<chrono::Utc>) -> Self {
        DateTime(x)
    }
}


impl std::convert::From<DateTime> for chrono::DateTime::<chrono::Utc> {
    fn from(x: DateTime) -> Self {
        x.0
    }
}

impl std::ops::Deref for DateTime {
    type Target = chrono::DateTime::<chrono::Utc>;
    fn deref(&self) -> &chrono::DateTime::<chrono::Utc> {
        &self.0
    }
}

impl std::ops::DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut chrono::DateTime::<chrono::Utc> {
        &mut self.0
    }
}



// Methods for converting between header::IntoHeaderValue<DivExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DivExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DivExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DivExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DivExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DivExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DivExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DivExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl DivExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> DivExpr {
        DivExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the DivExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DivExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DivExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DivExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DivExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DivExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DivExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in DivExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in DivExpr".to_string())?,
        })
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EasyString64(String);

impl std::convert::From<String> for EasyString64 {
    fn from(x: String) -> Self {
        EasyString64(x)
    }
}

impl std::string::ToString for EasyString64 {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for EasyString64 {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(EasyString64(x.to_string()))
    }
}

impl std::convert::From<EasyString64> for String {
    fn from(x: EasyString64) -> Self {
        x.0
    }
}

impl std::ops::Deref for EasyString64 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for EasyString64 {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EmailAddress(String);

impl std::convert::From<String> for EmailAddress {
    fn from(x: String) -> Self {
        EmailAddress(x)
    }
}

impl std::string::ToString for EmailAddress {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for EmailAddress {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(EmailAddress(x.to_string()))
    }
}

impl std::convert::From<EmailAddress> for String {
    fn from(x: EmailAddress) -> Self {
        x.0
    }
}

impl std::ops::Deref for EmailAddress {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for EmailAddress {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



// Methods for converting between header::IntoHeaderValue<EmailAlertBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EmailAlertBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EmailAlertBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for EmailAlertBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<EmailAlertBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EmailAlertBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into EmailAlertBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EmailAlertBody {
    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<String>,

    /// The addresses to which the alert email will be sent.
    #[serde(rename = "recipients")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recipients: Option<Vec<String>>,

    /// Your SMTP server's hostname
    #[serde(rename = "host")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host: Option<String>,

    /// The port number for your SMTP server
    #[serde(rename = "port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<f64>,

    #[serde(rename = "tlsMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tls_mode: Option<models::TlsMode>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

}

impl EmailAlertBody {
    pub fn new() -> EmailAlertBody {
        EmailAlertBody {
            from: None,
            recipients: None,
            host: None,
            port: None,
            tls_mode: None,
            username: None,
            password: None,
        }
    }
}

/// Converts the EmailAlertBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EmailAlertBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref from) = self.from {
            params.push("from".to_string());
            params.push(from.to_string());
        }


        if let Some(ref recipients) = self.recipients {
            params.push("recipients".to_string());
            params.push(recipients.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }


        if let Some(ref host) = self.host {
            params.push("host".to_string());
            params.push(host.to_string());
        }


        if let Some(ref port) = self.port {
            params.push("port".to_string());
            params.push(port.to_string());
        }

        // Skipping tlsMode in query parameter serialization


        if let Some(ref username) = self.username {
            params.push("username".to_string());
            params.push(username.to_string());
        }


        if let Some(ref password) = self.password {
            params.push("password".to_string());
            params.push(password.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EmailAlertBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EmailAlertBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub from: Vec<String>,
            pub recipients: Vec<Vec<String>>,
            pub host: Vec<String>,
            pub port: Vec<f64>,
            pub tls_mode: Vec<models::TlsMode>,
            pub username: Vec<String>,
            pub password: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing EmailAlertBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "from" => intermediate_rep.from.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "recipients" => return std::result::Result::Err("Parsing a container in this style is not supported in EmailAlertBody".to_string()),
                    "host" => intermediate_rep.host.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "port" => intermediate_rep.port.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    "tlsMode" => intermediate_rep.tls_mode.push(models::TlsMode::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing EmailAlertBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EmailAlertBody {
            from: intermediate_rep.from.into_iter().next(),
            recipients: intermediate_rep.recipients.into_iter().next(),
            host: intermediate_rep.host.into_iter().next(),
            port: intermediate_rep.port.into_iter().next(),
            tls_mode: intermediate_rep.tls_mode.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
            password: intermediate_rep.password.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<EqExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EqExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EqExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for EqExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<EqExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EqExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into EqExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EqExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl EqExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> EqExpr {
        EqExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the EqExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EqExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EqExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EqExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing EqExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing EqExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EqExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in EqExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in EqExpr".to_string())?,
        })
    }
}



/// Global error code that indicates what went wrong
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ErrorCode { 
    #[serde(rename = "unauthorised")]
    UNAUTHORISED,
    #[serde(rename = "forbidden")]
    FORBIDDEN,
    #[serde(rename = "not_found")]
    NOT_FOUND,
    #[serde(rename = "product_limit_reached")]
    PRODUCT_LIMIT_REACHED,
    #[serde(rename = "database_failure")]
    DATABASE_FAILURE,
    #[serde(rename = "solution_incorrect")]
    SOLUTION_INCORRECT,
    #[serde(rename = "solution_expired")]
    SOLUTION_EXPIRED,
    #[serde(rename = "internal_parse_failure")]
    INTERNAL_PARSE_FAILURE,
    #[serde(rename = "invalid_string")]
    INVALID_STRING,
    #[serde(rename = "invalid_confirmation_code")]
    INVALID_CONFIRMATION_CODE,
    #[serde(rename = "conflict")]
    CONFLICT,
    #[serde(rename = "insufficient_funds")]
    INSUFFICIENT_FUNDS,
    #[serde(rename = "internal_error")]
    INTERNAL_ERROR,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            ErrorCode::UNAUTHORISED => write!(f, "{}", "unauthorised"),
            ErrorCode::FORBIDDEN => write!(f, "{}", "forbidden"),
            ErrorCode::NOT_FOUND => write!(f, "{}", "not_found"),
            ErrorCode::PRODUCT_LIMIT_REACHED => write!(f, "{}", "product_limit_reached"),
            ErrorCode::DATABASE_FAILURE => write!(f, "{}", "database_failure"),
            ErrorCode::SOLUTION_INCORRECT => write!(f, "{}", "solution_incorrect"),
            ErrorCode::SOLUTION_EXPIRED => write!(f, "{}", "solution_expired"),
            ErrorCode::INTERNAL_PARSE_FAILURE => write!(f, "{}", "internal_parse_failure"),
            ErrorCode::INVALID_STRING => write!(f, "{}", "invalid_string"),
            ErrorCode::INVALID_CONFIRMATION_CODE => write!(f, "{}", "invalid_confirmation_code"),
            ErrorCode::CONFLICT => write!(f, "{}", "conflict"),
            ErrorCode::INSUFFICIENT_FUNDS => write!(f, "{}", "insufficient_funds"),
            ErrorCode::INTERNAL_ERROR => write!(f, "{}", "internal_error"),
        }
    }
}

impl std::str::FromStr for ErrorCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "unauthorised" => std::result::Result::Ok(ErrorCode::UNAUTHORISED),
            "forbidden" => std::result::Result::Ok(ErrorCode::FORBIDDEN),
            "not_found" => std::result::Result::Ok(ErrorCode::NOT_FOUND),
            "product_limit_reached" => std::result::Result::Ok(ErrorCode::PRODUCT_LIMIT_REACHED),
            "database_failure" => std::result::Result::Ok(ErrorCode::DATABASE_FAILURE),
            "solution_incorrect" => std::result::Result::Ok(ErrorCode::SOLUTION_INCORRECT),
            "solution_expired" => std::result::Result::Ok(ErrorCode::SOLUTION_EXPIRED),
            "internal_parse_failure" => std::result::Result::Ok(ErrorCode::INTERNAL_PARSE_FAILURE),
            "invalid_string" => std::result::Result::Ok(ErrorCode::INVALID_STRING),
            "invalid_confirmation_code" => std::result::Result::Ok(ErrorCode::INVALID_CONFIRMATION_CODE),
            "conflict" => std::result::Result::Ok(ErrorCode::CONFLICT),
            "insufficient_funds" => std::result::Result::Ok(ErrorCode::INSUFFICIENT_FUNDS),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<ErrorList> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorList>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorList>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorList - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ErrorList> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorList as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorList - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorList {
    #[serde(rename = "errors")]
    pub errors: Vec<models::ResponseError>,

}

impl ErrorList {
    pub fn new(errors: Vec<models::ResponseError>, ) -> ErrorList {
        ErrorList {
            errors: errors,
        }
    }
}

/// Converts the ErrorList value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ErrorList {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping errors in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorList value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorList {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub errors: Vec<Vec<models::ResponseError>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorList".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "errors" => return std::result::Result::Err("Parsing a container in this style is not supported in ErrorList".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorList".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorList {
            errors: intermediate_rep.errors.into_iter().next().ok_or("errors missing in ErrorList".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<FieldExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FieldExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FieldExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FieldExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FieldExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FieldExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FieldExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FieldExpr {
    #[serde(rename = "name")]
    pub name: String,

}

impl FieldExpr {
    pub fn new(name: String, ) -> FieldExpr {
        FieldExpr {
            name: name,
        }
    }
}

/// Converts the FieldExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FieldExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("name".to_string());
        params.push(self.name.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FieldExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FieldExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FieldExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FieldExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FieldExpr {
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in FieldExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<GtEqExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GtEqExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GtEqExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GtEqExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GtEqExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GtEqExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GtEqExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GtEqExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl GtEqExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> GtEqExpr {
        GtEqExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the GtEqExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GtEqExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GtEqExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GtEqExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GtEqExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GtEqExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GtEqExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in GtEqExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in GtEqExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<GtExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GtExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GtExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GtExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GtExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GtExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GtExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GtExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl GtExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> GtExpr {
        GtExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the GtExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GtExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GtExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GtExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GtExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GtExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GtExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in GtExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in GtExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<HttpHeader> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HttpHeader>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<HttpHeader>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for HttpHeader - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<HttpHeader> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <HttpHeader as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into HttpHeader - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HttpHeader {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value")]
    pub value: String,

}

impl HttpHeader {
    pub fn new(name: String, value: String, ) -> HttpHeader {
        HttpHeader {
            name: name,
            value: value,
        }
    }
}

/// Converts the HttpHeader value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HttpHeader {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("name".to_string());
        params.push(self.name.to_string());


        params.push("value".to_string());
        params.push(self.value.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HttpHeader value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HttpHeader {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub name: Vec<String>,
            pub value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing HttpHeader".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "value" => intermediate_rep.value.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing HttpHeader".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HttpHeader {
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in HttpHeader".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or("value missing in HttpHeader".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<HttpMonitorBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HttpMonitorBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<HttpMonitorBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for HttpMonitorBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<HttpMonitorBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <HttpMonitorBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into HttpMonitorBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HttpMonitorBody {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "method")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub method: Option<String>,

    #[serde(rename = "headers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub headers: Option<Vec<models::HttpHeader>>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}

impl HttpMonitorBody {
    pub fn new() -> HttpMonitorBody {
        HttpMonitorBody {
            url: None,
            method: None,
            headers: None,
            body: None,
        }
    }
}

/// Converts the HttpMonitorBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HttpMonitorBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref url) = self.url {
            params.push("url".to_string());
            params.push(url.to_string());
        }


        if let Some(ref method) = self.method {
            params.push("method".to_string());
            params.push(method.to_string());
        }

        // Skipping headers in query parameter serialization


        if let Some(ref body) = self.body {
            params.push("body".to_string());
            params.push(body.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HttpMonitorBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HttpMonitorBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub url: Vec<String>,
            pub method: Vec<String>,
            pub headers: Vec<Vec<models::HttpHeader>>,
            pub body: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing HttpMonitorBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "url" => intermediate_rep.url.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "method" => intermediate_rep.method.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "headers" => return std::result::Result::Err("Parsing a container in this style is not supported in HttpMonitorBody".to_string()),
                    "body" => intermediate_rep.body.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing HttpMonitorBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HttpMonitorBody {
            url: intermediate_rep.url.into_iter().next(),
            method: intermediate_rep.method.into_iter().next(),
            headers: intermediate_rep.headers.into_iter().next(),
            body: intermediate_rep.body.into_iter().next(),
        })
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IdString(String);

impl std::convert::From<String> for IdString {
    fn from(x: String) -> Self {
        IdString(x)
    }
}

impl std::string::ToString for IdString {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for IdString {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(IdString(x.to_string()))
    }
}

impl std::convert::From<IdString> for String {
    fn from(x: IdString) -> Self {
        x.0
    }
}

impl std::ops::Deref for IdString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for IdString {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



// Methods for converting between header::IntoHeaderValue<InlineResponse200> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse200>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse200>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse200 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse200> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse200 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse200 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse200 {
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accounts: Option<Vec<models::Account>>,

}

impl InlineResponse200 {
    pub fn new() -> InlineResponse200 {
        InlineResponse200 {
            accounts: None,
        }
    }
}

/// Converts the InlineResponse200 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse200 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping accounts in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse200 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse200 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub accounts: Vec<Vec<models::Account>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse200".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "accounts" => return std::result::Result::Err("Parsing a container in this style is not supported in InlineResponse200".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse200".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse200 {
            accounts: intermediate_rep.accounts.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<InlineResponse2001> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2001>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse2001>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse2001 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse2001> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse2001 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse2001 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2001 {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<models::Account>,

}

impl InlineResponse2001 {
    pub fn new() -> InlineResponse2001 {
        InlineResponse2001 {
            account: None,
        }
    }
}

/// Converts the InlineResponse2001 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2001 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping account in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2001 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2001 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub account: Vec<models::Account>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse2001".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "account" => intermediate_rep.account.push(models::Account::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse2001".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2001 {
            account: intermediate_rep.account.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<InlineResponse2002> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2002>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse2002>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse2002 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse2002> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse2002 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse2002 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2002 {
    #[serde(rename = "balance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub balance: Option<models::Balance>,

}

impl InlineResponse2002 {
    pub fn new() -> InlineResponse2002 {
        InlineResponse2002 {
            balance: None,
        }
    }
}

/// Converts the InlineResponse2002 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2002 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping balance in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2002 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2002 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub balance: Vec<models::Balance>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse2002".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "balance" => intermediate_rep.balance.push(models::Balance::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse2002".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2002 {
            balance: intermediate_rep.balance.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<JwtObject> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<JwtObject>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<JwtObject>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for JwtObject - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<JwtObject> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <JwtObject as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into JwtObject - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct JwtObject {
    /// The JSON Web Token that can be used to access the API.
    #[serde(rename = "jwt")]
    pub jwt: String,

    /// Time at which the JWT expires in seconds since Unix Epoc
    #[serde(rename = "jwtExpiry")]
    pub jwt_expiry: f64,

    #[serde(rename = "jwtScopes")]
    pub jwt_scopes: Vec<String>,

    /// A Bearer token that can used to get a new JWT.
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,

    /// Time at which the refresh token expires in seconds since Unix Epoc
    #[serde(rename = "refreshTokenExpiry")]
    pub refresh_token_expiry: f64,

}

impl JwtObject {
    pub fn new(jwt: String, jwt_expiry: f64, jwt_scopes: Vec<String>, refresh_token: String, refresh_token_expiry: f64, ) -> JwtObject {
        JwtObject {
            jwt: jwt,
            jwt_expiry: jwt_expiry,
            jwt_scopes: jwt_scopes,
            refresh_token: refresh_token,
            refresh_token_expiry: refresh_token_expiry,
        }
    }
}

/// Converts the JwtObject value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for JwtObject {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("jwt".to_string());
        params.push(self.jwt.to_string());


        params.push("jwtExpiry".to_string());
        params.push(self.jwt_expiry.to_string());


        params.push("jwtScopes".to_string());
        params.push(self.jwt_scopes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());


        params.push("refreshToken".to_string());
        params.push(self.refresh_token.to_string());


        params.push("refreshTokenExpiry".to_string());
        params.push(self.refresh_token_expiry.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a JwtObject value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for JwtObject {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub jwt: Vec<String>,
            pub jwt_expiry: Vec<f64>,
            pub jwt_scopes: Vec<Vec<String>>,
            pub refresh_token: Vec<String>,
            pub refresh_token_expiry: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing JwtObject".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "jwt" => intermediate_rep.jwt.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "jwtExpiry" => intermediate_rep.jwt_expiry.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    "jwtScopes" => return std::result::Result::Err("Parsing a container in this style is not supported in JwtObject".to_string()),
                    "refreshToken" => intermediate_rep.refresh_token.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "refreshTokenExpiry" => intermediate_rep.refresh_token_expiry.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing JwtObject".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(JwtObject {
            jwt: intermediate_rep.jwt.into_iter().next().ok_or("jwt missing in JwtObject".to_string())?,
            jwt_expiry: intermediate_rep.jwt_expiry.into_iter().next().ok_or("jwtExpiry missing in JwtObject".to_string())?,
            jwt_scopes: intermediate_rep.jwt_scopes.into_iter().next().ok_or("jwtScopes missing in JwtObject".to_string())?,
            refresh_token: intermediate_rep.refresh_token.into_iter().next().ok_or("refreshToken missing in JwtObject".to_string())?,
            refresh_token_expiry: intermediate_rep.refresh_token_expiry.into_iter().next().ok_or("refreshTokenExpiry missing in JwtObject".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<LtEqExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LtEqExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LtEqExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LtEqExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LtEqExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LtEqExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LtEqExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LtEqExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl LtEqExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> LtEqExpr {
        LtEqExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the LtEqExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LtEqExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LtEqExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LtEqExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LtEqExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LtEqExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LtEqExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in LtEqExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in LtEqExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<LtExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LtExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LtExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LtExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LtExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LtExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LtExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LtExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl LtExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> LtExpr {
        LtExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the LtExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LtExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LtExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LtExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LtExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LtExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LtExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in LtExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in LtExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Money> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Money>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Money>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Money - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Money> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Money as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Money - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Money {
    #[serde(rename = "value")]
    pub value: isize,

    #[serde(rename = "decimalPlaces")]
    pub decimal_places: isize,

    #[serde(rename = "currencyCode")]
    pub currency_code: models::CurrencyCode,

}

impl Money {
    pub fn new(value: isize, decimal_places: isize, currency_code: models::CurrencyCode, ) -> Money {
        Money {
            value: value,
            decimal_places: decimal_places,
            currency_code: currency_code,
        }
    }
}

/// Converts the Money value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Money {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("value".to_string());
        params.push(self.value.to_string());


        params.push("decimalPlaces".to_string());
        params.push(self.decimal_places.to_string());

        // Skipping currencyCode in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Money value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Money {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub value: Vec<isize>,
            pub decimal_places: Vec<isize>,
            pub currency_code: Vec<models::CurrencyCode>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Money".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "value" => intermediate_rep.value.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "decimalPlaces" => intermediate_rep.decimal_places.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "currencyCode" => intermediate_rep.currency_code.push(models::CurrencyCode::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Money".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Money {
            value: intermediate_rep.value.into_iter().next().ok_or("value missing in Money".to_string())?,
            decimal_places: intermediate_rep.decimal_places.into_iter().next().ok_or("decimalPlaces missing in Money".to_string())?,
            currency_code: intermediate_rep.currency_code.into_iter().next().ok_or("currencyCode missing in Money".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Monitor> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Monitor>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Monitor>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Monitor - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Monitor> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Monitor as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Monitor - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Monitor {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    pub type_: models::MonitorType,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "name")]
    pub name: String,

    /// Describes what this monitor checks.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "period")]
    pub period: String,

    #[serde(rename = "timeout")]
    pub timeout: String,

    #[serde(rename = "body")]
    pub body: models::MonitorBody,

}

impl Monitor {
    pub fn new(type_: models::MonitorType, name: String, period: String, timeout: String, body: models::MonitorBody, ) -> Monitor {
        Monitor {
            id: None,
            type_: type_,
            enabled: true,
            name: name,
            description: None,
            period: period,
            timeout: timeout,
            body: body,
        }
    }
}

/// Converts the Monitor value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Monitor {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }

        // Skipping type in query parameter serialization


        params.push("enabled".to_string());
        params.push(self.enabled.to_string());


        params.push("name".to_string());
        params.push(self.name.to_string());


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        params.push("period".to_string());
        params.push(self.period.to_string());


        params.push("timeout".to_string());
        params.push(self.timeout.to_string());

        // Skipping body in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Monitor value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Monitor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub type_: Vec<models::MonitorType>,
            pub enabled: Vec<bool>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub period: Vec<String>,
            pub timeout: Vec<String>,
            pub body: Vec<models::MonitorBody>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Monitor".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "type" => intermediate_rep.type_.push(models::MonitorType::from_str(val).map_err(|x| format!("{}", x))?),
                    "enabled" => intermediate_rep.enabled.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "period" => intermediate_rep.period.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "timeout" => intermediate_rep.timeout.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "body" => intermediate_rep.body.push(models::MonitorBody::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Monitor".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Monitor {
            id: intermediate_rep.id.into_iter().next(),
            type_: intermediate_rep.type_.into_iter().next().ok_or("type missing in Monitor".to_string())?,
            enabled: intermediate_rep.enabled.into_iter().next().ok_or("enabled missing in Monitor".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in Monitor".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            period: intermediate_rep.period.into_iter().next().ok_or("period missing in Monitor".to_string())?,
            timeout: intermediate_rep.timeout.into_iter().next().ok_or("timeout missing in Monitor".to_string())?,
            body: intermediate_rep.body.into_iter().next().ok_or("body missing in Monitor".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<MonitorArray> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonitorArray>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MonitorArray>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MonitorArray - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MonitorArray> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MonitorArray as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MonitorArray - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonitorArray {
    #[serde(rename = "monitors")]
    pub monitors: Vec<models::Monitor>,

}

impl MonitorArray {
    pub fn new(monitors: Vec<models::Monitor>, ) -> MonitorArray {
        MonitorArray {
            monitors: monitors,
        }
    }
}

/// Converts the MonitorArray value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonitorArray {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping monitors in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonitorArray value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonitorArray {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub monitors: Vec<Vec<models::Monitor>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MonitorArray".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "monitors" => return std::result::Result::Err("Parsing a container in this style is not supported in MonitorArray".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MonitorArray".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonitorArray {
            monitors: intermediate_rep.monitors.into_iter().next().ok_or("monitors missing in MonitorArray".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<MonitorBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonitorBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MonitorBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MonitorBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MonitorBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MonitorBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MonitorBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonitorBody {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "method")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub method: Option<String>,

    #[serde(rename = "headers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub headers: Option<Vec<models::HttpHeader>>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// The name of the executable process to be monitored.
    #[serde(rename = "executable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub executable: Option<String>,

    /// If true, the process(s) will be located by the full path of the executable e.g. /usr/bin/node
    #[serde(rename = "isPathAbsolute")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_path_absolute: Option<bool>,

    /// The minimum number of processes that match the executable.
    #[serde(rename = "minimumCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum_count: Option<isize>,

    /// The maximum number of processes that match the executable. 
    #[serde(rename = "maximumCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_count: Option<isize>,

    #[serde(rename = "maximumRamIndividual")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_ram_individual: Option<String>,

    #[serde(rename = "maximumRamTotal")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_ram_total: Option<String>,

    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,

    #[serde(rename = "port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<u16>,

    #[serde(rename = "db")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub db: Option<isize>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "expression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expression: Option<models::BoolExpr>,

}

impl MonitorBody {
    pub fn new() -> MonitorBody {
        MonitorBody {
            url: None,
            method: None,
            headers: None,
            body: None,
            executable: None,
            is_path_absolute: Some(false),
            minimum_count: None,
            maximum_count: None,
            maximum_ram_individual: None,
            maximum_ram_total: None,
            hostname: None,
            port: None,
            db: None,
            username: None,
            password: None,
            expression: None,
        }
    }
}

/// Converts the MonitorBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonitorBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref url) = self.url {
            params.push("url".to_string());
            params.push(url.to_string());
        }


        if let Some(ref method) = self.method {
            params.push("method".to_string());
            params.push(method.to_string());
        }

        // Skipping headers in query parameter serialization


        if let Some(ref body) = self.body {
            params.push("body".to_string());
            params.push(body.to_string());
        }


        if let Some(ref executable) = self.executable {
            params.push("executable".to_string());
            params.push(executable.to_string());
        }


        if let Some(ref is_path_absolute) = self.is_path_absolute {
            params.push("isPathAbsolute".to_string());
            params.push(is_path_absolute.to_string());
        }


        if let Some(ref minimum_count) = self.minimum_count {
            params.push("minimumCount".to_string());
            params.push(minimum_count.to_string());
        }


        if let Some(ref maximum_count) = self.maximum_count {
            params.push("maximumCount".to_string());
            params.push(maximum_count.to_string());
        }


        if let Some(ref maximum_ram_individual) = self.maximum_ram_individual {
            params.push("maximumRamIndividual".to_string());
            params.push(maximum_ram_individual.to_string());
        }


        if let Some(ref maximum_ram_total) = self.maximum_ram_total {
            params.push("maximumRamTotal".to_string());
            params.push(maximum_ram_total.to_string());
        }


        if let Some(ref hostname) = self.hostname {
            params.push("hostname".to_string());
            params.push(hostname.to_string());
        }


        if let Some(ref port) = self.port {
            params.push("port".to_string());
            params.push(port.to_string());
        }


        if let Some(ref db) = self.db {
            params.push("db".to_string());
            params.push(db.to_string());
        }


        if let Some(ref username) = self.username {
            params.push("username".to_string());
            params.push(username.to_string());
        }


        if let Some(ref password) = self.password {
            params.push("password".to_string());
            params.push(password.to_string());
        }

        // Skipping expression in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonitorBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonitorBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub url: Vec<String>,
            pub method: Vec<String>,
            pub headers: Vec<Vec<models::HttpHeader>>,
            pub body: Vec<String>,
            pub executable: Vec<String>,
            pub is_path_absolute: Vec<bool>,
            pub minimum_count: Vec<isize>,
            pub maximum_count: Vec<isize>,
            pub maximum_ram_individual: Vec<String>,
            pub maximum_ram_total: Vec<String>,
            pub hostname: Vec<String>,
            pub port: Vec<u16>,
            pub db: Vec<isize>,
            pub username: Vec<String>,
            pub password: Vec<String>,
            pub expression: Vec<models::BoolExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MonitorBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "url" => intermediate_rep.url.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "method" => intermediate_rep.method.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "headers" => return std::result::Result::Err("Parsing a container in this style is not supported in MonitorBody".to_string()),
                    "body" => intermediate_rep.body.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "executable" => intermediate_rep.executable.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "isPathAbsolute" => intermediate_rep.is_path_absolute.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "minimumCount" => intermediate_rep.minimum_count.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "maximumCount" => intermediate_rep.maximum_count.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "maximumRamIndividual" => intermediate_rep.maximum_ram_individual.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "maximumRamTotal" => intermediate_rep.maximum_ram_total.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "hostname" => intermediate_rep.hostname.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "port" => intermediate_rep.port.push(u16::from_str(val).map_err(|x| format!("{}", x))?),
                    "db" => intermediate_rep.db.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "expression" => intermediate_rep.expression.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MonitorBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonitorBody {
            url: intermediate_rep.url.into_iter().next(),
            method: intermediate_rep.method.into_iter().next(),
            headers: intermediate_rep.headers.into_iter().next(),
            body: intermediate_rep.body.into_iter().next(),
            executable: intermediate_rep.executable.into_iter().next(),
            is_path_absolute: intermediate_rep.is_path_absolute.into_iter().next(),
            minimum_count: intermediate_rep.minimum_count.into_iter().next(),
            maximum_count: intermediate_rep.maximum_count.into_iter().next(),
            maximum_ram_individual: intermediate_rep.maximum_ram_individual.into_iter().next(),
            maximum_ram_total: intermediate_rep.maximum_ram_total.into_iter().next(),
            hostname: intermediate_rep.hostname.into_iter().next(),
            port: intermediate_rep.port.into_iter().next(),
            db: intermediate_rep.db.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
            password: intermediate_rep.password.into_iter().next(),
            expression: intermediate_rep.expression.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<MonitorStatus> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonitorStatus>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MonitorStatus>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MonitorStatus - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MonitorStatus> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MonitorStatus as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MonitorStatus - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonitorStatus {
    #[serde(rename = "monitorId")]
    pub monitor_id: String,

    #[serde(rename = "status")]
    pub status: models::MonitorStatusIndicator,

    /// UTC UNIX timestamp in with fractional offset.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "lastResult")]
    pub last_result: models::MonitorStatusResult,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "log")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log: Option<Vec<models::MonitorStatusLogEntry>>,

}

impl MonitorStatus {
    pub fn new(monitor_id: String, status: models::MonitorStatusIndicator, timestamp: chrono::DateTime::<chrono::Utc>, last_result: models::MonitorStatusResult, description: String, ) -> MonitorStatus {
        MonitorStatus {
            monitor_id: monitor_id,
            status: status,
            timestamp: timestamp,
            last_result: last_result,
            description: description,
            log: None,
        }
    }
}

/// Converts the MonitorStatus value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonitorStatus {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("monitorId".to_string());
        params.push(self.monitor_id.to_string());

        // Skipping status in query parameter serialization

        // Skipping timestamp in query parameter serialization

        // Skipping lastResult in query parameter serialization


        params.push("description".to_string());
        params.push(self.description.to_string());

        // Skipping log in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonitorStatus value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonitorStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub monitor_id: Vec<String>,
            pub status: Vec<models::MonitorStatusIndicator>,
            pub timestamp: Vec<chrono::DateTime::<chrono::Utc>>,
            pub last_result: Vec<models::MonitorStatusResult>,
            pub description: Vec<String>,
            pub log: Vec<Vec<models::MonitorStatusLogEntry>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MonitorStatus".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "monitorId" => intermediate_rep.monitor_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "status" => intermediate_rep.status.push(models::MonitorStatusIndicator::from_str(val).map_err(|x| format!("{}", x))?),
                    "timestamp" => intermediate_rep.timestamp.push(chrono::DateTime::<chrono::Utc>::from_str(val).map_err(|x| format!("{}", x))?),
                    "lastResult" => intermediate_rep.last_result.push(models::MonitorStatusResult::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "log" => return std::result::Result::Err("Parsing a container in this style is not supported in MonitorStatus".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MonitorStatus".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonitorStatus {
            monitor_id: intermediate_rep.monitor_id.into_iter().next().ok_or("monitorId missing in MonitorStatus".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or("status missing in MonitorStatus".to_string())?,
            timestamp: intermediate_rep.timestamp.into_iter().next().ok_or("timestamp missing in MonitorStatus".to_string())?,
            last_result: intermediate_rep.last_result.into_iter().next().ok_or("lastResult missing in MonitorStatus".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or("description missing in MonitorStatus".to_string())?,
            log: intermediate_rep.log.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<MonitorStatusArray> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonitorStatusArray>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MonitorStatusArray>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MonitorStatusArray - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MonitorStatusArray> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MonitorStatusArray as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MonitorStatusArray - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonitorStatusArray {
    #[serde(rename = "statuses")]
    pub statuses: Vec<models::MonitorStatus>,

}

impl MonitorStatusArray {
    pub fn new(statuses: Vec<models::MonitorStatus>, ) -> MonitorStatusArray {
        MonitorStatusArray {
            statuses: statuses,
        }
    }
}

/// Converts the MonitorStatusArray value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonitorStatusArray {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping statuses in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonitorStatusArray value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonitorStatusArray {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub statuses: Vec<Vec<models::MonitorStatus>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MonitorStatusArray".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "statuses" => return std::result::Result::Err("Parsing a container in this style is not supported in MonitorStatusArray".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MonitorStatusArray".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonitorStatusArray {
            statuses: intermediate_rep.statuses.into_iter().next().ok_or("statuses missing in MonitorStatusArray".to_string())?,
        })
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum MonitorStatusIndicator { 
    #[serde(rename = "ok")]
    OK,
    #[serde(rename = "down")]
    DOWN,
}

impl std::fmt::Display for MonitorStatusIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            MonitorStatusIndicator::OK => write!(f, "{}", "ok"),
            MonitorStatusIndicator::DOWN => write!(f, "{}", "down"),
        }
    }
}

impl std::str::FromStr for MonitorStatusIndicator {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ok" => std::result::Result::Ok(MonitorStatusIndicator::OK),
            "down" => std::result::Result::Ok(MonitorStatusIndicator::DOWN),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<MonitorStatusLogEntry> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonitorStatusLogEntry>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MonitorStatusLogEntry>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MonitorStatusLogEntry - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MonitorStatusLogEntry> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MonitorStatusLogEntry as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MonitorStatusLogEntry - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonitorStatusLogEntry {
    /// UTC UNIX timestamp in with fractional offset.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "value")]
    pub value: String,

}

impl MonitorStatusLogEntry {
    pub fn new(timestamp: chrono::DateTime::<chrono::Utc>, value: String, ) -> MonitorStatusLogEntry {
        MonitorStatusLogEntry {
            timestamp: timestamp,
            value: value,
        }
    }
}

/// Converts the MonitorStatusLogEntry value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonitorStatusLogEntry {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping timestamp in query parameter serialization


        params.push("value".to_string());
        params.push(self.value.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonitorStatusLogEntry value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonitorStatusLogEntry {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub timestamp: Vec<chrono::DateTime::<chrono::Utc>>,
            pub value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MonitorStatusLogEntry".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "timestamp" => intermediate_rep.timestamp.push(chrono::DateTime::<chrono::Utc>::from_str(val).map_err(|x| format!("{}", x))?),
                    "value" => intermediate_rep.value.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MonitorStatusLogEntry".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonitorStatusLogEntry {
            timestamp: intermediate_rep.timestamp.into_iter().next().ok_or("timestamp missing in MonitorStatusLogEntry".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or("value missing in MonitorStatusLogEntry".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<MonitorStatusResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonitorStatusResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MonitorStatusResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MonitorStatusResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MonitorStatusResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MonitorStatusResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MonitorStatusResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonitorStatusResult {
    #[serde(rename = "expected")]
    pub expected: String,

    #[serde(rename = "actual")]
    pub actual: String,

}

impl MonitorStatusResult {
    pub fn new(expected: String, actual: String, ) -> MonitorStatusResult {
        MonitorStatusResult {
            expected: expected,
            actual: actual,
        }
    }
}

/// Converts the MonitorStatusResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonitorStatusResult {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("expected".to_string());
        params.push(self.expected.to_string());


        params.push("actual".to_string());
        params.push(self.actual.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonitorStatusResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonitorStatusResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub expected: Vec<String>,
            pub actual: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MonitorStatusResult".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "expected" => intermediate_rep.expected.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "actual" => intermediate_rep.actual.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MonitorStatusResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonitorStatusResult {
            expected: intermediate_rep.expected.into_iter().next().ok_or("expected missing in MonitorStatusResult".to_string())?,
            actual: intermediate_rep.actual.into_iter().next().ok_or("actual missing in MonitorStatusResult".to_string())?,
        })
    }
}



/// What sort of system or entity the monitor will run a check on
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum MonitorType { 
    #[serde(rename = "http")]
    HTTP,
    #[serde(rename = "process")]
    PROCESS,
    #[serde(rename = "tcp")]
    TCP,
}

impl std::fmt::Display for MonitorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            MonitorType::HTTP => write!(f, "{}", "http"),
            MonitorType::PROCESS => write!(f, "{}", "process"),
            MonitorType::TCP => write!(f, "{}", "tcp"),
        }
    }
}

impl std::str::FromStr for MonitorType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "http" => std::result::Result::Ok(MonitorType::HTTP),
            "process" => std::result::Result::Ok(MonitorType::PROCESS),
            "tcp" => std::result::Result::Ok(MonitorType::TCP),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<MsTeamsAlertBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MsTeamsAlertBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MsTeamsAlertBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MsTeamsAlertBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MsTeamsAlertBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MsTeamsAlertBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MsTeamsAlertBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MsTeamsAlertBody {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}

impl MsTeamsAlertBody {
    pub fn new() -> MsTeamsAlertBody {
        MsTeamsAlertBody {
            url: None,
        }
    }
}

/// Converts the MsTeamsAlertBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MsTeamsAlertBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref url) = self.url {
            params.push("url".to_string());
            params.push(url.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MsTeamsAlertBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MsTeamsAlertBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub url: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MsTeamsAlertBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "url" => intermediate_rep.url.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MsTeamsAlertBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MsTeamsAlertBody {
            url: intermediate_rep.url.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<MulExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MulExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MulExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MulExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MulExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MulExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MulExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MulExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl MulExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> MulExpr {
        MulExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the MulExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MulExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MulExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MulExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MulExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MulExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MulExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in MulExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in MulExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<NewAgent> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NewAgent>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewAgent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NewAgent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NewAgent> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewAgent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NewAgent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewAgent {
    #[serde(rename = "apiKey")]
    pub api_key: String,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    /// Describes what this agent is or where it will run.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "group")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<String>,

}

impl NewAgent {
    pub fn new(api_key: String, name: String, ) -> NewAgent {
        NewAgent {
            api_key: api_key,
            id: None,
            name: name,
            description: None,
            group: None,
        }
    }
}

/// Converts the NewAgent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NewAgent {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("apiKey".to_string());
        params.push(self.api_key.to_string());


        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        params.push("name".to_string());
        params.push(self.name.to_string());


        if let Some(ref description) = self.description {
            params.push("description".to_string());
            params.push(description.to_string());
        }


        if let Some(ref group) = self.group {
            params.push("group".to_string());
            params.push(group.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewAgent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewAgent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub api_key: Vec<String>,
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub group: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewAgent".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "apiKey" => intermediate_rep.api_key.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "group" => intermediate_rep.group.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewAgent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewAgent {
            api_key: intermediate_rep.api_key.into_iter().next().ok_or("apiKey missing in NewAgent".to_string())?,
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in NewAgent".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            group: intermediate_rep.group.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<NotExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NotExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NotExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NotExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NotExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NotExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NotExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NotExpr {
    #[serde(rename = "expr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expr: Option<models::BoolExpr>,

}

impl NotExpr {
    pub fn new() -> NotExpr {
        NotExpr {
            expr: None,
        }
    }
}

/// Converts the NotExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NotExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping expr in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NotExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NotExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub expr: Vec<models::BoolExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NotExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "expr" => intermediate_rep.expr.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NotExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NotExpr {
            expr: intermediate_rep.expr.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<OrExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OrExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OrExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OrExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OrExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OrExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OrExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OrExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::BoolExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::BoolExpr,

}

impl OrExpr {
    pub fn new(lhs: models::BoolExpr, rhs: models::BoolExpr, ) -> OrExpr {
        OrExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the OrExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OrExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OrExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OrExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::BoolExpr>,
            pub rhs: Vec<models::BoolExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OrExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing OrExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OrExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in OrExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in OrExpr".to_string())?,
        })
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Password(String);

impl std::convert::From<String> for Password {
    fn from(x: String) -> Self {
        Password(x)
    }
}

impl std::string::ToString for Password {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Password {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Password(x.to_string()))
    }
}

impl std::convert::From<Password> for String {
    fn from(x: Password) -> Self {
        x.0
    }
}

impl std::ops::Deref for Password {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Password {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



// Methods for converting between header::IntoHeaderValue<Plan> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Plan>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Plan>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Plan - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Plan> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Plan as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Plan - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Plan {
    #[serde(rename = "id")]
    pub id: String,

    /// Description of what this plan is for, and what it offers.
    #[serde(rename = "name")]
    pub name: String,

    /// Description of what this plan is for, and what it offers.
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "products")]
    pub products: Vec<models::Product>,

    #[serde(rename = "maximumPrice")]
    pub maximum_price: models::Money,

}

impl Plan {
    pub fn new(id: String, name: String, description: String, products: Vec<models::Product>, maximum_price: models::Money, ) -> Plan {
        Plan {
            id: id,
            name: name,
            description: description,
            products: products,
            maximum_price: maximum_price,
        }
    }
}

/// Converts the Plan value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Plan {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("id".to_string());
        params.push(self.id.to_string());


        params.push("name".to_string());
        params.push(self.name.to_string());


        params.push("description".to_string());
        params.push(self.description.to_string());

        // Skipping products in query parameter serialization

        // Skipping maximumPrice in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Plan value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Plan {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub products: Vec<Vec<models::Product>>,
            pub maximum_price: Vec<models::Money>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Plan".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "products" => return std::result::Result::Err("Parsing a container in this style is not supported in Plan".to_string()),
                    "maximumPrice" => intermediate_rep.maximum_price.push(models::Money::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Plan".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Plan {
            id: intermediate_rep.id.into_iter().next().ok_or("id missing in Plan".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or("name missing in Plan".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or("description missing in Plan".to_string())?,
            products: intermediate_rep.products.into_iter().next().ok_or("products missing in Plan".to_string())?,
            maximum_price: intermediate_rep.maximum_price.into_iter().next().ok_or("maximumPrice missing in Plan".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<PlanArray> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PlanArray>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PlanArray>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PlanArray - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PlanArray> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PlanArray as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PlanArray - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlanArray {
    #[serde(rename = "plans")]
    pub plans: Vec<models::Plan>,

}

impl PlanArray {
    pub fn new(plans: Vec<models::Plan>, ) -> PlanArray {
        PlanArray {
            plans: plans,
        }
    }
}

/// Converts the PlanArray value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PlanArray {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping plans in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlanArray value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlanArray {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub plans: Vec<Vec<models::Plan>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PlanArray".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "plans" => return std::result::Result::Err("Parsing a container in this style is not supported in PlanArray".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PlanArray".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlanArray {
            plans: intermediate_rep.plans.into_iter().next().ok_or("plans missing in PlanArray".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<PlusExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PlusExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PlusExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PlusExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PlusExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PlusExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PlusExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlusExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl PlusExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> PlusExpr {
        PlusExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the PlusExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PlusExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlusExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlusExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PlusExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PlusExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlusExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in PlusExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in PlusExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<ProcessMonitorBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ProcessMonitorBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ProcessMonitorBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ProcessMonitorBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ProcessMonitorBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ProcessMonitorBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ProcessMonitorBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProcessMonitorBody {
    /// The name of the executable process to be monitored.
    #[serde(rename = "executable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub executable: Option<String>,

    /// If true, the process(s) will be located by the full path of the executable e.g. /usr/bin/node
    #[serde(rename = "isPathAbsolute")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_path_absolute: Option<bool>,

    /// The minimum number of processes that match the executable.
    #[serde(rename = "minimumCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minimum_count: Option<isize>,

    /// The maximum number of processes that match the executable. 
    #[serde(rename = "maximumCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_count: Option<isize>,

    #[serde(rename = "maximumRamIndividual")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_ram_individual: Option<String>,

    #[serde(rename = "maximumRamTotal")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maximum_ram_total: Option<String>,

}

impl ProcessMonitorBody {
    pub fn new() -> ProcessMonitorBody {
        ProcessMonitorBody {
            executable: None,
            is_path_absolute: Some(false),
            minimum_count: None,
            maximum_count: None,
            maximum_ram_individual: None,
            maximum_ram_total: None,
        }
    }
}

/// Converts the ProcessMonitorBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ProcessMonitorBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref executable) = self.executable {
            params.push("executable".to_string());
            params.push(executable.to_string());
        }


        if let Some(ref is_path_absolute) = self.is_path_absolute {
            params.push("isPathAbsolute".to_string());
            params.push(is_path_absolute.to_string());
        }


        if let Some(ref minimum_count) = self.minimum_count {
            params.push("minimumCount".to_string());
            params.push(minimum_count.to_string());
        }


        if let Some(ref maximum_count) = self.maximum_count {
            params.push("maximumCount".to_string());
            params.push(maximum_count.to_string());
        }


        if let Some(ref maximum_ram_individual) = self.maximum_ram_individual {
            params.push("maximumRamIndividual".to_string());
            params.push(maximum_ram_individual.to_string());
        }


        if let Some(ref maximum_ram_total) = self.maximum_ram_total {
            params.push("maximumRamTotal".to_string());
            params.push(maximum_ram_total.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ProcessMonitorBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ProcessMonitorBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub executable: Vec<String>,
            pub is_path_absolute: Vec<bool>,
            pub minimum_count: Vec<isize>,
            pub maximum_count: Vec<isize>,
            pub maximum_ram_individual: Vec<String>,
            pub maximum_ram_total: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ProcessMonitorBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "executable" => intermediate_rep.executable.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "isPathAbsolute" => intermediate_rep.is_path_absolute.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "minimumCount" => intermediate_rep.minimum_count.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "maximumCount" => intermediate_rep.maximum_count.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "maximumRamIndividual" => intermediate_rep.maximum_ram_individual.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "maximumRamTotal" => intermediate_rep.maximum_ram_total.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ProcessMonitorBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ProcessMonitorBody {
            executable: intermediate_rep.executable.into_iter().next(),
            is_path_absolute: intermediate_rep.is_path_absolute.into_iter().next(),
            minimum_count: intermediate_rep.minimum_count.into_iter().next(),
            maximum_count: intermediate_rep.maximum_count.into_iter().next(),
            maximum_ram_individual: intermediate_rep.maximum_ram_individual.into_iter().next(),
            maximum_ram_total: intermediate_rep.maximum_ram_total.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Product> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Product>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Product>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Product - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Product> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Product as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Product - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Product {
    #[serde(rename = "entity")]
    pub entity: models::ProductEntity,

    #[serde(rename = "price")]
    pub price: models::Money,

    #[serde(rename = "count")]
    pub count: i32,

    /// 15 monitors for free.
    #[serde(rename = "description")]
    pub description: String,

}

impl Product {
    pub fn new(entity: models::ProductEntity, price: models::Money, count: i32, description: String, ) -> Product {
        Product {
            entity: entity,
            price: price,
            count: count,
            description: description,
        }
    }
}

/// Converts the Product value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Product {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping entity in query parameter serialization

        // Skipping price in query parameter serialization


        params.push("count".to_string());
        params.push(self.count.to_string());


        params.push("description".to_string());
        params.push(self.description.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Product value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Product {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub entity: Vec<models::ProductEntity>,
            pub price: Vec<models::Money>,
            pub count: Vec<i32>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Product".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "entity" => intermediate_rep.entity.push(models::ProductEntity::from_str(val).map_err(|x| format!("{}", x))?),
                    "price" => intermediate_rep.price.push(models::Money::from_str(val).map_err(|x| format!("{}", x))?),
                    "count" => intermediate_rep.count.push(i32::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Product".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Product {
            entity: intermediate_rep.entity.into_iter().next().ok_or("entity missing in Product".to_string())?,
            price: intermediate_rep.price.into_iter().next().ok_or("price missing in Product".to_string())?,
            count: intermediate_rep.count.into_iter().next().ok_or("count missing in Product".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or("description missing in Product".to_string())?,
        })
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ProductEntity { 
    #[serde(rename = "monitor")]
    MONITOR,
    #[serde(rename = "status")]
    STATUS,
    #[serde(rename = "alert")]
    ALERT,
    #[serde(rename = "agent")]
    AGENT,
    #[serde(rename = "user")]
    USER,
    #[serde(rename = "session")]
    SESSION,
}

impl std::fmt::Display for ProductEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            ProductEntity::MONITOR => write!(f, "{}", "monitor"),
            ProductEntity::STATUS => write!(f, "{}", "status"),
            ProductEntity::ALERT => write!(f, "{}", "alert"),
            ProductEntity::AGENT => write!(f, "{}", "agent"),
            ProductEntity::USER => write!(f, "{}", "user"),
            ProductEntity::SESSION => write!(f, "{}", "session"),
        }
    }
}

impl std::str::FromStr for ProductEntity {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "monitor" => std::result::Result::Ok(ProductEntity::MONITOR),
            "status" => std::result::Result::Ok(ProductEntity::STATUS),
            "alert" => std::result::Result::Ok(ProductEntity::ALERT),
            "agent" => std::result::Result::Ok(ProductEntity::AGENT),
            "user" => std::result::Result::Ok(ProductEntity::USER),
            "session" => std::result::Result::Ok(ProductEntity::SESSION),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<RedisMonitorBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RedisMonitorBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RedisMonitorBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RedisMonitorBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RedisMonitorBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RedisMonitorBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RedisMonitorBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RedisMonitorBody {
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,

    #[serde(rename = "port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<u16>,

    #[serde(rename = "db")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub db: Option<isize>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "expression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expression: Option<models::BoolExpr>,

}

impl RedisMonitorBody {
    pub fn new() -> RedisMonitorBody {
        RedisMonitorBody {
            hostname: None,
            port: None,
            db: None,
            username: None,
            password: None,
            expression: None,
        }
    }
}

/// Converts the RedisMonitorBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RedisMonitorBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref hostname) = self.hostname {
            params.push("hostname".to_string());
            params.push(hostname.to_string());
        }


        if let Some(ref port) = self.port {
            params.push("port".to_string());
            params.push(port.to_string());
        }


        if let Some(ref db) = self.db {
            params.push("db".to_string());
            params.push(db.to_string());
        }


        if let Some(ref username) = self.username {
            params.push("username".to_string());
            params.push(username.to_string());
        }


        if let Some(ref password) = self.password {
            params.push("password".to_string());
            params.push(password.to_string());
        }

        // Skipping expression in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RedisMonitorBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RedisMonitorBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub hostname: Vec<String>,
            pub port: Vec<u16>,
            pub db: Vec<isize>,
            pub username: Vec<String>,
            pub password: Vec<String>,
            pub expression: Vec<models::BoolExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RedisMonitorBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "hostname" => intermediate_rep.hostname.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "port" => intermediate_rep.port.push(u16::from_str(val).map_err(|x| format!("{}", x))?),
                    "db" => intermediate_rep.db.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "username" => intermediate_rep.username.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "expression" => intermediate_rep.expression.push(models::BoolExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RedisMonitorBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RedisMonitorBody {
            hostname: intermediate_rep.hostname.into_iter().next(),
            port: intermediate_rep.port.into_iter().next(),
            db: intermediate_rep.db.into_iter().next(),
            username: intermediate_rep.username.into_iter().next(),
            password: intermediate_rep.password.into_iter().next(),
            expression: intermediate_rep.expression.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Registration> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Registration>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Registration>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Registration - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Registration> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Registration as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Registration - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Registration {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "accountName")]
    pub account_name: String,

    #[serde(rename = "challengeId")]
    pub challenge_id: String,

    #[serde(rename = "challengeSolution")]
    pub challenge_solution: String,

    #[serde(rename = "emailAddress")]
    pub email_address: String,

    #[serde(rename = "password")]
    pub password: String,

}

impl Registration {
    pub fn new(account_name: String, challenge_id: String, challenge_solution: String, email_address: String, password: String, ) -> Registration {
        Registration {
            id: None,
            account_name: account_name,
            challenge_id: challenge_id,
            challenge_solution: challenge_solution,
            email_address: email_address,
            password: password,
        }
    }
}

/// Converts the Registration value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Registration {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        params.push("accountName".to_string());
        params.push(self.account_name.to_string());


        params.push("challengeId".to_string());
        params.push(self.challenge_id.to_string());


        params.push("challengeSolution".to_string());
        params.push(self.challenge_solution.to_string());


        params.push("emailAddress".to_string());
        params.push(self.email_address.to_string());


        params.push("password".to_string());
        params.push(self.password.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Registration value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Registration {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub account_name: Vec<String>,
            pub challenge_id: Vec<String>,
            pub challenge_solution: Vec<String>,
            pub email_address: Vec<String>,
            pub password: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Registration".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "accountName" => intermediate_rep.account_name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "challengeId" => intermediate_rep.challenge_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "challengeSolution" => intermediate_rep.challenge_solution.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "emailAddress" => intermediate_rep.email_address.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "password" => intermediate_rep.password.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Registration".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Registration {
            id: intermediate_rep.id.into_iter().next(),
            account_name: intermediate_rep.account_name.into_iter().next().ok_or("accountName missing in Registration".to_string())?,
            challenge_id: intermediate_rep.challenge_id.into_iter().next().ok_or("challengeId missing in Registration".to_string())?,
            challenge_solution: intermediate_rep.challenge_solution.into_iter().next().ok_or("challengeSolution missing in Registration".to_string())?,
            email_address: intermediate_rep.email_address.into_iter().next().ok_or("emailAddress missing in Registration".to_string())?,
            password: intermediate_rep.password.into_iter().next().ok_or("password missing in Registration".to_string())?,
        })
    }
}



/// A Captcha challenge that a human being will have to solve.
// Methods for converting between header::IntoHeaderValue<RegistrationChallenge> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RegistrationChallenge>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RegistrationChallenge>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RegistrationChallenge - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RegistrationChallenge> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RegistrationChallenge as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RegistrationChallenge - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegistrationChallenge {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    /// The solution that will activate the challenge. This can be used to prove you are human.
    #[serde(rename = "solution")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub solution: Option<String>,

    /// What the user has to do to pass the challenge. For now this is just `image`.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub type_: String,

    /// Since this is a JSON api, the image data has to be encoded in some way. For now image data is encoded in base 64.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "encoding")]
    pub encoding: String,

    /// Describes how to interpret the `data` field.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "mimeType")]
    pub mime_type: String,

    /// The URL for the image, if there is one.
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<String>,

}

impl RegistrationChallenge {
    pub fn new(type_: String, encoding: String, mime_type: String, ) -> RegistrationChallenge {
        RegistrationChallenge {
            id: None,
            status: None,
            solution: None,
            type_: type_,
            encoding: encoding,
            mime_type: mime_type,
            url: None,
            data: None,
        }
    }
}

/// Converts the RegistrationChallenge value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RegistrationChallenge {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        if let Some(ref status) = self.status {
            params.push("status".to_string());
            params.push(status.to_string());
        }


        if let Some(ref solution) = self.solution {
            params.push("solution".to_string());
            params.push(solution.to_string());
        }


        params.push("type".to_string());
        params.push(self.type_.to_string());


        params.push("encoding".to_string());
        params.push(self.encoding.to_string());


        params.push("mimeType".to_string());
        params.push(self.mime_type.to_string());


        if let Some(ref url) = self.url {
            params.push("url".to_string());
            params.push(url.to_string());
        }


        if let Some(ref data) = self.data {
            params.push("data".to_string());
            params.push(data.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RegistrationChallenge value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RegistrationChallenge {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub status: Vec<String>,
            pub solution: Vec<String>,
            pub type_: Vec<String>,
            pub encoding: Vec<String>,
            pub mime_type: Vec<String>,
            pub url: Vec<String>,
            pub data: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RegistrationChallenge".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "status" => intermediate_rep.status.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "solution" => intermediate_rep.solution.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "type" => intermediate_rep.type_.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "encoding" => intermediate_rep.encoding.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "mimeType" => intermediate_rep.mime_type.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "url" => intermediate_rep.url.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "data" => intermediate_rep.data.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RegistrationChallenge".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RegistrationChallenge {
            id: intermediate_rep.id.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            solution: intermediate_rep.solution.into_iter().next(),
            type_: intermediate_rep.type_.into_iter().next().ok_or("type missing in RegistrationChallenge".to_string())?,
            encoding: intermediate_rep.encoding.into_iter().next().ok_or("encoding missing in RegistrationChallenge".to_string())?,
            mime_type: intermediate_rep.mime_type.into_iter().next().ok_or("mimeType missing in RegistrationChallenge".to_string())?,
            url: intermediate_rep.url.into_iter().next(),
            data: intermediate_rep.data.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<RegistrationConfirmation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RegistrationConfirmation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RegistrationConfirmation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RegistrationConfirmation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RegistrationConfirmation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RegistrationConfirmation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RegistrationConfirmation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegistrationConfirmation {
    #[serde(rename = "id")]
    pub id: String,

    /// A secret string between the client and the API. A valid confirmation code is proof the user owns the email address
    #[serde(rename = "confirmationCode")]
    pub confirmation_code: String,

    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_address: Option<String>,

}

impl RegistrationConfirmation {
    pub fn new(id: String, confirmation_code: String, ) -> RegistrationConfirmation {
        RegistrationConfirmation {
            id: id,
            confirmation_code: confirmation_code,
            email_address: None,
        }
    }
}

/// Converts the RegistrationConfirmation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RegistrationConfirmation {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("id".to_string());
        params.push(self.id.to_string());


        params.push("confirmationCode".to_string());
        params.push(self.confirmation_code.to_string());


        if let Some(ref email_address) = self.email_address {
            params.push("emailAddress".to_string());
            params.push(email_address.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RegistrationConfirmation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RegistrationConfirmation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub confirmation_code: Vec<String>,
            pub email_address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RegistrationConfirmation".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "confirmationCode" => intermediate_rep.confirmation_code.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "emailAddress" => intermediate_rep.email_address.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RegistrationConfirmation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RegistrationConfirmation {
            id: intermediate_rep.id.into_iter().next().ok_or("id missing in RegistrationConfirmation".to_string())?,
            confirmation_code: intermediate_rep.confirmation_code.into_iter().next().ok_or("confirmationCode missing in RegistrationConfirmation".to_string())?,
            email_address: intermediate_rep.email_address.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<ResponseError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ResponseError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ResponseError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ResponseError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ResponseError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ResponseError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ResponseError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResponseError {
    #[serde(rename = "errorCode")]
    pub error_code: models::ErrorCode,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(rename = "userErrorMessage")]
    pub user_error_message: String,

}

impl ResponseError {
    pub fn new(error_code: models::ErrorCode, error_message: String, user_error_message: String, ) -> ResponseError {
        ResponseError {
            error_code: error_code,
            error_message: error_message,
            user_error_message: user_error_message,
        }
    }
}

/// Converts the ResponseError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ResponseError {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping errorCode in query parameter serialization


        params.push("errorMessage".to_string());
        params.push(self.error_message.to_string());


        params.push("userErrorMessage".to_string());
        params.push(self.user_error_message.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResponseError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResponseError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub error_code: Vec<models::ErrorCode>,
            pub error_message: Vec<String>,
            pub user_error_message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ResponseError".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "errorCode" => intermediate_rep.error_code.push(models::ErrorCode::from_str(val).map_err(|x| format!("{}", x))?),
                    "errorMessage" => intermediate_rep.error_message.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "userErrorMessage" => intermediate_rep.user_error_message.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ResponseError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResponseError {
            error_code: intermediate_rep.error_code.into_iter().next().ok_or("errorCode missing in ResponseError".to_string())?,
            error_message: intermediate_rep.error_message.into_iter().next().ok_or("errorMessage missing in ResponseError".to_string())?,
            user_error_message: intermediate_rep.user_error_message.into_iter().next().ok_or("userErrorMessage missing in ResponseError".to_string())?,
        })
    }
}



/// Server info and status about open connections
// Methods for converting between header::IntoHeaderValue<ServerInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ServerInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ServerInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServerInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ServerInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServerInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServerInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServerInfo {
    #[serde(rename = "vcsRef")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_ref: Option<String>,

    #[serde(rename = "processMemoryBytes")]
    pub process_memory_bytes: f64,

    #[serde(rename = "openInwardConnections")]
    pub open_inward_connections: f64,

}

impl ServerInfo {
    pub fn new(process_memory_bytes: f64, open_inward_connections: f64, ) -> ServerInfo {
        ServerInfo {
            vcs_ref: None,
            process_memory_bytes: process_memory_bytes,
            open_inward_connections: open_inward_connections,
        }
    }
}

/// Converts the ServerInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ServerInfo {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref vcs_ref) = self.vcs_ref {
            params.push("vcsRef".to_string());
            params.push(vcs_ref.to_string());
        }


        params.push("processMemoryBytes".to_string());
        params.push(self.process_memory_bytes.to_string());


        params.push("openInwardConnections".to_string());
        params.push(self.open_inward_connections.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServerInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServerInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub vcs_ref: Vec<String>,
            pub process_memory_bytes: Vec<f64>,
            pub open_inward_connections: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ServerInfo".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "vcsRef" => intermediate_rep.vcs_ref.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "processMemoryBytes" => intermediate_rep.process_memory_bytes.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    "openInwardConnections" => intermediate_rep.open_inward_connections.push(f64::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServerInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServerInfo {
            vcs_ref: intermediate_rep.vcs_ref.into_iter().next(),
            process_memory_bytes: intermediate_rep.process_memory_bytes.into_iter().next().ok_or("processMemoryBytes missing in ServerInfo".to_string())?,
            open_inward_connections: intermediate_rep.open_inward_connections.into_iter().next().ok_or("openInwardConnections missing in ServerInfo".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<SubExpr> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SubExpr>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SubExpr>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SubExpr - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SubExpr> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SubExpr as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SubExpr - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SubExpr {
    #[serde(rename = "lhs")]
    pub lhs: models::ArthExpr,

    #[serde(rename = "rhs")]
    pub rhs: models::ArthExpr,

}

impl SubExpr {
    pub fn new(lhs: models::ArthExpr, rhs: models::ArthExpr, ) -> SubExpr {
        SubExpr {
            lhs: lhs,
            rhs: rhs,
        }
    }
}

/// Converts the SubExpr value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SubExpr {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping lhs in query parameter serialization

        // Skipping rhs in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SubExpr value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SubExpr {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lhs: Vec<models::ArthExpr>,
            pub rhs: Vec<models::ArthExpr>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SubExpr".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lhs" => intermediate_rep.lhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    "rhs" => intermediate_rep.rhs.push(models::ArthExpr::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SubExpr".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SubExpr {
            lhs: intermediate_rep.lhs.into_iter().next().ok_or("lhs missing in SubExpr".to_string())?,
            rhs: intermediate_rep.rhs.into_iter().next().ok_or("rhs missing in SubExpr".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<Subscription> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Subscription>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Subscription>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Subscription - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Subscription> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Subscription as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Subscription - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Subscription {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "organisationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organisation_id: Option<String>,

    /// UTC UNIX timestamp in with fractional offset.
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime::<chrono::Utc>>,

    /// UTC UNIX timestamp in with fractional offset.
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stopped_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "planId")]
    pub plan_id: String,

    #[serde(rename = "chargeType")]
    pub charge_type: models::ChargeType,

}

impl Subscription {
    pub fn new(plan_id: String, charge_type: models::ChargeType, ) -> Subscription {
        Subscription {
            id: None,
            organisation_id: None,
            started_at: None,
            stopped_at: None,
            plan_id: plan_id,
            charge_type: charge_type,
        }
    }
}

/// Converts the Subscription value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Subscription {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        if let Some(ref organisation_id) = self.organisation_id {
            params.push("organisationId".to_string());
            params.push(organisation_id.to_string());
        }

        // Skipping startedAt in query parameter serialization

        // Skipping stoppedAt in query parameter serialization


        params.push("planId".to_string());
        params.push(self.plan_id.to_string());

        // Skipping chargeType in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Subscription value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Subscription {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub organisation_id: Vec<String>,
            pub started_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub stopped_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub plan_id: Vec<String>,
            pub charge_type: Vec<models::ChargeType>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Subscription".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "organisationId" => intermediate_rep.organisation_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "startedAt" => intermediate_rep.started_at.push(chrono::DateTime::<chrono::Utc>::from_str(val).map_err(|x| format!("{}", x))?),
                    "stoppedAt" => intermediate_rep.stopped_at.push(chrono::DateTime::<chrono::Utc>::from_str(val).map_err(|x| format!("{}", x))?),
                    "planId" => intermediate_rep.plan_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "chargeType" => intermediate_rep.charge_type.push(models::ChargeType::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Subscription".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Subscription {
            id: intermediate_rep.id.into_iter().next(),
            organisation_id: intermediate_rep.organisation_id.into_iter().next(),
            started_at: intermediate_rep.started_at.into_iter().next(),
            stopped_at: intermediate_rep.stopped_at.into_iter().next(),
            plan_id: intermediate_rep.plan_id.into_iter().next().ok_or("planId missing in Subscription".to_string())?,
            charge_type: intermediate_rep.charge_type.into_iter().next().ok_or("chargeType missing in Subscription".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<SubscriptionContainer> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SubscriptionContainer>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SubscriptionContainer>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SubscriptionContainer - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SubscriptionContainer> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SubscriptionContainer as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SubscriptionContainer - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SubscriptionContainer {
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription: Option<models::Subscription>,

}

impl SubscriptionContainer {
    pub fn new() -> SubscriptionContainer {
        SubscriptionContainer {
            subscription: None,
        }
    }
}

/// Converts the SubscriptionContainer value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SubscriptionContainer {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping subscription in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SubscriptionContainer value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SubscriptionContainer {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub subscription: Vec<models::Subscription>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SubscriptionContainer".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "subscription" => intermediate_rep.subscription.push(models::Subscription::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SubscriptionContainer".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SubscriptionContainer {
            subscription: intermediate_rep.subscription.into_iter().next(),
        })
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum SubscriptionStatus { 
    #[serde(rename = "active")]
    ACTIVE,
    #[serde(rename = "inactive")]
    INACTIVE,
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            SubscriptionStatus::ACTIVE => write!(f, "{}", "active"),
            SubscriptionStatus::INACTIVE => write!(f, "{}", "inactive"),
        }
    }
}

impl std::str::FromStr for SubscriptionStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "active" => std::result::Result::Ok(SubscriptionStatus::ACTIVE),
            "inactive" => std::result::Result::Ok(SubscriptionStatus::INACTIVE),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<TcpMonitorBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TcpMonitorBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TcpMonitorBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TcpMonitorBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TcpMonitorBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TcpMonitorBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TcpMonitorBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TcpMonitorBody {
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hostname: Option<String>,

    #[serde(rename = "port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<u16>,

}

impl TcpMonitorBody {
    pub fn new() -> TcpMonitorBody {
        TcpMonitorBody {
            hostname: None,
            port: None,
        }
    }
}

/// Converts the TcpMonitorBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TcpMonitorBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref hostname) = self.hostname {
            params.push("hostname".to_string());
            params.push(hostname.to_string());
        }


        if let Some(ref port) = self.port {
            params.push("port".to_string());
            params.push(port.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TcpMonitorBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TcpMonitorBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub hostname: Vec<String>,
            pub port: Vec<u16>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TcpMonitorBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "hostname" => intermediate_rep.hostname.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "port" => intermediate_rep.port.push(u16::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TcpMonitorBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TcpMonitorBody {
            hostname: intermediate_rep.hostname.into_iter().next(),
            port: intermediate_rep.port.into_iter().next(),
        })
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimeDelta(String);

impl std::convert::From<String> for TimeDelta {
    fn from(x: String) -> Self {
        TimeDelta(x)
    }
}

impl std::string::ToString for TimeDelta {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for TimeDelta {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TimeDelta(x.to_string()))
    }
}

impl std::convert::From<TimeDelta> for String {
    fn from(x: TimeDelta) -> Self {
        x.0
    }
}

impl std::ops::Deref for TimeDelta {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TimeDelta {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// Controls whether or not STARTTLS will be used.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TlsMode { 
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "tls")]
    TLS,
    #[serde(rename = "starttls")]
    STARTTLS,
}

impl std::fmt::Display for TlsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            TlsMode::NONE => write!(f, "{}", "none"),
            TlsMode::TLS => write!(f, "{}", "tls"),
            TlsMode::STARTTLS => write!(f, "{}", "starttls"),
        }
    }
}

impl std::str::FromStr for TlsMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "none" => std::result::Result::Ok(TlsMode::NONE),
            "tls" => std::result::Result::Ok(TlsMode::TLS),
            "starttls" => std::result::Result::Ok(TlsMode::STARTTLS),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<Transaction> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Transaction>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Transaction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Transaction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Transaction> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Transaction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Transaction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Transaction {
    #[serde(rename = "id")]
    pub id: String,

    /// UTC UNIX timestamp in with fractional offset.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "amount")]
    pub amount: models::Money,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "code")]
    pub code: models::TransactionCode,

}

impl Transaction {
    pub fn new(id: String, timestamp: chrono::DateTime::<chrono::Utc>, amount: models::Money, description: String, code: models::TransactionCode, ) -> Transaction {
        Transaction {
            id: id,
            timestamp: timestamp,
            amount: amount,
            description: description,
            code: code,
        }
    }
}

/// Converts the Transaction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Transaction {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("id".to_string());
        params.push(self.id.to_string());

        // Skipping timestamp in query parameter serialization

        // Skipping amount in query parameter serialization


        params.push("description".to_string());
        params.push(self.description.to_string());

        // Skipping code in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Transaction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Transaction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub timestamp: Vec<chrono::DateTime::<chrono::Utc>>,
            pub amount: Vec<models::Money>,
            pub description: Vec<String>,
            pub code: Vec<models::TransactionCode>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Transaction".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "timestamp" => intermediate_rep.timestamp.push(chrono::DateTime::<chrono::Utc>::from_str(val).map_err(|x| format!("{}", x))?),
                    "amount" => intermediate_rep.amount.push(models::Money::from_str(val).map_err(|x| format!("{}", x))?),
                    "description" => intermediate_rep.description.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "code" => intermediate_rep.code.push(models::TransactionCode::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Transaction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Transaction {
            id: intermediate_rep.id.into_iter().next().ok_or("id missing in Transaction".to_string())?,
            timestamp: intermediate_rep.timestamp.into_iter().next().ok_or("timestamp missing in Transaction".to_string())?,
            amount: intermediate_rep.amount.into_iter().next().ok_or("amount missing in Transaction".to_string())?,
            description: intermediate_rep.description.into_iter().next().ok_or("description missing in Transaction".to_string())?,
            code: intermediate_rep.code.into_iter().next().ok_or("code missing in Transaction".to_string())?,
        })
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TransactionCode { 
    #[serde(rename = "subscription_credit")]
    SUBSCRIPTION_CREDIT,
    #[serde(rename = "subscription_charge")]
    SUBSCRIPTION_CHARGE,
    #[serde(rename = "subscription_refund")]
    SUBSCRIPTION_REFUND,
    #[serde(rename = "subscription_correction")]
    SUBSCRIPTION_CORRECTION,
    #[serde(rename = "subscription_adjustment")]
    SUBSCRIPTION_ADJUSTMENT,
    #[serde(rename = "opening_balance")]
    OPENING_BALANCE,
}

impl std::fmt::Display for TransactionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            TransactionCode::SUBSCRIPTION_CREDIT => write!(f, "{}", "subscription_credit"),
            TransactionCode::SUBSCRIPTION_CHARGE => write!(f, "{}", "subscription_charge"),
            TransactionCode::SUBSCRIPTION_REFUND => write!(f, "{}", "subscription_refund"),
            TransactionCode::SUBSCRIPTION_CORRECTION => write!(f, "{}", "subscription_correction"),
            TransactionCode::SUBSCRIPTION_ADJUSTMENT => write!(f, "{}", "subscription_adjustment"),
            TransactionCode::OPENING_BALANCE => write!(f, "{}", "opening_balance"),
        }
    }
}

impl std::str::FromStr for TransactionCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "subscription_credit" => std::result::Result::Ok(TransactionCode::SUBSCRIPTION_CREDIT),
            "subscription_charge" => std::result::Result::Ok(TransactionCode::SUBSCRIPTION_CHARGE),
            "subscription_refund" => std::result::Result::Ok(TransactionCode::SUBSCRIPTION_REFUND),
            "subscription_correction" => std::result::Result::Ok(TransactionCode::SUBSCRIPTION_CORRECTION),
            "subscription_adjustment" => std::result::Result::Ok(TransactionCode::SUBSCRIPTION_ADJUSTMENT),
            "opening_balance" => std::result::Result::Ok(TransactionCode::OPENING_BALANCE),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}


// Methods for converting between header::IntoHeaderValue<TransactionList> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TransactionList>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TransactionList>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TransactionList - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TransactionList> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TransactionList as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TransactionList - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TransactionList {
    #[serde(rename = "transactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transactions: Option<Vec<models::Transaction>>,

    #[serde(rename = "openingBalance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub opening_balance: Option<models::Balance>,

    #[serde(rename = "closingBalance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closing_balance: Option<models::Balance>,

}

impl TransactionList {
    pub fn new() -> TransactionList {
        TransactionList {
            transactions: None,
            opening_balance: None,
            closing_balance: None,
        }
    }
}

/// Converts the TransactionList value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TransactionList {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping transactions in query parameter serialization

        // Skipping openingBalance in query parameter serialization

        // Skipping closingBalance in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TransactionList value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TransactionList {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub transactions: Vec<Vec<models::Transaction>>,
            pub opening_balance: Vec<models::Balance>,
            pub closing_balance: Vec<models::Balance>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TransactionList".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "transactions" => return std::result::Result::Err("Parsing a container in this style is not supported in TransactionList".to_string()),
                    "openingBalance" => intermediate_rep.opening_balance.push(models::Balance::from_str(val).map_err(|x| format!("{}", x))?),
                    "closingBalance" => intermediate_rep.closing_balance.push(models::Balance::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TransactionList".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TransactionList {
            transactions: intermediate_rep.transactions.into_iter().next(),
            opening_balance: intermediate_rep.opening_balance.into_iter().next(),
            closing_balance: intermediate_rep.closing_balance.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<WebhookAlertBody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WebhookAlertBody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WebhookAlertBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WebhookAlertBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WebhookAlertBody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WebhookAlertBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WebhookAlertBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WebhookAlertBody {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "headers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub headers: Option<Vec<models::HttpHeader>>,

}

impl WebhookAlertBody {
    pub fn new() -> WebhookAlertBody {
        WebhookAlertBody {
            url: None,
            headers: None,
        }
    }
}

/// Converts the WebhookAlertBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WebhookAlertBody {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref url) = self.url {
            params.push("url".to_string());
            params.push(url.to_string());
        }

        // Skipping headers in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WebhookAlertBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WebhookAlertBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub url: Vec<String>,
            pub headers: Vec<Vec<models::HttpHeader>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WebhookAlertBody".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "url" => intermediate_rep.url.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "headers" => return std::result::Result::Err("Parsing a container in this style is not supported in WebhookAlertBody".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing WebhookAlertBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WebhookAlertBody {
            url: intermediate_rep.url.into_iter().next(),
            headers: intermediate_rep.headers.into_iter().next(),
        })
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WriteableIdString(String);

impl std::convert::From<String> for WriteableIdString {
    fn from(x: String) -> Self {
        WriteableIdString(x)
    }
}

impl std::string::ToString for WriteableIdString {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for WriteableIdString {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(WriteableIdString(x.to_string()))
    }
}

impl std::convert::From<WriteableIdString> for String {
    fn from(x: WriteableIdString) -> Self {
        x.0
    }
}

impl std::ops::Deref for WriteableIdString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for WriteableIdString {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


