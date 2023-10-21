/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;
use crate::sign_request;

/// struct for passing parameters to the method [`reset_load_balancer_service_field`]
#[derive(Clone, Debug)]
pub struct ResetLoadBalancerServiceFieldParams {
    pub id: String,
    pub service_id: String,
    pub field: String,
}

/// struct for typed errors of method [`reset_load_balancer_service_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetLoadBalancerServiceFieldError {
    UnknownValue(serde_json::Value),
}

///
pub async fn reset_load_balancer_service_field(
    configuration: &configuration::Configuration,
    params: ResetLoadBalancerServiceFieldParams,
) -> Result<crate::models::Operation, Error<ResetLoadBalancerServiceFieldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let service_id = params.service_id;
    let field = params.field;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}/service/{service_id}/{field}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
        service_id = crate::apis::urlencode(service_id),
        field = crate::apis::urlencode(field)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ResetLoadBalancerServiceFieldError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
