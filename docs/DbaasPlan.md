# DbaasPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_count** | Option<**u64**> | DBaaS plan node count | [optional][readonly]
**backup_config** | Option<[**models::DbaasBackupConfig**](dbaas-backup-config.md)> |  | [optional]
**node_cpu_count** | Option<**u64**> | DBaaS plan CPU count per node | [optional][readonly]
**family** | Option<**String**> | Instance family subset which the service can use | [optional]
**disk_space** | Option<**i64**> | DBaaS plan disk space | [optional][readonly]
**authorized** | Option<**bool**> | Requires authorization or publicly available | [optional][readonly]
**name** | Option<**String**> | DBaaS plan name | [optional][readonly]
**max_memory_percent** | Option<**u64**> | DBaaS plan max memory allocated percentage | [optional][readonly]
**zones** | Option<**Vec<String>**> | Zones where the plan is available | [optional]
**node_memory** | Option<**u64**> | DBaaS plan memory count per node | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


