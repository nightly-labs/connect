# Certificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authority_key_id** | Option<**Vec<i32>**> |  | [optional]
**basic_constraints_valid** | Option<**bool**> | BasicConstraintsValid indicates whether IsCA, MaxPathLen, and MaxPathLenZero are valid. | [optional]
**crl_distribution_points** | Option<**Vec<String>**> | CRL Distribution Points | [optional]
**dns_names** | Option<**Vec<String>**> | Subject Alternate Name values. (Note that these values may not be valid if invalid values were contained within a parsed certificate. For example, an element of DNSNames may not be a valid DNS domain name.) | [optional]
**email_addresses** | Option<**Vec<String>**> |  | [optional]
**excluded_dns_domains** | Option<**Vec<String>**> |  | [optional]
**excluded_email_addresses** | Option<**Vec<String>**> |  | [optional]
**excluded_ip_ranges** | Option<[**Vec<models::IpNet>**](IPNet.md)> |  | [optional]
**excluded_uri_domains** | Option<**Vec<String>**> |  | [optional]
**ext_key_usage** | Option<**Vec<i64>**> |  | [optional]
**extensions** | Option<[**Vec<models::Extension>**](Extension.md)> | Extensions contains raw X.509 extensions. When parsing certificates, this can be used to extract non-critical extensions that are not parsed by this package. When marshaling certificates, the Extensions field is ignored, see ExtraExtensions. | [optional]
**extra_extensions** | Option<[**Vec<models::Extension>**](Extension.md)> | ExtraExtensions contains extensions to be copied, raw, into any marshaled certificates. Values override any extensions that would otherwise be produced based on the other fields. The ExtraExtensions field is not populated when parsing certificates, see Extensions. | [optional]
**ip_addresses** | Option<**Vec<String>**> |  | [optional]
**is_ca** | Option<**bool**> |  | [optional]
**issuer** | Option<[**models::Name**](Name.md)> |  | [optional]
**issuing_certificate_url** | Option<**Vec<String>**> |  | [optional]
**key_usage** | Option<**i64**> | KeyUsage represents the set of actions that are valid for a given key. It's a bitmap of the KeyUsage* constants. | [optional]
**max_path_len** | Option<**i64**> | MaxPathLen and MaxPathLenZero indicate the presence and value of the BasicConstraints' \"pathLenConstraint\".  When parsing a certificate, a positive non-zero MaxPathLen means that the field was specified, -1 means it was unset, and MaxPathLenZero being true mean that the field was explicitly set to zero. The case of MaxPathLen==0 with MaxPathLenZero==false should be treated equivalent to -1 (unset).  When generating a certificate, an unset pathLenConstraint can be requested with either MaxPathLen == -1 or using the zero value for both MaxPathLen and MaxPathLenZero. | [optional]
**max_path_len_zero** | Option<**bool**> | MaxPathLenZero indicates that BasicConstraintsValid==true and MaxPathLen==0 should be interpreted as an actual maximum path length of zero. Otherwise, that combination is interpreted as MaxPathLen not being set. | [optional]
**not_before** | Option<**String**> |  | [optional]
**ocsp_server** | Option<**Vec<String>**> | RFC 5280, 4.2.2.1 (Authority Information Access) | [optional]
**permitted_dns_domains** | Option<**Vec<String>**> |  | [optional]
**permitted_dns_domains_critical** | Option<**bool**> | Name constraints | [optional]
**permitted_email_addresses** | Option<**Vec<String>**> |  | [optional]
**permitted_ip_ranges** | Option<[**Vec<models::IpNet>**](IPNet.md)> |  | [optional]
**permitted_uri_domains** | Option<**Vec<String>**> |  | [optional]
**policy_identifiers** | Option<[**Vec<Vec<i64>>**](Vec.md)> |  | [optional]
**public_key** | Option<[**serde_json::Value**](.md)> |  | [optional]
**public_key_algorithm** | Option<**i64**> |  | [optional]
**raw** | Option<**Vec<i32>**> |  | [optional]
**raw_issuer** | Option<**Vec<i32>**> |  | [optional]
**raw_subject** | Option<**Vec<i32>**> |  | [optional]
**raw_subject_public_key_info** | Option<**Vec<i32>**> |  | [optional]
**raw_tbs_certificate** | Option<**Vec<i32>**> |  | [optional]
**serial_number** | Option<**String**> |  | [optional]
**signature** | Option<**Vec<i32>**> |  | [optional]
**signature_algorithm** | Option<**i64**> |  | [optional]
**subject** | Option<[**models::Name**](Name.md)> |  | [optional]
**subject_key_id** | Option<**Vec<i32>**> |  | [optional]
**uris** | Option<[**Vec<models::Url>**](URL.md)> |  | [optional]
**unhandled_critical_extensions** | Option<[**Vec<Vec<i64>>**](Vec.md)> | UnhandledCriticalExtensions contains a list of extension IDs that were not (fully) processed when parsing. Verify will fail if this slice is non-empty, unless verification is delegated to an OS library which understands all the critical extensions.  Users can access these extensions using Extensions and can remove elements from this slice if they believe that they have been handled. | [optional]
**unknown_ext_key_usage** | Option<[**Vec<Vec<i64>>**](Vec.md)> |  | [optional]
**version** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


