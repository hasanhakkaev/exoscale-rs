# ShardIndexingBackPressureSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**primary_parameter** | Option<[**models::PrimaryParameter**](Primary_parameter.md)> |  | [optional]
**operating_factor** | Option<[**models::OperatingFactor**](Operating_factor.md)> |  | [optional]
**enforced** | Option<**bool**> | Run shard indexing backpressure in shadow mode or enforced mode. In shadow mode (value set as false), shard indexing backpressure tracks all granular-level metrics, but it doesn’t actually reject any indexing requests. In enforced mode (value set as true), shard indexing backpressure rejects any requests to the cluster that might cause a dip in its performance. Default is false | [optional]
**enabled** | Option<**bool**> | Enable or disable shard indexing backpressure. Default is false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


