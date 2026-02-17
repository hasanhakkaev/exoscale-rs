# AutovacuumSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**log_autovacuum_min_duration** | Option<**i32**> | Causes each action executed by autovacuum to be logged if it ran for at least the specified number of milliseconds. Setting this to zero logs all autovacuum actions. Minus-one disables logging autovacuum actions. The default is `1000`. | [optional]
**autovacuum_vacuum_cost_limit** | Option<**i16**> | Specifies the cost limit value that will be used in automatic VACUUM operations. If `-1` is specified, the regular vacuum_cost_limit value will be used. The default is `-1` (upstream default). | [optional]
**autovacuum_max_workers** | Option<**u8**> | Specifies the maximum number of autovacuum processes (other than the autovacuum launcher) that may be running at any one time. The default is `3`. Changing this parameter causes a service restart. | [optional]
**autovacuum_vacuum_threshold** | Option<**u32**> | Specifies the minimum number of updated or deleted tuples needed to trigger a VACUUM in any one table. The default is `50`. | [optional]
**autovacuum_naptime** | Option<**u32**> | Specifies the minimum delay between autovacuum runs on any given database. The delay is measured in seconds. The default is `60`. | [optional]
**autovacuum_vacuum_scale_factor** | Option<**f64**> | Specifies a fraction of the table size to add to autovacuum_vacuum_threshold when deciding whether to trigger a VACUUM (e.g. `0.2` for 20% of the table size). The default is `0.2`. | [optional]
**autovacuum_vacuum_cost_delay** | Option<**i8**> | Specifies the cost delay value that will be used in automatic VACUUM operations. If `-1` is specified, the regular vacuum_cost_delay value will be used. The default is `2` (upstream default). | [optional]
**autovacuum_analyze_scale_factor** | Option<**f64**> | Specifies a fraction of the table size to add to autovacuum_analyze_threshold when deciding whether to trigger an ANALYZE (e.g. `0.2` for 20% of the table size). The default is `0.2`. | [optional]
**autovacuum_analyze_threshold** | Option<**u32**> | Specifies the minimum number of inserted, updated or deleted tuples needed to trigger an ANALYZE in any one table. The default is `50`. | [optional]
**autovacuum_freeze_max_age** | Option<**u32**> | Specifies the maximum age (in transactions) that a table's pg_class.relfrozenxid field can attain before a VACUUM operation is forced to prevent transaction ID wraparound within the table. The system launches autovacuum processes to prevent wraparound even when autovacuum is otherwise disabled. Changing this parameter causes a service restart. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


