# DbaasNodeState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the service node | 
**progress_updates** | Option<[**Vec<models::DbaasNodeStateProgressUpdate>**](DbaasNodeStateProgressUpdate.md)> | Extra information regarding the progress for current state | [optional]
**role** | Option<**Role**> | Role of this node. Only returned for a subset of service types (enum: standby, master, read-replica) | [optional]
**state** | **State** | Current state of the service node (enum: leaving, running, syncing_data, setting_up_vm, unknown) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


