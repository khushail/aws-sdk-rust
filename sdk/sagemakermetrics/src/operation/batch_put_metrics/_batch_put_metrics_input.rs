// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchPutMetricsInput {
    /// <p>The name of the Trial Component to associate with the metrics.</p>
    #[doc(hidden)]
    pub trial_component_name: ::std::option::Option<::std::string::String>,
    /// <p>A list of raw metric values to put.</p>
    #[doc(hidden)]
    pub metric_data: ::std::option::Option<::std::vec::Vec<crate::types::RawMetricData>>,
}
impl BatchPutMetricsInput {
    /// <p>The name of the Trial Component to associate with the metrics.</p>
    pub fn trial_component_name(&self) -> ::std::option::Option<&str> {
        self.trial_component_name.as_deref()
    }
    /// <p>A list of raw metric values to put.</p>
    pub fn metric_data(&self) -> ::std::option::Option<&[crate::types::RawMetricData]> {
        self.metric_data.as_deref()
    }
}
impl BatchPutMetricsInput {
    /// Creates a new builder-style object to manufacture [`BatchPutMetricsInput`](crate::operation::batch_put_metrics::BatchPutMetricsInput).
    pub fn builder() -> crate::operation::batch_put_metrics::builders::BatchPutMetricsInputBuilder {
        crate::operation::batch_put_metrics::builders::BatchPutMetricsInputBuilder::default()
    }
}

/// A builder for [`BatchPutMetricsInput`](crate::operation::batch_put_metrics::BatchPutMetricsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchPutMetricsInputBuilder {
    pub(crate) trial_component_name: ::std::option::Option<::std::string::String>,
    pub(crate) metric_data: ::std::option::Option<::std::vec::Vec<crate::types::RawMetricData>>,
}
impl BatchPutMetricsInputBuilder {
    /// <p>The name of the Trial Component to associate with the metrics.</p>
    pub fn trial_component_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.trial_component_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Trial Component to associate with the metrics.</p>
    pub fn set_trial_component_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.trial_component_name = input;
        self
    }
    /// Appends an item to `metric_data`.
    ///
    /// To override the contents of this collection use [`set_metric_data`](Self::set_metric_data).
    ///
    /// <p>A list of raw metric values to put.</p>
    pub fn metric_data(mut self, input: crate::types::RawMetricData) -> Self {
        let mut v = self.metric_data.unwrap_or_default();
        v.push(input);
        self.metric_data = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of raw metric values to put.</p>
    pub fn set_metric_data(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RawMetricData>>,
    ) -> Self {
        self.metric_data = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchPutMetricsInput`](crate::operation::batch_put_metrics::BatchPutMetricsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_put_metrics::BatchPutMetricsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_put_metrics::BatchPutMetricsInput {
            trial_component_name: self.trial_component_name,
            metric_data: self.metric_data,
        })
    }
}
