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
pub struct PredictionInput {
    #[serde(rename = "prediction", deserialize_with = "Option::deserialize")]
    pub prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PredictionInput {
    #[must_use]
    pub fn new(
        prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
    ) -> PredictionInput {
        PredictionInput { prediction }
    }
}
