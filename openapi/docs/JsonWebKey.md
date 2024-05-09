# JsonWebKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm** | Option<**String**> | Key algorithm, parsed from `alg` header. | [optional]
**certificate_thumbprint_sha1** | Option<**Vec<i32>**> | X.509 certificate thumbprint (SHA-1), parsed from `x5t` header. | [optional]
**certificate_thumbprint_sha256** | Option<**Vec<i32>**> | X.509 certificate thumbprint (SHA-256), parsed from `x5t#S256` header. | [optional]
**certificates** | Option<[**Vec<models::Certificate>**](Certificate.md)> | X.509 certificate chain, parsed from `x5c` header. | [optional]
**certificates_url** | Option<[**models::Url**](URL.md)> |  | [optional]
**key** | Option<[**serde_json::Value**](.md)> | Key is the Go in-memory representation of this key. It must have one of these types: ed25519.PublicKey ed25519.PrivateKey ecdsa.PublicKey ecdsa.PrivateKey rsa.PublicKey rsa.PrivateKey []byte (a symmetric key)  When marshaling this JSONWebKey into JSON, the \"kty\" header parameter will be automatically set based on the type of this field. | [optional]
**key_id** | Option<**String**> | Key identifier, parsed from `kid` header. | [optional]
**r#use** | Option<**String**> | Key use, parsed from `use` header. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


