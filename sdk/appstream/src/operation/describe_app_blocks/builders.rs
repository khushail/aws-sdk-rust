// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_app_blocks::_describe_app_blocks_output::DescribeAppBlocksOutputBuilder;

pub use crate::operation::describe_app_blocks::_describe_app_blocks_input::DescribeAppBlocksInputBuilder;

/// Fluent builder constructing a request to `DescribeAppBlocks`.
///
/// <p>Retrieves a list that describes one or more app blocks.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAppBlocksFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_app_blocks::builders::DescribeAppBlocksInputBuilder,
}
impl DescribeAppBlocksFluentBuilder {
    /// Creates a new `DescribeAppBlocks`.
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
            crate::operation::describe_app_blocks::DescribeAppBlocks,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_app_blocks::DescribeAppBlocksError,
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
        crate::operation::describe_app_blocks::DescribeAppBlocksOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_app_blocks::DescribeAppBlocksError,
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
        crate::operation::describe_app_blocks::DescribeAppBlocksOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_app_blocks::DescribeAppBlocksError,
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
            crate::operation::describe_app_blocks::DescribeAppBlocks,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_app_blocks::DescribeAppBlocksError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `Arns`.
    ///
    /// To override the contents of this collection use [`set_arns`](Self::set_arns).
    ///
    /// <p>The ARNs of the app blocks.</p>
    pub fn arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arns(input.into());
        self
    }
    /// <p>The ARNs of the app blocks.</p>
    pub fn set_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_arns(input);
        self
    }
    /// <p>The pagination token used to retrieve the next page of results for this operation.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token used to retrieve the next page of results for this operation.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum size of each page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
