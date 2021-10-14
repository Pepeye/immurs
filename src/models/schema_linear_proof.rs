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
pub struct SchemaLinearProof {
    #[serde(rename = "sourceTxId", skip_serializing_if = "Option::is_none")]
    pub source_tx_id: Option<String>,
    #[serde(rename = "TargetTxId", skip_serializing_if = "Option::is_none")]
    pub target_tx_id: Option<String>,
    #[serde(rename = "terms", skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<String>>,
}

impl SchemaLinearProof {
    pub fn new() -> SchemaLinearProof {
        SchemaLinearProof {
            source_tx_id: None,
            target_tx_id: None,
            terms: None,
        }
    }
}


