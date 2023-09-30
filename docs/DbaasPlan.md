# DbaasPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | DBaaS plan name | [optional][readonly]
**node_count** | Option<**i64**> | DBaaS plan node count | [optional][readonly]
**node_cpu_count** | Option<**i64**> | DBaaS plan CPU count per node | [optional][readonly]
**disk_space** | Option<**i64**> | DBaaS plan disk space | [optional][readonly]
**node_memory** | Option<**i64**> | DBaaS plan memory count per node | [optional][readonly]
**max_memory_percent** | Option<**i64**> | DBaaS plan max memory allocated percentage | [optional][readonly]
**backup_config** | Option<[**crate::models::DbaasBackupConfig**](dbaas-backup-config.md)> |  | [optional]
**authorized** | Option<**bool**> | Requires authorization or publicly available | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


