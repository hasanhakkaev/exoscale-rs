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

/// struct for typed errors of method [`add_service_to_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddServiceToLoadBalancerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLoadBalancerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLoadBalancerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_load_balancer_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLoadBalancerServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoadBalancerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_load_balancer_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoadBalancerServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_load_balancers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLoadBalancersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reset_load_balancer_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetLoadBalancerFieldError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reset_load_balancer_service_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetLoadBalancerServiceFieldError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLoadBalancerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_load_balancer_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLoadBalancerServiceError {
    UnknownValue(serde_json::Value),
}

pub async fn add_service_to_load_balancer(
    configuration: &configuration::Configuration,
    id: &str,
    add_service_to_load_balancer_request: models::AddServiceToLoadBalancerRequest,
) -> Result<models::Operation, Error<AddServiceToLoadBalancerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}/service",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&add_service_to_load_balancer_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddServiceToLoadBalancerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_load_balancer(
    configuration: &configuration::Configuration,
    create_load_balancer_request: models::CreateLoadBalancerRequest,
) -> Result<models::Operation, Error<CreateLoadBalancerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/load-balancer", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_load_balancer_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateLoadBalancerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_load_balancer(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Operation, Error<DeleteLoadBalancerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
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
        let local_var_entity: Option<DeleteLoadBalancerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_load_balancer_service(
    configuration: &configuration::Configuration,
    id: &str,
    service_id: &str,
) -> Result<models::Operation, Error<DeleteLoadBalancerServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}/service/{service_id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
        service_id = crate::apis::urlencode(service_id)
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
        let local_var_entity: Option<DeleteLoadBalancerServiceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_load_balancer(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::LoadBalancer, Error<GetLoadBalancerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
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
        let local_var_entity: Option<GetLoadBalancerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_load_balancer_service(
    configuration: &configuration::Configuration,
    id: &str,
    service_id: &str,
) -> Result<models::LoadBalancerService, Error<GetLoadBalancerServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}/service/{service_id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
        service_id = crate::apis::urlencode(service_id)
    );
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
        let local_var_entity: Option<GetLoadBalancerServiceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_load_balancers(
    configuration: &configuration::Configuration,
) -> Result<models::ListLoadBalancers200Response, Error<ListLoadBalancersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/load-balancer", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListLoadBalancersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn reset_load_balancer_field(
    configuration: &configuration::Configuration,
    id: &str,
    field: &str,
) -> Result<models::Operation, Error<ResetLoadBalancerFieldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}/{field}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
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
        let local_var_entity: Option<ResetLoadBalancerFieldError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn reset_load_balancer_service_field(
    configuration: &configuration::Configuration,
    id: &str,
    service_id: &str,
    field: &str,
) -> Result<models::Operation, Error<ResetLoadBalancerServiceFieldError>> {
    let local_var_configuration = configuration;

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

pub async fn update_load_balancer(
    configuration: &configuration::Configuration,
    id: &str,
    update_load_balancer_request: models::UpdateLoadBalancerRequest,
) -> Result<models::Operation, Error<UpdateLoadBalancerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_load_balancer_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateLoadBalancerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_load_balancer_service(
    configuration: &configuration::Configuration,
    id: &str,
    service_id: &str,
    update_load_balancer_service_request: models::UpdateLoadBalancerServiceRequest,
) -> Result<models::Operation, Error<UpdateLoadBalancerServiceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/load-balancer/{id}/service/{service_id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
        service_id = crate::apis::urlencode(service_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_load_balancer_service_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateLoadBalancerServiceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
