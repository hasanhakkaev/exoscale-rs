# Networking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_cidr** | Option<**String**> | CIDR Range for Pods in cluster. This must not overlap with any IP ranges assigned to pods. Max of two, comma-separated, dual-stack CIDRs is allowed.               If not specified, defaults to 192.168.0.0/16. | [optional]
**service_cluster_ip_range** | Option<**String**> | CIDR range for service cluster IPs. This must not overlap with any IP ranges assigned to nodes or pods. Max of two, comma-separated, dual-stack CIDRs is allowed.               If not specified, defaults to 10.96.0.0/12. | [optional]
**node_cidr_mask_size_ipv4** | Option<**u64**> | Mask size for node cidr in cluster. It must be larger than the Pod CIDR subnet mask. Defaults to 24 | [optional]
**node_cidr_mask_size_ipv6** | Option<**u64**> | Mask size for node cidr in cluster. It must be larger than the Pod CIDR subnet mask. Defaults to 64 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


