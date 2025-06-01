# IpAddressRateLimitingSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_tries** | Option<**u32**> | The number of login attempts allowed before login is blocked | [optional]
**block_expiry_seconds** | Option<**u16**> | The duration of time that login remains blocked after a failed login | [optional]
**max_blocked_clients** | Option<**u32**> | The maximum number of blocked IP addresses | [optional]
**max_tracked_clients** | Option<**u32**> | The maximum number of tracked IP addresses that have failed login | [optional]
**time_window_seconds** | Option<**u16**> | The window of time in which the value for `allowed_tries` is enforced | [optional]
**r#type** | Option<**String**> | The type of rate limiting | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


