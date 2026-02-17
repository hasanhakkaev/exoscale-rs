# SksCluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Cluster description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**cni** | Option<**Cni**> | Cluster CNI (enum: calico, cilium) | [optional]
**auto_upgrade** | Option<**bool**> | Enable auto upgrade of the control plane to the latest patch version available | [optional]
**name** | Option<**String**> | Cluster name | [optional]
**enable_operators_ca** | Option<**bool**> | Indicates whether to add operators certificate authority (CA) as part of trusted CAs for the API server. | [optional]
**state** | Option<**State**> | Cluster state (enum: rotating-csi-credentials, rotating-ccm-credentials, creating, upgrading, deleting, running, suspending, updating, error, rotating-karpenter-credentials, resuming) | [optional][readonly]
**enable_kube_proxy** | Option<**bool**> | Indicates whether to deploy the Kubernetes network proxy. | [optional]
**nodepools** | Option<[**HashSet<models::SksNodepool>**](SksNodepool.md)> | Cluster Nodepools | [optional][readonly]
**level** | Option<**Level**> | Cluster level (enum: starter, pro) | [optional]
**feature_gates** | Option<**Vec<String>**> | A list of Kubernetes-only Alpha features to enable for API server component | [optional]
**addons** | Option<**HashSet<Addons>**> | Cluster addons (enum: exoscale-cloud-controller, exoscale-container-storage-interface, metrics-server, karpenter) | [optional]
**id** | Option<**uuid::Uuid**> | Cluster ID | [optional][readonly]
**audit** | Option<[**models::SksAudit**](SksAudit.md)> |  | [optional]
**version** | Option<**String**> | Control plane Kubernetes version | [optional]
**created_at** | Option<**String**> | Cluster creation date | [optional][readonly]
**endpoint** | Option<**String**> | Cluster endpoint | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


