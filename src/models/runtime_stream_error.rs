/*
 * immudb REST API
 *
 * <b>IMPORTANT</b>: All <code>get</code> and <code>safeget</code> functions return <u>base64-encoded</u> keys and values, while all <code>set</code> and <code>safeset</code> functions expect <u>base64-encoded</u> inputs.
 *
 * The version of the OpenAPI document: version not set
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuntimeStreamError {
    #[serde(rename = "grpc_code", skip_serializing_if = "Option::is_none")]
    pub grpc_code: Option<i32>,
    #[serde(rename = "http_code", skip_serializing_if = "Option::is_none")]
    pub http_code: Option<i32>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "http_status", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::ProtobufAny>>,
}

impl RuntimeStreamError {
    pub fn new() -> RuntimeStreamError {
        RuntimeStreamError {
            grpc_code: None,
            http_code: None,
            message: None,
            http_status: None,
            details: None,
        }
    }
}

