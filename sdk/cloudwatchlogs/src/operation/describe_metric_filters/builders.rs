// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_metric_filters::_describe_metric_filters_output::DescribeMetricFiltersOutputBuilder;

pub use crate::operation::describe_metric_filters::_describe_metric_filters_input::DescribeMetricFiltersInputBuilder;

/// Fluent builder constructing a request to `DescribeMetricFilters`.
///
/// <p>Lists the specified metric filters. You can list all of the metric filters or filter the results by log name, prefix, metric name, or metric namespace. The results are ASCII-sorted by filter name.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeMetricFiltersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_metric_filters::builders::DescribeMetricFiltersInputBuilder,
}
impl DescribeMetricFiltersFluentBuilder {
    /// Creates a new `DescribeMetricFilters`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_metric_filters::DescribeMetricFilters,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_metric_filters::DescribeMetricFiltersError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_metric_filters::DescribeMetricFiltersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_metric_filters::DescribeMetricFiltersError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_metric_filters::DescribeMetricFiltersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_metric_filters::DescribeMetricFiltersError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_metric_filters::DescribeMetricFilters,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_metric_filters::DescribeMetricFiltersError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_metric_filters::paginator::DescribeMetricFiltersPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_metric_filters::paginator::DescribeMetricFiltersPaginator {
        crate::operation::describe_metric_filters::paginator::DescribeMetricFiltersPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of the log group.</p>
    pub fn log_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// <p>The prefix to match. CloudWatch Logs uses the value that you set here only if you also include the <code>logGroupName</code> parameter in your request.</p>
    pub fn filter_name_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.filter_name_prefix(input.into());
        self
    }
    /// <p>The prefix to match. CloudWatch Logs uses the value that you set here only if you also include the <code>logGroupName</code> parameter in your request.</p>
    pub fn set_filter_name_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_filter_name_prefix(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>Filters results to include only those with the specified metric name. If you include this parameter in your request, you must also include the <code>metricNamespace</code> parameter.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_name(input.into());
        self
    }
    /// <p>Filters results to include only those with the specified metric name. If you include this parameter in your request, you must also include the <code>metricNamespace</code> parameter.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// <p>Filters results to include only those in the specified namespace. If you include this parameter in your request, you must also include the <code>metricName</code> parameter.</p>
    pub fn metric_namespace(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.metric_namespace(input.into());
        self
    }
    /// <p>Filters results to include only those in the specified namespace. If you include this parameter in your request, you must also include the <code>metricName</code> parameter.</p>
    pub fn set_metric_namespace(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_metric_namespace(input);
        self
    }
}
