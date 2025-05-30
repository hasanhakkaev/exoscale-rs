/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models, sign_request};
use reqwest;

/// struct for typed errors of method [`get_sos_presigned_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSosPresignedUrlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_sos_buckets_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSosBucketsUsageError {
    UnknownValue(serde_json::Value),
}

/// Generates Presigned Download URL for SOS object
pub async fn get_sos_presigned_url(
    configuration: &configuration::Configuration,
    bucket: &str,
    key: Option<&str>,
) -> Result<models::GetSosPresignedUrl200Response, Error<GetSosPresignedUrlError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/sos/{bucket}/presigned-url",
        local_var_configuration.base_path,
        bucket = crate::apis::urlencode(bucket)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_content_type) = local_var_configuration.content_type {
        local_var_req_builder = local_var_req_builder.header(
            reqwest::header::CONTENT_TYPE,
            local_var_content_type.clone(),
        );
    }

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSosPresignedUrlError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_sos_buckets_usage(
    configuration: &configuration::Configuration,
) -> Result<models::ListSosBucketsUsage200Response, Error<ListSosBucketsUsageError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sos-buckets-usage", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref local_var_content_type) = local_var_configuration.content_type {
        local_var_req_builder = local_var_req_builder.header(
            reqwest::header::CONTENT_TYPE,
            local_var_content_type.clone(),
        );
    }

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListSosBucketsUsageError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
