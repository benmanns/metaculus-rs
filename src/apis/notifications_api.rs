/*
 * Metaculus API
 *
 * Welcome to the unofficial Rust client for the Metaculus API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`notifications_list`]
#[derive(Clone, Debug)]
pub struct NotificationsListParams {
    /// A page number within the paginated result set.
    pub page: Option<i32>,
}

/// struct for passing parameters to the method [`notifications_mark_read_create`]
#[derive(Clone, Debug)]
pub struct NotificationsMarkReadCreateParams {
    pub notification: crate::models::Notification,
}

/// struct for typed errors of method [`notifications_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotificationsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`notifications_mark_read_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotificationsMarkReadCreateError {
    UnknownValue(serde_json::Value),
}

pub async fn notifications_list(
    configuration: &configuration::Configuration,
    params: NotificationsListParams,
) -> Result<crate::models::PaginatedNotificationList, Error<NotificationsListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api2/notifications/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_cookie) = local_var_configuration.cookie {
        local_var_req_builder =
            local_var_req_builder.header("Cookie", format!("sessionid={}", local_var_cookie.value));
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NotificationsListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn notifications_mark_read_create(
    configuration: &configuration::Configuration,
    params: NotificationsMarkReadCreateParams,
) -> Result<crate::models::Notification, Error<NotificationsMarkReadCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let notification = params.notification;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/notifications/mark_read/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_cookie) = local_var_configuration.cookie {
        local_var_req_builder =
            local_var_req_builder.header("Cookie", format!("sessionid={}", local_var_cookie.value));
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&notification);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NotificationsMarkReadCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
