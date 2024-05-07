# FloatHistogram

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**f64**> | Total number of observations. Must be zero or positive. | [optional]
**counter_reset_hint** | Option<**i32**> | or alternatively that we are dealing with a gauge histogram, where counter resets do not apply. | [optional]
**positive_buckets** | Option<**Vec<f64>**> | Observation counts in buckets. Each represents an absolute count and must be zero or positive. | [optional]
**positive_spans** | Option<[**Vec<models::Span>**](Span.md)> | Spans for positive and negative buckets (see Span below). | [optional]
**schema** | Option<**i32**> | Currently valid schema numbers are -4 <= n <= 8.  They are all for base-2 bucket schemas, where 1 is a bucket boundary in each case, and then each power of two is divided into 2^n logarithmic buckets.  Or in other words, each bucket boundary is the previous boundary times 2^(2^-n). | [optional]
**sum** | Option<**f64**> | Sum of observations. This is also used as the stale marker. | [optional]
**zero_count** | Option<**f64**> | Observations falling into the zero bucket. Must be zero or positive. | [optional]
**zero_threshold** | Option<**f64**> | Width of the zero bucket. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


