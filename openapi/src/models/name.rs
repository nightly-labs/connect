/*
 * Grafana HTTP API.
 *
 * The Grafana backend exposes an HTTP API, the same API is used by the frontend to do everything from saving dashboards, creating users and updating data sources.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: hello@grafana.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// Name : Name represents an X.509 distinguished name. This only includes the common elements of a DN. Note that Name is only an approximation of the X.509 structure. If an accurate representation is needed, asn1.Unmarshal the raw subject or issuer as an RDNSequence.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// ExtraNames contains attributes to be copied, raw, into any marshaled distinguished names. Values override any attributes with the same OID. The ExtraNames field is not populated when parsing, see Names.
    #[serde(rename = "ExtraNames", skip_serializing_if = "Option::is_none")]
    pub extra_names: Option<Vec<models::AttributeTypeAndValue>>,
    #[serde(rename = "Locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<Vec<String>>,
    /// Names contains all parsed attributes. When parsing distinguished names, this can be used to extract non-standard attributes that are not parsed by this package. When marshaling to RDNSequences, the Names field is ignored, see ExtraNames.
    #[serde(rename = "Names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<models::AttributeTypeAndValue>>,
    #[serde(rename = "SerialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "StreetAddress", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<Vec<String>>,
}

impl Name {
    /// Name represents an X.509 distinguished name. This only includes the common elements of a DN. Note that Name is only an approximation of the X.509 structure. If an accurate representation is needed, asn1.Unmarshal the raw subject or issuer as an RDNSequence.
    pub fn new() -> Name {
        Name {
            country: None,
            extra_names: None,
            locality: None,
            names: None,
            serial_number: None,
            street_address: None,
        }
    }
}

