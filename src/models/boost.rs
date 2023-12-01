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
pub struct Boost {
    #[serde(rename = "message")]
    pub message: String,
}

impl Boost {
    #[must_use]
    pub fn new(message: String) -> Boost {
        Boost { message }
    }
}
