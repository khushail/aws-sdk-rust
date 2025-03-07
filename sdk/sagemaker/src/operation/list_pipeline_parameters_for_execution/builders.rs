// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_pipeline_parameters_for_execution::_list_pipeline_parameters_for_execution_output::ListPipelineParametersForExecutionOutputBuilder;

pub use crate::operation::list_pipeline_parameters_for_execution::_list_pipeline_parameters_for_execution_input::ListPipelineParametersForExecutionInputBuilder;

/// Fluent builder constructing a request to `ListPipelineParametersForExecution`.
///
/// <p>Gets a list of parameters for a pipeline execution.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPipelineParametersForExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::list_pipeline_parameters_for_execution::builders::ListPipelineParametersForExecutionInputBuilder,
}
impl ListPipelineParametersForExecutionFluentBuilder {
    /// Creates a new `ListPipelineParametersForExecution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecution, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionOutput, ::aws_smithy_http::result::SdkError<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecution, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::list_pipeline_parameters_for_execution::ListPipelineParametersForExecutionError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_pipeline_parameters_for_execution::paginator::ListPipelineParametersForExecutionPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_pipeline_parameters_for_execution::paginator::ListPipelineParametersForExecutionPaginator{
        crate::operation::list_pipeline_parameters_for_execution::paginator::ListPipelineParametersForExecutionPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
    pub fn pipeline_execution_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pipeline_execution_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the pipeline execution.</p>
    pub fn set_pipeline_execution_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_execution_arn(input);
        self
    }
    /// <p>If the result of the previous <code>ListPipelineParametersForExecution</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of parameters, use the token in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the result of the previous <code>ListPipelineParametersForExecution</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of parameters, use the token in the next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of parameters to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of parameters to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
