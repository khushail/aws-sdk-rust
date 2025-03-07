// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetContainerServiceMetricDataOutput {
    /// <p>The name of the metric returned. </p>
    #[doc(hidden)]
    pub metric_name: ::std::option::Option<crate::types::ContainerServiceMetricName>,
    /// <p>An array of objects that describe the metric data returned.</p>
    #[doc(hidden)]
    pub metric_data: ::std::option::Option<::std::vec::Vec<crate::types::MetricDatapoint>>,
    _request_id: Option<String>,
}
impl GetContainerServiceMetricDataOutput {
    /// <p>The name of the metric returned. </p>
    pub fn metric_name(&self) -> ::std::option::Option<&crate::types::ContainerServiceMetricName> {
        self.metric_name.as_ref()
    }
    /// <p>An array of objects that describe the metric data returned.</p>
    pub fn metric_data(&self) -> ::std::option::Option<&[crate::types::MetricDatapoint]> {
        self.metric_data.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetContainerServiceMetricDataOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetContainerServiceMetricDataOutput {
    /// Creates a new builder-style object to manufacture [`GetContainerServiceMetricDataOutput`](crate::operation::get_container_service_metric_data::GetContainerServiceMetricDataOutput).
    pub fn builder() -> crate::operation::get_container_service_metric_data::builders::GetContainerServiceMetricDataOutputBuilder{
        crate::operation::get_container_service_metric_data::builders::GetContainerServiceMetricDataOutputBuilder::default()
    }
}

/// A builder for [`GetContainerServiceMetricDataOutput`](crate::operation::get_container_service_metric_data::GetContainerServiceMetricDataOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetContainerServiceMetricDataOutputBuilder {
    pub(crate) metric_name: ::std::option::Option<crate::types::ContainerServiceMetricName>,
    pub(crate) metric_data: ::std::option::Option<::std::vec::Vec<crate::types::MetricDatapoint>>,
    _request_id: Option<String>,
}
impl GetContainerServiceMetricDataOutputBuilder {
    /// <p>The name of the metric returned. </p>
    pub fn metric_name(mut self, input: crate::types::ContainerServiceMetricName) -> Self {
        self.metric_name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the metric returned. </p>
    pub fn set_metric_name(
        mut self,
        input: ::std::option::Option<crate::types::ContainerServiceMetricName>,
    ) -> Self {
        self.metric_name = input;
        self
    }
    /// Appends an item to `metric_data`.
    ///
    /// To override the contents of this collection use [`set_metric_data`](Self::set_metric_data).
    ///
    /// <p>An array of objects that describe the metric data returned.</p>
    pub fn metric_data(mut self, input: crate::types::MetricDatapoint) -> Self {
        let mut v = self.metric_data.unwrap_or_default();
        v.push(input);
        self.metric_data = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of objects that describe the metric data returned.</p>
    pub fn set_metric_data(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricDatapoint>>,
    ) -> Self {
        self.metric_data = input;
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
    /// Consumes the builder and constructs a [`GetContainerServiceMetricDataOutput`](crate::operation::get_container_service_metric_data::GetContainerServiceMetricDataOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_container_service_metric_data::GetContainerServiceMetricDataOutput
    {
        crate::operation::get_container_service_metric_data::GetContainerServiceMetricDataOutput {
            metric_name: self.metric_name,
            metric_data: self.metric_data,
            _request_id: self._request_id,
        }
    }
}
