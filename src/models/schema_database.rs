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
pub struct SchemaDatabase {
    #[serde(rename = "databaseName", skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}

impl SchemaDatabase {
    pub fn new() -> SchemaDatabase {
        SchemaDatabase {
            database_name: None,
        }
    }
}


