use crate::apis::configuration::Configuration;
use crate::apis::{Error, ResponseContent}; // Your main error types from src/lib.rs or generated error handling
use crate::middleware::OriginalBodyBytes; // Your middleware helper from src/middleware.rs
use bytes::Bytes;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

// Ensure your generated API error enums implement this trait or are compatible
pub trait ApiError: DeserializeOwned + Send + Sync + std::fmt::Debug + 'static {}

pub async fn execute_request<T, B, E>(
    config: &Configuration,
    method: Method,
    path_template: &str,
    path_params_map: HashMap<String, String>,
    query_params_vec: Option<&[(&str, String)]>, // Vec of key-value pairs for query
    body_payload_option: Option<B>,
) -> Result<T, Error<E>>
where
    T: DeserializeOwned + Send + Sync + 'static,
    B: Serialize + Send + Sync,
    E: ApiError, // Use the ApiError trait
{
    let mut path = path_template.to_string();
    for (key, value) in path_params_map {
        path = path.replace(&format!("{{{}}}", key), &value);
    }
    let url_str = format!("{}{}", config.base_path, path);

    let mut req_builder = config.client.request(method.clone(), url_str);

    if let Some(ref user_agent) = Some(&config.user_agent) {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.to_string());
    }

    if let Some(params) = query_params_vec {
        req_builder = req_builder.query(params);
    }

    if let Some(payload) = body_payload_option {
        match serde_json::to_vec(&payload) {
            // Assuming JSON body
            Ok(body_vec) => {
                let body_bytes = Bytes::from(body_vec);
                if !body_bytes.is_empty() {
                    req_builder = req_builder.with_extension(OriginalBodyBytes(body_bytes.clone()));
                }
                req_builder =
                    req_builder.header(reqwest::header::CONTENT_TYPE, &config.content_type);
                req_builder = req_builder.body(body_bytes);
            }
            Err(e) => return Err(Error::Serde(e)), // Ensure your Error enum has a Serde variant
        }
    }
    // Note: Removed complex multipart/form logic, assuming JSON for now.
    // If multipart or forms are needed, this helper must be significantly expanded,
    // or those specific operations in the template should bypass this part of the helper.

    match req_builder.build() {
        Ok(req) => {
            match config.client.execute(req).await {
                Ok(resp) => {
                    let status = resp.status();
                    match resp.text().await {
                        Ok(content) => {
                            if !status.is_client_error() && !status.is_server_error() {
                                // Handle successful deserialization
                                serde_json::from_str(&content).map_err(Error::Serde)
                            } else {
                                // Handle API error deserialization
                                let entity: Option<E> = serde_json::from_str(&content).ok();
                                let err_content = ResponseContent {
                                    status,
                                    content,
                                    entity,
                                };
                                Err(Error::ResponseError(err_content))
                            }
                        }
                        Err(e) => Err(Error::Reqwest(e)), // Ensure Error::Reqwest variant exists
                    }
                }
                Err(e) => Err(Error::ReqwestMiddleware(e)),
            }
        }
        Err(e) => Err(Error::Reqwest(e)),
    }
}
