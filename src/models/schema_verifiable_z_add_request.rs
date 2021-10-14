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
pub struct SchemaVerifiableZAddRequest {
    #[serde(rename = "zAddRequest", skip_serializing_if = "Option::is_none")]
    pub z_add_request: Option<Box<crate::models::SchemaZAddRequest>>,
    #[serde(rename = "proveSinceTx", skip_serializing_if = "Option::is_none")]
    pub prove_since_tx: Option<String>,
}

impl SchemaVerifiableZAddRequest {
    pub fn new() -> SchemaVerifiableZAddRequest {
        SchemaVerifiableZAddRequest {
            z_add_request: None,
            prove_since_tx: None,
        }
    }
}

