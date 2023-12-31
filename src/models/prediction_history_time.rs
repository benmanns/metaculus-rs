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
pub struct PredictionHistoryTime {
    #[serde(rename = "t")]
    pub t: f64,
    #[serde(rename = "y")]
    pub y: Vec<f64>,
    #[serde(rename = "q1")]
    pub q1: f64,
    #[serde(rename = "q2")]
    pub q2: f64,
    #[serde(rename = "q3")]
    pub q3: f64,
    #[serde(rename = "low")]
    pub low: f64,
    #[serde(rename = "high")]
    pub high: f64,
}

impl PredictionHistoryTime {
    #[must_use]
    pub fn new(
        t: f64,
        y: Vec<f64>,
        q1: f64,
        q2: f64,
        q3: f64,
        low: f64,
        high: f64,
    ) -> PredictionHistoryTime {
        PredictionHistoryTime {
            t,
            y,
            q1,
            q2,
            q3,
            low,
            high,
        }
    }
}
