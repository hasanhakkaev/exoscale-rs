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
use serde::{Deserialize, Serialize};
use crate::{models, utils,apis::ResponseContent};
use super::{Error, configuration};


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


/// struct for typed errors of method [`reset_iam_organization_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetIamOrganizationPolicyError {
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


pub async fn create_api_key(configuration: &configuration::Configuration, create_api_key_request: models::CreateApiKeyRequest) -> Result<models::IamApiKeyCreated, Error<CreateApiKeyError>> {
    let local_var_create_api_key_request = create_api_key_request;

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_create_api_key_request);

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/api-key",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn create_iam_role(configuration: &configuration::Configuration, create_iam_role_request: models::CreateIamRoleRequest) -> Result<models::Operation, Error<CreateIamRoleError>> {
    let local_var_create_iam_role_request = create_iam_role_request;

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_create_iam_role_request);

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/iam-role",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn create_user(configuration: &configuration::Configuration, create_user_request: models::CreateUserRequest) -> Result<models::Operation, Error<CreateUserError>> {
    let local_var_create_user_request = create_user_request;

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_create_user_request);

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/user",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn delete_api_key(configuration: &configuration::Configuration, id: &str) -> Result<models::Operation, Error<DeleteApiKeyError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/api-key/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn delete_iam_role(configuration: &configuration::Configuration, id: String) -> Result<models::Operation, Error<DeleteIamRoleError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/iam-role/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn delete_user(configuration: &configuration::Configuration, id: String) -> Result<models::Operation, Error<DeleteUserError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/user/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn get_api_key(configuration: &configuration::Configuration, id: &str) -> Result<models::IamApiKey, Error<GetApiKeyError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/api-key/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn get_iam_organization_policy(configuration: &configuration::Configuration, ) -> Result<models::IamPolicy, Error<GetIamOrganizationPolicyError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/iam-organization-policy",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn get_iam_role(configuration: &configuration::Configuration, id: String) -> Result<models::IamRole, Error<GetIamRoleError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/iam-role/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn list_api_keys(configuration: &configuration::Configuration, ) -> Result<models::ListApiKeys200Response, Error<ListApiKeysError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/api-key",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn list_iam_roles(configuration: &configuration::Configuration, ) -> Result<models::ListIamRoles200Response, Error<ListIamRolesError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/iam-role",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn list_users(configuration: &configuration::Configuration, ) -> Result<models::ListUsers200Response, Error<ListUsersError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/user",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn reset_iam_organization_policy(configuration: &configuration::Configuration, ) -> Result<models::Operation, Error<ResetIamOrganizationPolicyError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/iam-organization-policy:reset",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_iam_organization_policy(configuration: &configuration::Configuration, iam_policy: models::IamPolicy) -> Result<models::Operation, Error<UpdateIamOrganizationPolicyError>> {
    let local_var_iam_policy = iam_policy;

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_iam_policy);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/iam-organization-policy",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_iam_role(configuration: &configuration::Configuration, id: String, update_iam_role_request: models::UpdateIamRoleRequest) -> Result<models::Operation, Error<UpdateIamRoleError>> {
    let local_var_id = id;
    let local_var_update_iam_role_request = update_iam_role_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_update_iam_role_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/iam-role/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_iam_role_policy(configuration: &configuration::Configuration, id: String, iam_policy: models::IamPolicy) -> Result<models::Operation, Error<UpdateIamRolePolicyError>> {
    let local_var_id = id;
    let local_var_iam_policy = iam_policy;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_iam_policy);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/iam-role/{id}:policy",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_user_role(configuration: &configuration::Configuration, id: String, update_user_role_request: models::UpdateUserRoleRequest) -> Result<models::Operation, Error<UpdateUserRoleError>> {
    let local_var_id = id;
    let local_var_update_user_role_request = update_user_role_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_update_user_role_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/user/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
