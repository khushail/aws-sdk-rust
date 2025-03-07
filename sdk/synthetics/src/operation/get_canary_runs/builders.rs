// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_canary_runs::_get_canary_runs_output::GetCanaryRunsOutputBuilder;

pub use crate::operation::get_canary_runs::_get_canary_runs_input::GetCanaryRunsInputBuilder;

/// Fluent builder constructing a request to `GetCanaryRuns`.
///
/// <p>Retrieves a list of runs for a specified canary.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCanaryRunsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_canary_runs::builders::GetCanaryRunsInputBuilder,
}
impl GetCanaryRunsFluentBuilder {
    /// Creates a new `GetCanaryRuns`.
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
            crate::operation::get_canary_runs::GetCanaryRuns,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_canary_runs::GetCanaryRunsError>,
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
        crate::operation::get_canary_runs::GetCanaryRunsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_canary_runs::GetCanaryRunsError>,
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
        crate::operation::get_canary_runs::GetCanaryRunsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_canary_runs::GetCanaryRunsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_canary_runs::GetCanaryRuns,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_canary_runs::GetCanaryRunsError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_canary_runs::paginator::GetCanaryRunsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_canary_runs::paginator::GetCanaryRunsPaginator {
        crate::operation::get_canary_runs::paginator::GetCanaryRunsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of the canary that you want to see runs for.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the canary that you want to see runs for.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>GetCanaryRuns</code> operation to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that indicates that there is more data available. You can use this token in a subsequent <code>GetCanaryRuns</code> operation to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Specify this parameter to limit how many runs are returned each time you use the <code>GetCanaryRuns</code> operation. If you omit this parameter, the default of 100 is used.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Specify this parameter to limit how many runs are returned each time you use the <code>GetCanaryRuns</code> operation. If you omit this parameter, the default of 100 is used.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
