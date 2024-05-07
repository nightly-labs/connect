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

/// QueryStat : The embedded FieldConfig's display name must be set. It corresponds to the QueryResultMetaStat on the frontend (https://github.com/grafana/grafana/blob/master/packages/grafana-data/src/types/data.ts#L53).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryStat {
    /// Map values to a display color NOTE: this interface is under development in the frontend... so simple map for now
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Panel Specific Values
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i32>,
    /// Description is human readable field metadata
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// DisplayName overrides Grafana default naming, should not be used from a data source
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// DisplayNameFromDS overrides Grafana default naming strategy.
    #[serde(rename = "displayNameFromDS", skip_serializing_if = "Option::is_none")]
    pub display_name_from_ds: Option<String>,
    /// Filterable indicates if the Field's data can be filtered by additional calls.
    #[serde(rename = "filterable", skip_serializing_if = "Option::is_none")]
    pub filterable: Option<bool>,
    /// Interval indicates the expected regular step between values in the series. When an interval exists, consumers can identify \"missing\" values when the expected value is not present. The grafana timeseries visualization will render disconnected values when missing values are found it the time field. The interval uses the same units as the values.  For time.Time, this is defined in milliseconds.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// The behavior when clicking on a result
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<models::DataLink>>,
    #[serde(rename = "mappings", skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<serde_json::Value>>,
    /// ConfFloat64 is a float64. It Marshals float64 values of NaN of Inf to null.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// ConfFloat64 is a float64. It Marshals float64 values of NaN of Inf to null.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Alternative to empty string
    #[serde(rename = "noValue", skip_serializing_if = "Option::is_none")]
    pub no_value: Option<String>,
    /// Path is an explicit path to the field in the datasource. When the frame meta includes a path, this will default to `${frame.meta.path}/${field.name}  When defined, this value can be used as an identifier within the datasource scope, and may be used as an identifier to update values in a subsequent request
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "thresholds", skip_serializing_if = "Option::is_none")]
    pub thresholds: Option<Box<models::ThresholdsConfig>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<models::FieldTypeConfig>>,
    /// Numeric Options
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Writeable indicates that the datasource knows how to update this value
    #[serde(rename = "writeable", skip_serializing_if = "Option::is_none")]
    pub writeable: Option<bool>,
}

impl QueryStat {
    /// The embedded FieldConfig's display name must be set. It corresponds to the QueryResultMetaStat on the frontend (https://github.com/grafana/grafana/blob/master/packages/grafana-data/src/types/data.ts#L53).
    pub fn new() -> QueryStat {
        QueryStat {
            color: None,
            custom: None,
            decimals: None,
            description: None,
            display_name: None,
            display_name_from_ds: None,
            filterable: None,
            interval: None,
            links: None,
            mappings: None,
            max: None,
            min: None,
            no_value: None,
            path: None,
            thresholds: None,
            r#type: None,
            unit: None,
            value: None,
            writeable: None,
        }
    }
}

