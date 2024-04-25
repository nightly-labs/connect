# OAuth2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tls_config** | Option<[**models::TlsConfig**](TLSConfig.md)> |  | [optional]
**client_id** | Option<**String**> |  | [optional]
**client_secret** | Option<**String**> |  | [optional]
**client_secret_file** | Option<**String**> |  | [optional]
**endpoint_params** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**no_proxy** | Option<**String**> | NoProxy contains addresses that should not use a proxy. | [optional]
**proxy_connect_header** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> |  | [optional]
**proxy_from_environment** | Option<**bool**> | ProxyFromEnvironment makes use of net/http ProxyFromEnvironment function to determine proxies. | [optional]
**proxy_url** | Option<[**models::Url**](URL.md)> |  | [optional]
**scopes** | Option<**Vec<String>**> |  | [optional]
**token_url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


