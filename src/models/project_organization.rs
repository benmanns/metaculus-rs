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
pub struct ProjectOrganization {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "selected")]
    pub selected: bool,
}

impl ProjectOrganization {
    #[must_use]
    pub fn new(id: i32, selected: bool) -> ProjectOrganization {
        ProjectOrganization { id, selected }
    }
}
