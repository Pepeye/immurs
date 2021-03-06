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
pub struct SchemaExecAllRequest {
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<crate::models::SchemaOp>>,
    #[serde(rename = "noWait", skip_serializing_if = "Option::is_none")]
    pub no_wait: Option<bool>,
}

impl SchemaExecAllRequest {
    pub fn new() -> SchemaExecAllRequest {
        SchemaExecAllRequest {
            operations: None,
            no_wait: None,
        }
    }
}


