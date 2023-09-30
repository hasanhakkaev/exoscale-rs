# DbaasBackupConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_count** | Option<**i64**> | Maximum number of backups to keep. Zero when no backups are created. | [optional][readonly]
**interval** | Option<**i64**> | The interval, in hours, at which backups are generated.                                             For some services, like PostgreSQL, this is the interval                                             at which full snapshots are taken and continuous incremental                                             backup stream is maintained in addition to that. | [optional][readonly]
**recovery_mode** | Option<**String**> | Mechanism how backups can be restored. 'regular'                                             means a backup is restored as is so that the system                                             is restored to the state it was when the backup was generated.                                             'pitr' means point-in-time-recovery, which allows restoring                                             the system to any state since the first available full snapshot. | [optional][readonly]
**frequent_interval_minutes** | Option<**i64**> | Interval of taking a frequent backup in service types supporting different backup schedules | [optional][readonly]
**frequent_oldest_age_minutes** | Option<**i64**> | Maximum age of the oldest frequent backup in service types supporting different backup schedules | [optional][readonly]
**infrequent_interval_minutes** | Option<**i64**> | Interval of taking a frequent backup in service types supporting different backup schedules | [optional][readonly]
**infrequent_oldest_age_minutes** | Option<**i64**> | Maximum age of the oldest infrequent backup in service types supporting different backup schedules | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


