// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListFleetMetricsOutput {
    /// <p>The list of fleet metrics objects.</p>
    #[doc(hidden)]
    pub fleet_metrics: ::std::option::Option<::std::vec::Vec<crate::types::FleetMetricNameAndArn>>,
    /// <p>The token for the next set of results. Will not be returned if the operation has returned all results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListFleetMetricsOutput {
    /// <p>The list of fleet metrics objects.</p>
    pub fn fleet_metrics(&self) -> ::std::option::Option<&[crate::types::FleetMetricNameAndArn]> {
        self.fleet_metrics.as_deref()
    }
    /// <p>The token for the next set of results. Will not be returned if the operation has returned all results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListFleetMetricsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListFleetMetricsOutput {
    /// Creates a new builder-style object to manufacture [`ListFleetMetricsOutput`](crate::operation::list_fleet_metrics::ListFleetMetricsOutput).
    pub fn builder() -> crate::operation::list_fleet_metrics::builders::ListFleetMetricsOutputBuilder
    {
        crate::operation::list_fleet_metrics::builders::ListFleetMetricsOutputBuilder::default()
    }
}

/// A builder for [`ListFleetMetricsOutput`](crate::operation::list_fleet_metrics::ListFleetMetricsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListFleetMetricsOutputBuilder {
    pub(crate) fleet_metrics:
        ::std::option::Option<::std::vec::Vec<crate::types::FleetMetricNameAndArn>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListFleetMetricsOutputBuilder {
    /// Appends an item to `fleet_metrics`.
    ///
    /// To override the contents of this collection use [`set_fleet_metrics`](Self::set_fleet_metrics).
    ///
    /// <p>The list of fleet metrics objects.</p>
    pub fn fleet_metrics(mut self, input: crate::types::FleetMetricNameAndArn) -> Self {
        let mut v = self.fleet_metrics.unwrap_or_default();
        v.push(input);
        self.fleet_metrics = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of fleet metrics objects.</p>
    pub fn set_fleet_metrics(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FleetMetricNameAndArn>>,
    ) -> Self {
        self.fleet_metrics = input;
        self
    }
    /// <p>The token for the next set of results. Will not be returned if the operation has returned all results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of results. Will not be returned if the operation has returned all results.</p>
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
    /// Consumes the builder and constructs a [`ListFleetMetricsOutput`](crate::operation::list_fleet_metrics::ListFleetMetricsOutput).
    pub fn build(self) -> crate::operation::list_fleet_metrics::ListFleetMetricsOutput {
        crate::operation::list_fleet_metrics::ListFleetMetricsOutput {
            fleet_metrics: self.fleet_metrics,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
