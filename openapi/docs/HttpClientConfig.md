# HttpClientConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authorization** | Option<[**models::Authorization**](Authorization.md)> |  | [optional]
**basic_auth** | Option<[**models::BasicAuth**](BasicAuth.md)> |  | [optional]
**bearer_token** | Option<**String**> |  | [optional]
**bearer_token_file** | Option<**String**> | The bearer token file for the targets. Deprecated in favour of Authorization.CredentialsFile. | [optional]
**enable_http2** | Option<**bool**> | EnableHTTP2 specifies whether the client should configure HTTP2. The omitempty flag is not set, because it would be hidden from the marshalled configuration when set to false. | [optional]
**follow_redirects** | Option<**bool**> | FollowRedirects specifies whether the client should follow HTTP 3xx redirects. The omitempty flag is not set, because it would be hidden from the marshalled configuration when set to false. | [optional]
**no_proxy** | Option<**String**> | NoProxy contains addresses that should not use a proxy. | [optional]
**oauth2** | Option<[**models::OAuth2**](OAuth2.md)> |  | [optional]
**proxy_connect_header** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> |  | [optional]
**proxy_from_environment** | Option<**bool**> | ProxyFromEnvironment makes use of net/http ProxyFromEnvironment function to determine proxies. | [optional]
**proxy_url** | Option<[**models::Url**](URL.md)> |  | [optional]
**tls_config** | Option<[**models::TlsConfig**](TLSConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


