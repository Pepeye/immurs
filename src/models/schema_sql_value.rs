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
pub struct SchemaSqlValue {
    #[serde(rename = "null", skip_serializing_if = "Option::is_none")]
    pub null: Option<String>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename = "b", skip_serializing_if = "Option::is_none")]
    pub b: Option<bool>,
    #[serde(rename = "bs", skip_serializing_if = "Option::is_none")]
    pub bs: Option<String>,
}

impl SchemaSqlValue {
    pub fn new() -> SchemaSqlValue {
        SchemaSqlValue {
            null: None,
            n: None,
            s: None,
            b: None,
            bs: None,
        }
    }
}


