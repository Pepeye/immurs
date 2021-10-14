# \ImmuDBApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**immu_db_change_password**](ImmuDBApi.md#immu_db_change_password) | **POST** /user/password/change | 
[**immu_db_change_permission**](ImmuDBApi.md#immu_db_change_permission) | **POST** /user/changepermission | 
[**immu_db_compact_index**](ImmuDBApi.md#immu_db_compact_index) | **GET** /db/compactindex | 
[**immu_db_count**](ImmuDBApi.md#immu_db_count) | **GET** /db/count/{prefix} | NOT YET SUPPORTED
[**immu_db_count_all**](ImmuDBApi.md#immu_db_count_all) | **GET** /db/countall | NOT YET SUPPORTED
[**immu_db_create_database**](ImmuDBApi.md#immu_db_create_database) | **POST** /db/create | DEPRECATED: kept for backward compatibility
[**immu_db_create_database_with**](ImmuDBApi.md#immu_db_create_database_with) | **POST** /db/createwith | 
[**immu_db_create_user**](ImmuDBApi.md#immu_db_create_user) | **POST** /user | 
[**immu_db_current_state**](ImmuDBApi.md#immu_db_current_state) | **GET** /db/state | 
[**immu_db_database_list**](ImmuDBApi.md#immu_db_database_list) | **POST** /db/list | 
[**immu_db_describe_table**](ImmuDBApi.md#immu_db_describe_table) | **POST** /db/tables | 
[**immu_db_exec_all**](ImmuDBApi.md#immu_db_exec_all) | **POST** /db/execall | 
[**immu_db_get**](ImmuDBApi.md#immu_db_get) | **GET** /db/get/{key} | 
[**immu_db_get_all**](ImmuDBApi.md#immu_db_get_all) | **POST** /db/getall | 
[**immu_db_health**](ImmuDBApi.md#immu_db_health) | **GET** /health | 
[**immu_db_history**](ImmuDBApi.md#immu_db_history) | **POST** /db/history | 
[**immu_db_list_tables**](ImmuDBApi.md#immu_db_list_tables) | **GET** /db/table/list | 
[**immu_db_list_users**](ImmuDBApi.md#immu_db_list_users) | **GET** /user/list | 
[**immu_db_login**](ImmuDBApi.md#immu_db_login) | **POST** /login | 
[**immu_db_logout**](ImmuDBApi.md#immu_db_logout) | **POST** /logout | 
[**immu_db_scan**](ImmuDBApi.md#immu_db_scan) | **POST** /db/scan | 
[**immu_db_set**](ImmuDBApi.md#immu_db_set) | **POST** /db/set | 
[**immu_db_set_active_user**](ImmuDBApi.md#immu_db_set_active_user) | **POST** /user/setactiveUser | 
[**immu_db_set_reference**](ImmuDBApi.md#immu_db_set_reference) | **POST** /db/setreference | 
[**immu_db_update_database**](ImmuDBApi.md#immu_db_update_database) | **POST** /db/update | 
[**immu_db_use_database**](ImmuDBApi.md#immu_db_use_database) | **GET** /db/use/{databaseName} | 
[**immu_db_use_snapshot**](ImmuDBApi.md#immu_db_use_snapshot) | **GET** /db/usesnapshot | SQL
[**immu_db_verifiable_get**](ImmuDBApi.md#immu_db_verifiable_get) | **POST** /db/verifiable/get | 
[**immu_db_verifiable_set**](ImmuDBApi.md#immu_db_verifiable_set) | **POST** /db/verifiable/set | 
[**immu_db_verifiable_set_reference**](ImmuDBApi.md#immu_db_verifiable_set_reference) | **POST** /db/verifiable/setreference | 
[**immu_db_verifiable_sql_get**](ImmuDBApi.md#immu_db_verifiable_sql_get) | **POST** /db/verifiable/sqlget | 
[**immu_db_verifiable_tx_by_id**](ImmuDBApi.md#immu_db_verifiable_tx_by_id) | **GET** /db/verifiable/tx/{tx} | 
[**immu_db_verifiable_z_add**](ImmuDBApi.md#immu_db_verifiable_z_add) | **POST** /db/verifiable/zadd | 
[**immu_dbsql_exec**](ImmuDBApi.md#immu_dbsql_exec) | **POST** /db/sqlexec | 
[**immu_dbsql_query**](ImmuDBApi.md#immu_dbsql_query) | **POST** /db/sqlquery | 
[**immu_dbtx_by_id**](ImmuDBApi.md#immu_dbtx_by_id) | **GET** /db/tx/{tx} | 
[**immu_dbtx_scan**](ImmuDBApi.md#immu_dbtx_scan) | **POST** /db/tx | 
[**immu_dbz_add**](ImmuDBApi.md#immu_dbz_add) | **POST** /db/zadd | 
[**immu_dbz_scan**](ImmuDBApi.md#immu_dbz_scan) | **POST** /db/zscan | 



## immu_db_change_password

> serde_json::Value immu_db_change_password(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaChangePasswordRequest**](SchemaChangePasswordRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_change_permission

> serde_json::Value immu_db_change_permission(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaChangePermissionRequest**](SchemaChangePermissionRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_compact_index

> serde_json::Value immu_db_compact_index()


### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_count

> crate::models::SchemaEntryCount immu_db_count(prefix)
NOT YET SUPPORTED

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | **String** |  | [required] |

### Return type

[**crate::models::SchemaEntryCount**](schemaEntryCount.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_count_all

> crate::models::SchemaEntryCount immu_db_count_all()
NOT YET SUPPORTED

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SchemaEntryCount**](schemaEntryCount.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_create_database

> serde_json::Value immu_db_create_database(body)
DEPRECATED: kept for backward compatibility

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaDatabase**](SchemaDatabase.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_create_database_with

> serde_json::Value immu_db_create_database_with(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaDatabaseSettings**](SchemaDatabaseSettings.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_create_user

> serde_json::Value immu_db_create_user(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaCreateUserRequest**](SchemaCreateUserRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_current_state

> crate::models::SchemaImmutableState immu_db_current_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SchemaImmutableState**](schemaImmutableState.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_database_list

> crate::models::SchemaDatabaseListResponse immu_db_database_list(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::SchemaDatabaseListResponse**](schemaDatabaseListResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_describe_table

> crate::models::SchemaSqlQueryResult immu_db_describe_table(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaTable**](SchemaTable.md) |  | [required] |

### Return type

[**crate::models::SchemaSqlQueryResult**](schemaSQLQueryResult.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_exec_all

> crate::models::SchemaTxMetadata immu_db_exec_all(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaExecAllRequest**](SchemaExecAllRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaTxMetadata**](schemaTxMetadata.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_get

> crate::models::SchemaEntry immu_db_get(key, at_tx, since_tx)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**at_tx** | Option<**String**> |  |  |
**since_tx** | Option<**String**> |  |  |

### Return type

[**crate::models::SchemaEntry**](schemaEntry.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_get_all

> crate::models::SchemaEntries immu_db_get_all(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaKeyListRequest**](SchemaKeyListRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaEntries**](schemaEntries.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_health

> crate::models::SchemaHealthResponse immu_db_health()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SchemaHealthResponse**](schemaHealthResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_history

> crate::models::SchemaEntries immu_db_history(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaHistoryRequest**](SchemaHistoryRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaEntries**](schemaEntries.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_list_tables

> crate::models::SchemaSqlQueryResult immu_db_list_tables()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SchemaSqlQueryResult**](schemaSQLQueryResult.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_list_users

> crate::models::SchemaUserList immu_db_list_users()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SchemaUserList**](schemaUserList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_login

> crate::models::SchemaLoginResponse immu_db_login(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaLoginRequest**](SchemaLoginRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaLoginResponse**](schemaLoginResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_logout

> serde_json::Value immu_db_logout(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_scan

> crate::models::SchemaEntries immu_db_scan(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaScanRequest**](SchemaScanRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaEntries**](schemaEntries.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_set

> crate::models::SchemaTxMetadata immu_db_set(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaSetRequest**](SchemaSetRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaTxMetadata**](schemaTxMetadata.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_set_active_user

> serde_json::Value immu_db_set_active_user(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaSetActiveUserRequest**](SchemaSetActiveUserRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_set_reference

> crate::models::SchemaTxMetadata immu_db_set_reference(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaReferenceRequest**](SchemaReferenceRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaTxMetadata**](schemaTxMetadata.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_update_database

> serde_json::Value immu_db_update_database(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaDatabaseSettings**](SchemaDatabaseSettings.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_use_database

> crate::models::SchemaUseDatabaseReply immu_db_use_database(database_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_name** | **String** |  | [required] |

### Return type

[**crate::models::SchemaUseDatabaseReply**](schemaUseDatabaseReply.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_use_snapshot

> serde_json::Value immu_db_use_snapshot(since_tx, as_before_tx)
SQL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since_tx** | Option<**String**> |  |  |
**as_before_tx** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_verifiable_get

> crate::models::SchemaVerifiableEntry immu_db_verifiable_get(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaVerifiableGetRequest**](SchemaVerifiableGetRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaVerifiableEntry**](schemaVerifiableEntry.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_verifiable_set

> crate::models::SchemaVerifiableTx immu_db_verifiable_set(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaVerifiableSetRequest**](SchemaVerifiableSetRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaVerifiableTx**](schemaVerifiableTx.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_verifiable_set_reference

> crate::models::SchemaVerifiableTx immu_db_verifiable_set_reference(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaVerifiableReferenceRequest**](SchemaVerifiableReferenceRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaVerifiableTx**](schemaVerifiableTx.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_verifiable_sql_get

> crate::models::SchemaVerifiableSqlEntry immu_db_verifiable_sql_get(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaVerifiableSqlGetRequest**](SchemaVerifiableSqlGetRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaVerifiableSqlEntry**](schemaVerifiableSQLEntry.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_verifiable_tx_by_id

> crate::models::SchemaVerifiableTx immu_db_verifiable_tx_by_id(tx, prove_since_tx)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx** | **String** |  | [required] |
**prove_since_tx** | Option<**String**> |  |  |

### Return type

[**crate::models::SchemaVerifiableTx**](schemaVerifiableTx.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_db_verifiable_z_add

> crate::models::SchemaVerifiableTx immu_db_verifiable_z_add(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaVerifiableZAddRequest**](SchemaVerifiableZAddRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaVerifiableTx**](schemaVerifiableTx.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_dbsql_exec

> crate::models::SchemaSqlExecResult immu_dbsql_exec(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaSqlExecRequest**](SchemaSqlExecRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaSqlExecResult**](schemaSQLExecResult.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_dbsql_query

> crate::models::SchemaSqlQueryResult immu_dbsql_query(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaSqlQueryRequest**](SchemaSqlQueryRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaSqlQueryResult**](schemaSQLQueryResult.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_dbtx_by_id

> crate::models::SchemaTx immu_dbtx_by_id(tx)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx** | **String** |  | [required] |

### Return type

[**crate::models::SchemaTx**](schemaTx.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_dbtx_scan

> crate::models::SchemaTxList immu_dbtx_scan(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaTxScanRequest**](SchemaTxScanRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaTxList**](schemaTxList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_dbz_add

> crate::models::SchemaTxMetadata immu_dbz_add(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaZAddRequest**](SchemaZAddRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaTxMetadata**](schemaTxMetadata.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## immu_dbz_scan

> crate::models::SchemaZEntries immu_dbz_scan(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SchemaZScanRequest**](SchemaZScanRequest.md) |  | [required] |

### Return type

[**crate::models::SchemaZEntries**](schemaZEntries.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

