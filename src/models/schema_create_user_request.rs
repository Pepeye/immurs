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
pub struct SchemaCreateUserRequest {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<i64>,
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}

impl SchemaCreateUserRequest {
    pub fn new() -> SchemaCreateUserRequest {
        SchemaCreateUserRequest {
            user: None,
            password: None,
            permission: None,
            database: None,
        }
    }
}


