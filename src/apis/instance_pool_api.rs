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

/// struct for passing parameters to the method [`create_instance_pool`]
#[derive(Clone, Debug)]
pub struct CreateInstancePoolParams {
    pub create_instance_pool_request: crate::models::CreateInstancePoolRequest,
}

/// struct for passing parameters to the method [`delete_instance_pool`]
#[derive(Clone, Debug)]
pub struct DeleteInstancePoolParams {
    pub id: String,
}

/// struct for passing parameters to the method [`evict_instance_pool_members`]
#[derive(Clone, Debug)]
pub struct EvictInstancePoolMembersParams {
    pub id: String,
    pub evict_instance_pool_members_request: crate::models::EvictInstancePoolMembersRequest,
}

/// struct for passing parameters to the method [`get_instance_pool`]
#[derive(Clone, Debug)]
pub struct GetInstancePoolParams {
    pub id: String,
}

/// struct for passing parameters to the method [`reset_instance_pool_field`]
#[derive(Clone, Debug)]
pub struct ResetInstancePoolFieldParams {
    pub id: String,
    pub field: String,
}

/// struct for passing parameters to the method [`scale_instance_pool`]
#[derive(Clone, Debug)]
pub struct ScaleInstancePoolParams {
    pub id: String,
    pub scale_instance_pool_request: crate::models::ScaleInstancePoolRequest,
}

/// struct for passing parameters to the method [`update_instance_pool`]
#[derive(Clone, Debug)]
pub struct UpdateInstancePoolParams {
    pub id: String,
    pub update_instance_pool_request: crate::models::UpdateInstancePoolRequest,
}

/// struct for typed errors of method [`create_instance_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInstancePoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_instance_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteInstancePoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`evict_instance_pool_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvictInstancePoolMembersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_instance_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstancePoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_instance_pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListInstancePoolsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reset_instance_pool_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetInstancePoolFieldError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`scale_instance_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleInstancePoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_instance_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateInstancePoolError {
    UnknownValue(serde_json::Value),
}

///
pub async fn create_instance_pool(
    configuration: &configuration::Configuration,
    params: CreateInstancePoolParams,
) -> Result<crate::models::Operation, Error<CreateInstancePoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_instance_pool_request = params.create_instance_pool_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/instance-pool", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&create_instance_pool_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateInstancePoolError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn delete_instance_pool(
    configuration: &configuration::Configuration,
    params: DeleteInstancePoolParams,
) -> Result<crate::models::Operation, Error<DeleteInstancePoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance-pool/{id}",
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
        let local_var_entity: Option<DeleteInstancePoolError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This operation evicts the specified Compute instances member from the Instance Pool, shrinking it to `&lt;current pool size&gt; - &lt;# evicted members&gt;`.
pub async fn evict_instance_pool_members(
    configuration: &configuration::Configuration,
    params: EvictInstancePoolMembersParams,
) -> Result<crate::models::Operation, Error<EvictInstancePoolMembersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let evict_instance_pool_members_request = params.evict_instance_pool_members_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance-pool/{id}:evict",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&evict_instance_pool_members_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EvictInstancePoolMembersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn get_instance_pool(
    configuration: &configuration::Configuration,
    params: GetInstancePoolParams,
) -> Result<crate::models::InstancePool, Error<GetInstancePoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance-pool/{id}",
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
        let local_var_entity: Option<GetInstancePoolError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn list_instance_pools(
    configuration: &configuration::Configuration,
) -> Result<crate::models::ListInstancePools200Response, Error<ListInstancePoolsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/instance-pool", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListInstancePoolsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn reset_instance_pool_field(
    configuration: &configuration::Configuration,
    params: ResetInstancePoolFieldParams,
) -> Result<crate::models::Operation, Error<ResetInstancePoolFieldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let field = params.field;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance-pool/{id}/{field}",
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
        let local_var_entity: Option<ResetInstancePoolFieldError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn scale_instance_pool(
    configuration: &configuration::Configuration,
    params: ScaleInstancePoolParams,
) -> Result<crate::models::Operation, Error<ScaleInstancePoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let scale_instance_pool_request = params.scale_instance_pool_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance-pool/{id}:scale",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&scale_instance_pool_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ScaleInstancePoolError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn update_instance_pool(
    configuration: &configuration::Configuration,
    params: UpdateInstancePoolParams,
) -> Result<crate::models::Operation, Error<UpdateInstancePoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let update_instance_pool_request = params.update_instance_pool_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance-pool/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&update_instance_pool_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateInstancePoolError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
