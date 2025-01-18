# Template

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maintainer** | Option<**String**> | Template maintainer | [optional][readonly]
**description** | Option<**String**> | Template description | [optional]
**ssh_key_enabled** | Option<**bool**> | Enable SSH key-based login | [optional]
**family** | Option<**String**> | Template family | [optional][readonly]
**name** | Option<**String**> | Template name | [optional]
**default_user** | Option<**String**> | Template default user | [optional]
**size** | Option<**i64**> | Template size | [optional]
**password_enabled** | Option<**bool**> | Enable password-based login | [optional]
**build** | Option<**String**> | Template build | [optional][readonly]
**checksum** | Option<**String**> | Template MD5 checksum | [optional]
**boot_mode** | Option<**String**> | Boot mode (default: legacy) | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Template ID | [optional][readonly]
**zones** | Option<[**Vec<models::ZoneName>**](zone-name.md)> | Zones availability | [optional]
**url** | Option<**String**> | Template source URL | [optional]
**version** | Option<**String**> | Template version | [optional][readonly]
**created_at** | Option<**String**> | Template creation date | [optional][readonly]
**visibility** | Option<**String**> | Template visibility | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


