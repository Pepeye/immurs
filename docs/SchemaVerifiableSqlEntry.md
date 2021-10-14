# SchemaVerifiableSqlEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sql_entry** | Option<[**crate::models::SchemaSqlEntry**](schemaSQLEntry.md)> |  | [optional]
**verifiable_tx** | Option<[**crate::models::SchemaVerifiableTx**](schemaVerifiableTx.md)> |  | [optional]
**inclusion_proof** | Option<[**crate::models::SchemaInclusionProof**](schemaInclusionProof.md)> |  | [optional]
**database_id** | Option<**i64**> |  | [optional]
**table_id** | Option<**i64**> |  | [optional]
**pkids** | Option<**Vec<i64>**> |  | [optional]
**col_names_by_id** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**col_ids_by_name** | Option<**::std::collections::HashMap<String, i64>**> |  | [optional]
**col_types_by_id** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**col_len_by_id** | Option<**::std::collections::HashMap<String, i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


