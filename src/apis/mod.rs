use std::error;
use std::fmt;
use serde::de::DeserializeOwned;
use crate::utils::ApiError;

impl<T> ApiError for T where T: DeserializeOwned + Send + Sync + std::fmt::Debug + 'static {}

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}
#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
        ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}
impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
        Error::Reqwest(e) => ("reqwest", e.to_string()),
        Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
        Error::Serde(e) => ("serde", e.to_string()),
        Error::Io(e) => ("IO", e.to_string()),
        Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}
impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
    Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
    Error::ReqwestMiddleware(e)
    }
}
    impl<T> From<serde_json::Error> for Error<T> {
        fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value:&serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                            &format!("{}[{}]", prefix, key),
                            value,
                        )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                    params.append(&mut parse_deep_object(
                        &format!("{}[{}][{}]", prefix, key, i),
                        value,
                        ));
                    }
                },
                serde_json::Value::String(s) =>
                    params.push((format!("{}[{}]", prefix, key),
                        s.clone())),
                _ => params.push((format!("{}[{}]", prefix,
                    key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with
style=deepObject")
}

#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return
            Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod anti_affinity_group_api;
pub mod block_storage_api;
pub mod ccm_api;
pub mod compute_api;
pub mod dbaas_api;
pub mod deploy_target_api;
pub mod dns_api;
pub mod elastic_ip_api;
pub mod event_api;
pub mod iam_api;
pub mod instance_api;
pub mod instance_pool_api;
pub mod instance_type_api;
pub mod network_load_balancer_api;
pub mod operation_api;
pub mod organization_api;
pub mod private_network_api;
pub mod quotas_api;
pub mod reverse_dns_api;
pub mod security_group_api;
pub mod sks_api;
pub mod snapshot_api;
pub mod sos_api;
pub mod ssh_key_api;
pub mod template_api;
pub mod zone_api;
pub mod configuration;
