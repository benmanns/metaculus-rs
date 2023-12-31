/*
 * Metaculus API
 *
 * Welcome to the unofficial Rust client for the Metaculus API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

/// `QuestionRelated` : Just contains basic data used by all other serializers. Does not include all of the derived data.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionRelated {
    #[serde(rename = "active_state", deserialize_with = "Option::deserialize")]
    pub active_state: Option<serde_json::Value>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "page_url")]
    pub page_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "author")]
    pub author: i32,
    #[serde(rename = "author_name", deserialize_with = "Option::deserialize")]
    pub author_name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_short", skip_serializing_if = "Option::is_none")]
    pub title_short: Option<String>,
    #[serde(rename = "group_label", skip_serializing_if = "Option::is_none")]
    pub group_label: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::Status3baEnum>,
    #[serde(rename = "resolution", deserialize_with = "Option::deserialize")]
    pub resolution: Option<f64>,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "publish_time", skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    #[serde(rename = "close_time", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[serde(rename = "effected_close_time")]
    pub effected_close_time: String,
    #[serde(rename = "resolve_time", skip_serializing_if = "Option::is_none")]
    pub resolve_time: Option<String>,
    #[serde(rename = "possibilities", skip_serializing_if = "Option::is_none")]
    pub possibilities: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "scoring", skip_serializing_if = "Option::is_none")]
    pub scoring: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::QuestionTypes>,
    #[serde(rename = "user_perms", deserialize_with = "Option::deserialize")]
    pub user_perms: Option<serde_json::Value>,
    #[serde(
        rename = "weekly_movement",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weekly_movement: Option<Option<f64>>,
    #[serde(
        rename = "weekly_movement_direction",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weekly_movement_direction: Option<Option<i32>>,
    #[serde(rename = "cp_reveal_time", skip_serializing_if = "Option::is_none")]
    pub cp_reveal_time: Option<String>,
    #[serde(rename = "edited_time")]
    pub edited_time: String,
    #[serde(
        rename = "community_prediction",
        deserialize_with = "Option::deserialize"
    )]
    pub community_prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl QuestionRelated {
    /// Just contains basic data used by all other serializers. Does not include all of the derived data.
    #[must_use]
    pub fn new(
        active_state: Option<serde_json::Value>,
        url: String,
        page_url: String,
        id: i32,
        author: i32,
        author_name: Option<String>,
        resolution: Option<f64>,
        created_time: String,
        effected_close_time: String,
        user_perms: Option<serde_json::Value>,
        edited_time: String,
        community_prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
    ) -> QuestionRelated {
        QuestionRelated {
            active_state,
            url,
            page_url,
            id,
            author,
            author_name,
            title: None,
            title_short: None,
            group_label: None,
            status: None,
            resolution,
            created_time,
            publish_time: None,
            close_time: None,
            effected_close_time,
            resolve_time: None,
            possibilities: None,
            scoring: None,
            r#type: None,
            user_perms,
            weekly_movement: None,
            weekly_movement_direction: None,
            cp_reveal_time: None,
            edited_time,
            community_prediction,
        }
    }
}
