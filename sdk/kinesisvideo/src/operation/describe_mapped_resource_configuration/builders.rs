// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_mapped_resource_configuration::_describe_mapped_resource_configuration_output::DescribeMappedResourceConfigurationOutputBuilder;

pub use crate::operation::describe_mapped_resource_configuration::_describe_mapped_resource_configuration_input::DescribeMappedResourceConfigurationInputBuilder;

/// Fluent builder constructing a request to `DescribeMappedResourceConfiguration`.
///
/// <p>Returns the most current information about the stream. Either streamName or streamARN should be provided in the input.</p>
/// <p>Returns the most current information about the stream. The <code>streamName</code> or <code>streamARN</code> should be provided in the input.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeMappedResourceConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_mapped_resource_configuration::builders::DescribeMappedResourceConfigurationInputBuilder,
}
impl DescribeMappedResourceConfigurationFluentBuilder {
    /// Creates a new `DescribeMappedResourceConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfigurationError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfigurationError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfigurationOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfigurationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfiguration, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::describe_mapped_resource_configuration::DescribeMappedResourceConfigurationError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_mapped_resource_configuration::paginator::DescribeMappedResourceConfigurationPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_mapped_resource_configuration::paginator::DescribeMappedResourceConfigurationPaginator{
        crate::operation::describe_mapped_resource_configuration::paginator::DescribeMappedResourceConfigurationPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the stream.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token to provide in your next request, to get another batch of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to provide in your next request, to get another batch of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
