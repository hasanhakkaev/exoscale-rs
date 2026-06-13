# JsonSchemaPgaudit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role** | Option<**String**> | Specifies the master role to use for object audit logging. | [optional]
**log_parameter** | Option<**bool**> | Specifies that audit logging should include the parameters that were passed with the statement. | [optional][default to false]
**log_rows** | Option<**bool**> |  | [optional][default to false]
**log_level** | Option<**LogLevel**> | Specifies the log level that will be used for log entries. (enum: debug1, debug2, debug3, debug4, debug5, info, notice, warning, log) | [optional][default to Log]
**log_relation** | Option<**bool**> | Specifies whether session audit logging should create a separate log entry for each relation (TABLE, VIEW, etc.) referenced in a SELECT or DML statement. | [optional][default to false]
**log_statement_once** | Option<**bool**> | Specifies whether logging will include the statement text and parameters with the first log entry for a statement/substatement combination or with every entry. | [optional][default to false]
**log_max_string_length** | Option<**i32**> | Crop parameters representation and whole statements if they exceed this threshold. A (default) value of -1 disable the truncation. | [optional][default to -1]
**log_catalog** | Option<**bool**> | Specifies that session logging should be enabled in the case where all relations in a statement are in pg_catalog. | [optional][default to true]
**log_nested_statements** | Option<**bool**> | This GUC allows to turn off logging nested statements, that is, statements that are executed as part of another ExecutorRun. | [optional][default to true]
**log_statement** | Option<**bool**> | Specifies whether logging will include the statement text and parameters (if enabled). | [optional][default to true]
**log_client** | Option<**bool**> | Specifies whether log messages will be visible to a client process such as psql. | [optional][default to false]
**feature_enabled** | Option<**bool**> | Enable pgaudit extension. When enabled, pgaudit extension will be automatically installed.Otherwise, extension will be uninstalled but auditing configurations will be preserved. | [optional][default to false]
**log** | Option<**Vec<Log>**> | Specifies which classes of statements will be logged by session audit logging. (enum: all, ddl, function, misc, misc_set, read, role, write) | [optional]
**log_parameter_max_size** | Option<**i32**> | Specifies that parameter values longer than this setting (in bytes) should not be logged, but replaced with <long param suppressed>. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


