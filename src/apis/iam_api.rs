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

/// struct for typed errors of method [`create_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApiKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_iam_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIamRoleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApiKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_iam_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIamRoleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApiKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_iam_organization_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIamOrganizationPolicyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_iam_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIamRoleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_api_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApiKeysError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_iam_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIamRolesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_iam_organization_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIamOrganizationPolicyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_iam_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIamRoleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_iam_role_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIamRolePolicyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserRoleError {
    UnknownValue(serde_json::Value),
}

pub async fn create_api_key(
    configuration: &configuration::Configuration,
    create_api_key_request: models::CreateApiKeyRequest,
) -> Result<models::IamApiKeyCreated, Error<CreateApiKeyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access-key", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_api_key_request);

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
        let local_var_entity: Option<CreateApiKeyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_iam_role(
    configuration: &configuration::Configuration,
    create_iam_role_request: models::CreateIamRoleRequest,
) -> Result<models::Operation, Error<CreateIamRoleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iam-role", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_iam_role_request);

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
        let local_var_entity: Option<CreateIamRoleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_user(
    configuration: &configuration::Configuration,
    create_user_request: models::CreateUserRequest,
) -> Result<models::Operation, Error<CreateUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_user_request);

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
        let local_var_entity: Option<CreateUserError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_api_key(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Operation, Error<DeleteApiKeyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api-key/{id}",
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
        let local_var_entity: Option<DeleteApiKeyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_iam_role(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Operation, Error<DeleteIamRoleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/iam-role/{id}",
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
        let local_var_entity: Option<DeleteIamRoleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_user(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::Operation, Error<DeleteUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/user/{id}",
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
        let local_var_entity: Option<DeleteUserError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_api_key(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::IamApiKey, Error<GetApiKeyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api-key/{id}",
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
        let local_var_entity: Option<GetApiKeyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_iam_organization_policy(
    configuration: &configuration::Configuration,
) -> Result<models::IamPolicy, Error<GetIamOrganizationPolicyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/iam-organization-policy",
        local_var_configuration.base_path
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
        let local_var_entity: Option<GetIamOrganizationPolicyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_iam_role(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::IamRole, Error<GetIamRoleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/iam-role/{id}",
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
        let local_var_entity: Option<GetIamRoleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_api_keys(
    configuration: &configuration::Configuration,
) -> Result<models::ListApiKeys200Response, Error<ListApiKeysError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api-key", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListApiKeysError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_iam_roles(
    configuration: &configuration::Configuration,
) -> Result<models::ListIamRoles200Response, Error<ListIamRolesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iam-role", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListIamRolesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_users(
    configuration: &configuration::Configuration,
) -> Result<models::ListUsers200Response, Error<ListUsersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListUsersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_iam_organization_policy(
    configuration: &configuration::Configuration,
    iam_policy: models::IamPolicy,
) -> Result<models::Operation, Error<UpdateIamOrganizationPolicyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/iam-organization-policy",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&iam_policy);

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
        let local_var_entity: Option<UpdateIamOrganizationPolicyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_iam_role(
    configuration: &configuration::Configuration,
    id: &str,
    update_iam_role_request: models::UpdateIamRoleRequest,
) -> Result<models::Operation, Error<UpdateIamRoleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/iam-role/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_iam_role_request);

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
        let local_var_entity: Option<UpdateIamRoleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_iam_role_policy(
    configuration: &configuration::Configuration,
    id: &str,
    iam_policy: models::IamPolicy,
) -> Result<models::Operation, Error<UpdateIamRolePolicyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/iam-role/{id}:policy",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&iam_policy);

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
        let local_var_entity: Option<UpdateIamRolePolicyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_user_role(
    configuration: &configuration::Configuration,
    id: &str,
    update_user_role_request: models::UpdateUserRoleRequest,
) -> Result<models::Operation, Error<UpdateUserRoleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/user/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&update_user_role_request);

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
        let local_var_entity: Option<UpdateUserRoleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
