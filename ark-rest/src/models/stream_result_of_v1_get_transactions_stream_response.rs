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
pub struct StreamResultOfV1GetTransactionsStreamResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::V1GetTransactionsStreamResponse>>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::RpcStatus>>,
}

impl StreamResultOfV1GetTransactionsStreamResponse {
    pub fn new() -> StreamResultOfV1GetTransactionsStreamResponse {
        StreamResultOfV1GetTransactionsStreamResponse {
            result: None,
            error: None,
        }
    }
}
