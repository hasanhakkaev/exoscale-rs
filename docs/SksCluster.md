# SksCluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Cluster description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**cni** | Option<**String**> | Cluster CNI | [optional]
**auto_upgrade** | Option<**bool**> | Enable auto upgrade of the control plane to the latest patch version available | [optional]
**name** | Option<**String**> | Cluster name | [optional]
**state** | Option<**String**> | Cluster state | [optional][readonly]
**nodepools** | Option<[**Vec<models::SksNodepool>**](sks-nodepool.md)> | Cluster Nodepools | [optional][readonly]
**level** | Option<**String**> | Cluster level | [optional]
**addons** | Option<**Vec<String>**> | Cluster addons | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Cluster ID | [optional][readonly]
**version** | Option<**String**> | Control plane Kubernetes version | [optional]
**created_at** | Option<**String**> | Cluster creation date | [optional][readonly]
**endpoint** | Option<**String**> | Cluster endpoint | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


