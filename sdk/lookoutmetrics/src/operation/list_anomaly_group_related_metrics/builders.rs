// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_anomaly_group_related_metrics::_list_anomaly_group_related_metrics_output::ListAnomalyGroupRelatedMetricsOutputBuilder;

pub use crate::operation::list_anomaly_group_related_metrics::_list_anomaly_group_related_metrics_input::ListAnomalyGroupRelatedMetricsInputBuilder;

/// Fluent builder constructing a request to `ListAnomalyGroupRelatedMetrics`.
///
/// <p>Returns a list of measures that are potential causes or effects of an anomaly group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAnomalyGroupRelatedMetricsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::list_anomaly_group_related_metrics::builders::ListAnomalyGroupRelatedMetricsInputBuilder,
}
impl ListAnomalyGroupRelatedMetricsFluentBuilder {
    /// Creates a new `ListAnomalyGroupRelatedMetrics`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetrics, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetrics, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::list_anomaly_group_related_metrics::ListAnomalyGroupRelatedMetricsError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_anomaly_group_related_metrics::paginator::ListAnomalyGroupRelatedMetricsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_anomaly_group_related_metrics::paginator::ListAnomalyGroupRelatedMetricsPaginator{
        crate::operation::list_anomaly_group_related_metrics::paginator::ListAnomalyGroupRelatedMetricsPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) of the anomaly detector.</p>
    pub fn anomaly_detector_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.anomaly_detector_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the anomaly detector.</p>
    pub fn set_anomaly_detector_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_anomaly_detector_arn(input);
        self
    }
    /// <p>The ID of the anomaly group.</p>
    pub fn anomaly_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.anomaly_group_id(input.into());
        self
    }
    /// <p>The ID of the anomaly group.</p>
    pub fn set_anomaly_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_anomaly_group_id(input);
        self
    }
    /// <p>Filter for potential causes (<code>CAUSE_OF_INPUT_ANOMALY_GROUP</code>) or downstream effects (<code>EFFECT_OF_INPUT_ANOMALY_GROUP</code>) of the anomaly group.</p>
    pub fn relationship_type_filter(mut self, input: crate::types::RelationshipType) -> Self {
        self.inner = self.inner.relationship_type_filter(input);
        self
    }
    /// <p>Filter for potential causes (<code>CAUSE_OF_INPUT_ANOMALY_GROUP</code>) or downstream effects (<code>EFFECT_OF_INPUT_ANOMALY_GROUP</code>) of the anomaly group.</p>
    pub fn set_relationship_type_filter(
        mut self,
        input: ::std::option::Option<crate::types::RelationshipType>,
    ) -> Self {
        self.inner = self.inner.set_relationship_type_filter(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
