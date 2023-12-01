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
pub struct PredictionUsername {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "predictions", skip_serializing_if = "Option::is_none")]
    pub predictions: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "points_won",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub points_won: Option<Option<f64>>,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<i32>,
    #[serde(rename = "question")]
    pub question: i32,
    #[serde(rename = "username")]
    pub username: String,
}

impl PredictionUsername {
    #[must_use]
    pub fn new(id: i32, user: Option<i32>, question: i32, username: String) -> PredictionUsername {
        PredictionUsername {
            id,
            predictions: None,
            points_won: None,
            user,
            question,
            username,
        }
    }
}
