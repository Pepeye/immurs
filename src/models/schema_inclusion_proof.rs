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
pub struct SchemaInclusionProof {
    #[serde(rename = "leaf", skip_serializing_if = "Option::is_none")]
    pub leaf: Option<i32>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "terms", skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<String>>,
}

impl SchemaInclusionProof {
    pub fn new() -> SchemaInclusionProof {
        SchemaInclusionProof {
            leaf: None,
            width: None,
            terms: None,
        }
    }
}

