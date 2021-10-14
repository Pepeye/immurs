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
pub struct SchemaImmutableState {
    #[serde(rename = "db", skip_serializing_if = "Option::is_none")]
    pub db: Option<String>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "txHash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Box<crate::models::SchemaSignature>>,
}

impl SchemaImmutableState {
    pub fn new() -> SchemaImmutableState {
        SchemaImmutableState {
            db: None,
            tx_id: None,
            tx_hash: None,
            signature: None,
        }
    }
}


