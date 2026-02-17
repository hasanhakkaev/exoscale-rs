# RegisterTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_consistent_snapshot_enabled** | Option<**bool**> | Template with support for Application Consistent Snapshots | [optional]
**maintainer** | Option<**String**> | Template maintainer | [optional]
**description** | Option<**String**> | Template description | [optional]
**ssh_key_enabled** | **bool** | Enable SSH key-based login | 
**name** | **String** | Template name | 
**default_user** | Option<**String**> | Template default user | [optional]
**size** | Option<**u64**> | Template size | [optional]
**password_enabled** | **bool** | Enable password-based login | 
**build** | Option<**String**> | Template build | [optional]
**checksum** | **String** | Template MD5 checksum | 
**boot_mode** | Option<**BootMode**> | Boot mode (default: legacy) (enum: legacy, uefi) | [optional]
**url** | **String** | Template source URL | 
**version** | Option<**String**> | Template version | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


