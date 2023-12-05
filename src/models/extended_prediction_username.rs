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
pub struct ExtendedPredictionUsername {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "predictions", skip_serializing_if = "Option::is_none")]
    pub predictions: Option<Vec<::std::collections::HashMap<String, serde_json::Value>>>,
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
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "username")]
    pub username: String,
    /// Used in tournament scoring. Defined as log2(player prediction / community prediction), averaged over the lifetime of the question. Zero for null / void predictions.
    #[serde(
        rename = "log_score",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_score: Option<Option<f64>>,
    /// Defined as per our scoring FAQ. Zero for null / void predictions.
    #[serde(
        rename = "absolute_log_score",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub absolute_log_score: Option<Option<f64>>,
    /// Percentage of question open time that this prediction has been active.
    #[serde(
        rename = "coverage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub coverage: Option<Option<f64>>,
}

impl ExtendedPredictionUsername {
    #[must_use]
    pub fn new(
        id: i32,
        user: Option<i32>,
        question: i32,
        username: String,
    ) -> ExtendedPredictionUsername {
        ExtendedPredictionUsername {
            id,
            predictions: None,
            points_won: None,
            user,
            question,
            active: None,
            username,
            log_score: None,
            absolute_log_score: None,
            coverage: None,
        }
    }
}
