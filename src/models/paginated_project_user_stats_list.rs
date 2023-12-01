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
pub struct PaginatedProjectUserStatsList {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(
        rename = "next",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next: Option<Option<String>>,
    #[serde(
        rename = "previous",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous: Option<Option<String>>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::ProjectUserStats>>,
}

impl Default for PaginatedProjectUserStatsList {
    fn default() -> Self {
        Self::new()
    }
}

impl PaginatedProjectUserStatsList {
    #[must_use]
    pub fn new() -> PaginatedProjectUserStatsList {
        PaginatedProjectUserStatsList {
            count: None,
            next: None,
            previous: None,
            results: None,
        }
    }
}
