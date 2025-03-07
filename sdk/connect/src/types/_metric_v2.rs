// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the metric.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetricV2 {
    /// <p>The name of the metric.</p> <important>
    /// <p>This parameter is required. The following Required = No is incorrect.</p>
    /// </important>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Contains information about the threshold for service level metrics.</p>
    #[doc(hidden)]
    pub threshold: ::std::option::Option<::std::vec::Vec<crate::types::ThresholdV2>>,
    /// <p>Contains the filters to be used when returning data.</p>
    #[doc(hidden)]
    pub metric_filters: ::std::option::Option<::std::vec::Vec<crate::types::MetricFilterV2>>,
}
impl MetricV2 {
    /// <p>The name of the metric.</p> <important>
    /// <p>This parameter is required. The following Required = No is incorrect.</p>
    /// </important>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Contains information about the threshold for service level metrics.</p>
    pub fn threshold(&self) -> ::std::option::Option<&[crate::types::ThresholdV2]> {
        self.threshold.as_deref()
    }
    /// <p>Contains the filters to be used when returning data.</p>
    pub fn metric_filters(&self) -> ::std::option::Option<&[crate::types::MetricFilterV2]> {
        self.metric_filters.as_deref()
    }
}
impl MetricV2 {
    /// Creates a new builder-style object to manufacture [`MetricV2`](crate::types::MetricV2).
    pub fn builder() -> crate::types::builders::MetricV2Builder {
        crate::types::builders::MetricV2Builder::default()
    }
}

/// A builder for [`MetricV2`](crate::types::MetricV2).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MetricV2Builder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) threshold: ::std::option::Option<::std::vec::Vec<crate::types::ThresholdV2>>,
    pub(crate) metric_filters: ::std::option::Option<::std::vec::Vec<crate::types::MetricFilterV2>>,
}
impl MetricV2Builder {
    /// <p>The name of the metric.</p> <important>
    /// <p>This parameter is required. The following Required = No is incorrect.</p>
    /// </important>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the metric.</p> <important>
    /// <p>This parameter is required. The following Required = No is incorrect.</p>
    /// </important>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `threshold`.
    ///
    /// To override the contents of this collection use [`set_threshold`](Self::set_threshold).
    ///
    /// <p>Contains information about the threshold for service level metrics.</p>
    pub fn threshold(mut self, input: crate::types::ThresholdV2) -> Self {
        let mut v = self.threshold.unwrap_or_default();
        v.push(input);
        self.threshold = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains information about the threshold for service level metrics.</p>
    pub fn set_threshold(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ThresholdV2>>,
    ) -> Self {
        self.threshold = input;
        self
    }
    /// Appends an item to `metric_filters`.
    ///
    /// To override the contents of this collection use [`set_metric_filters`](Self::set_metric_filters).
    ///
    /// <p>Contains the filters to be used when returning data.</p>
    pub fn metric_filters(mut self, input: crate::types::MetricFilterV2) -> Self {
        let mut v = self.metric_filters.unwrap_or_default();
        v.push(input);
        self.metric_filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains the filters to be used when returning data.</p>
    pub fn set_metric_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricFilterV2>>,
    ) -> Self {
        self.metric_filters = input;
        self
    }
    /// Consumes the builder and constructs a [`MetricV2`](crate::types::MetricV2).
    pub fn build(self) -> crate::types::MetricV2 {
        crate::types::MetricV2 {
            name: self.name,
            threshold: self.threshold,
            metric_filters: self.metric_filters,
        }
    }
}
