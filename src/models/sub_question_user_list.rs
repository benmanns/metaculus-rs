/*
 * Metaculus API
 *
 * Welcome to the unofficial Rust client for the Metaculus API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubQuestionUserList {
    #[serde(rename = "active_state", deserialize_with = "Option::deserialize")]
    pub active_state: Option<serde_json::Value>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "resolution", deserialize_with = "Option::deserialize")]
    pub resolution: Option<f64>,
    #[serde(rename = "publish_time", skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    #[serde(rename = "close_time", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[serde(
        rename = "effected_close_time",
        deserialize_with = "Option::deserialize"
    )]
    pub effected_close_time: Option<String>,
    #[serde(rename = "resolve_time", skip_serializing_if = "Option::is_none")]
    pub resolve_time: Option<String>,
    #[serde(rename = "possibilities", skip_serializing_if = "Option::is_none")]
    pub possibilities: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "sub_question_label", skip_serializing_if = "Option::is_none")]
    pub sub_question_label: Option<String>,
    #[serde(
        rename = "metaculus_prediction",
        deserialize_with = "Option::deserialize"
    )]
    pub metaculus_prediction: Option<serde_json::Value>,
    #[serde(
        rename = "community_prediction",
        deserialize_with = "Option::deserialize"
    )]
    pub community_prediction: Option<serde_json::Value>,
    #[serde(rename = "conditioned_on_resolution")]
    pub conditioned_on_resolution: f64,
    #[serde(rename = "cp_reveal_time", skip_serializing_if = "Option::is_none")]
    pub cp_reveal_time: Option<String>,
    #[serde(rename = "my_predictions")]
    pub my_predictions: Box<crate::models::Prediction>,
    #[serde(rename = "user_community_vis")]
    pub user_community_vis: crate::models::UserCommunityVisEnum,
}

impl SubQuestionUserList {
    #[must_use]
    pub fn new(
        active_state: Option<serde_json::Value>,
        id: i32,
        resolution: Option<f64>,
        effected_close_time: Option<String>,
        metaculus_prediction: Option<serde_json::Value>,
        community_prediction: Option<serde_json::Value>,
        conditioned_on_resolution: f64,
        my_predictions: crate::models::Prediction,
        user_community_vis: crate::models::UserCommunityVisEnum,
    ) -> SubQuestionUserList {
        SubQuestionUserList {
            active_state,
            id,
            resolution,
            publish_time: None,
            close_time: None,
            effected_close_time,
            resolve_time: None,
            possibilities: None,
            sub_question_label: None,
            metaculus_prediction,
            community_prediction,
            conditioned_on_resolution,
            cp_reveal_time: None,
            my_predictions: Box::new(my_predictions),
            user_community_vis,
        }
    }
}
