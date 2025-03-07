// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAnomalyGroupRelatedMetricsOutput {
    /// <p>Aggregated details about the measures contributing to the anomaly group, and the measures potentially impacted by the anomaly group.</p>
    #[doc(hidden)]
    pub inter_metric_impact_list:
        ::std::option::Option<::std::vec::Vec<crate::types::InterMetricImpactDetails>>,
    /// <p>The pagination token that's included if more results are available.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAnomalyGroupRelatedMetricsOutput {
    /// <p>Aggregated details about the measures contributing to the anomaly group, and the measures potentially impacted by the anomaly group.</p>
    pub fn inter_metric_impact_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::InterMetricImpactDetails]> {
        self.inter_metric_impact_list.as_deref()
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListAnomalyGroupRelatedMetricsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAnomalyGroupRelatedMetricsOutput {
    /// Creates a new builder-style object to manufacture [`ListAnomalyGroupRelatedMetricsOutput`](crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsOutput).
    pub fn builder() -> crate::operation::list_anomaly_group_related_metrics::builders::ListAnomalyGroupRelatedMetricsOutputBuilder{
        crate::operation::list_anomaly_group_related_metrics::builders::ListAnomalyGroupRelatedMetricsOutputBuilder::default()
    }
}

/// A builder for [`ListAnomalyGroupRelatedMetricsOutput`](crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAnomalyGroupRelatedMetricsOutputBuilder {
    pub(crate) inter_metric_impact_list:
        ::std::option::Option<::std::vec::Vec<crate::types::InterMetricImpactDetails>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAnomalyGroupRelatedMetricsOutputBuilder {
    /// Appends an item to `inter_metric_impact_list`.
    ///
    /// To override the contents of this collection use [`set_inter_metric_impact_list`](Self::set_inter_metric_impact_list).
    ///
    /// <p>Aggregated details about the measures contributing to the anomaly group, and the measures potentially impacted by the anomaly group.</p>
    pub fn inter_metric_impact_list(
        mut self,
        input: crate::types::InterMetricImpactDetails,
    ) -> Self {
        let mut v = self.inter_metric_impact_list.unwrap_or_default();
        v.push(input);
        self.inter_metric_impact_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>Aggregated details about the measures contributing to the anomaly group, and the measures potentially impacted by the anomaly group.</p>
    pub fn set_inter_metric_impact_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InterMetricImpactDetails>>,
    ) -> Self {
        self.inter_metric_impact_list = input;
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListAnomalyGroupRelatedMetricsOutput`](crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsOutput
    {
        crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsOutput {
            inter_metric_impact_list: self.inter_metric_impact_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
