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

/// struct for typed errors of method [`add_external_source_to_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddExternalSourceToSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_rule_to_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRuleToSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`attach_instance_to_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachInstanceToSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_rule_from_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRuleFromSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`detach_instance_from_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetachInstanceFromSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_security_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityGroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_external_source_from_security_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveExternalSourceFromSecurityGroupError {
    UnknownValue(serde_json::Value),
}

pub async fn add_external_source_to_security_group(
    configuration: &configuration::Configuration,
    id: &str,
    add_external_source_to_security_group_request: models::AddExternalSourceToSecurityGroupRequest,
) -> Result<models::Operation, Error<AddExternalSourceToSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}:add-source",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&add_external_source_to_security_group_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddExternalSourceToSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn add_rule_to_security_group(
    configuration: &configuration::Configuration,
    id: &str,
    add_rule_to_security_group_request: models::AddRuleToSecurityGroupRequest,
) -> Result<models::Operation, Error<AddRuleToSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}/rules",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&add_rule_to_security_group_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddRuleToSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn attach_instance_to_security_group(
    configuration: &configuration::Configuration,
    id: &str,
    detach_instance_from_private_network_request: models::DetachInstanceFromPrivateNetworkRequest,
) -> Result<models::Operation, Error<AttachInstanceToSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}:attach",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&detach_instance_from_private_network_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AttachInstanceToSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_security_group(
    configuration: &configuration::Configuration,
    create_security_group_request: models::CreateSecurityGroupRequest,
) -> Result<models::Operation, Error<CreateSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/security-group", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_security_group_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_rule_from_security_group(
    configuration: &configuration::Configuration,
    id: &str,
    rule_id: &str,
) -> Result<models::Operation, Error<DeleteRuleFromSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}/rules/{rule_id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
        rule_id = crate::apis::urlencode(rule_id)
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
        let local_var_entity: Option<DeleteRuleFromSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_security_group(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Operation, Error<DeleteSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}",
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
        let local_var_entity: Option<DeleteSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn detach_instance_from_security_group(
    configuration: &configuration::Configuration,
    id: &str,
    detach_instance_from_private_network_request: models::DetachInstanceFromPrivateNetworkRequest,
) -> Result<models::Operation, Error<DetachInstanceFromSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}:detach",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&detach_instance_from_private_network_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DetachInstanceFromSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_security_group(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::SecurityGroup, Error<GetSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}",
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
        let local_var_entity: Option<GetSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists security groups. When visibility is set to public, lists public security groups. Public security groups are objects maintained by Exoscale which contain source addresses for relevant services hosted by Exoscale. They can be used a source in ingress rules and as a destination in egress rules.
pub async fn list_security_groups(
    configuration: &configuration::Configuration,
    visibility: Option<&str>,
) -> Result<models::ListSecurityGroups200Response, Error<ListSecurityGroupsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/security-group", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = visibility {
        local_var_req_builder =
            local_var_req_builder.query(&[("visibility", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListSecurityGroupsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn remove_external_source_from_security_group(
    configuration: &configuration::Configuration,
    id: &str,
    remove_external_source_from_security_group_request: models::RemoveExternalSourceFromSecurityGroupRequest,
) -> Result<models::Operation, Error<RemoveExternalSourceFromSecurityGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/security-group/{id}:remove-source",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&remove_external_source_from_security_group_request);

    let mut local_var_req = local_var_req_builder.build()?;
    let _ = sign_request(&mut local_var_req, configuration);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RemoveExternalSourceFromSecurityGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
