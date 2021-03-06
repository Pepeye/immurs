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
pub struct SchemaVerifiableReferenceRequest {
    #[serde(rename = "referenceRequest", skip_serializing_if = "Option::is_none")]
    pub reference_request: Option<Box<crate::models::SchemaReferenceRequest>>,
    #[serde(rename = "proveSinceTx", skip_serializing_if = "Option::is_none")]
    pub prove_since_tx: Option<String>,
}

impl SchemaVerifiableReferenceRequest {
    pub fn new() -> SchemaVerifiableReferenceRequest {
        SchemaVerifiableReferenceRequest {
            reference_request: None,
            prove_since_tx: None,
        }
    }
}


