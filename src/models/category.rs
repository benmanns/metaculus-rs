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
pub struct Category {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(rename = "long_name")]
    pub long_name: String,
}

impl Category {
    #[must_use]
    pub fn new(url: String, id: String, short_name: String, long_name: String) -> Category {
        Category {
            url,
            id,
            short_name,
            long_name,
        }
    }
}
