/*
 * ark/v1/service.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: version not set
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1TreeLevel {
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<models::V1Node>>,
}

impl V1TreeLevel {
    pub fn new() -> V1TreeLevel {
        V1TreeLevel { nodes: None }
    }
}
