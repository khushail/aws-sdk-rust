// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_client_properties::_describe_client_properties_output::DescribeClientPropertiesOutputBuilder;

pub use crate::operation::describe_client_properties::_describe_client_properties_input::DescribeClientPropertiesInputBuilder;

/// Fluent builder constructing a request to `DescribeClientProperties`.
///
/// <p>Retrieves a list that describes one or more specified Amazon WorkSpaces clients.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeClientPropertiesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_client_properties::builders::DescribeClientPropertiesInputBuilder,
}
impl DescribeClientPropertiesFluentBuilder {
    /// Creates a new `DescribeClientProperties`.
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
            crate::operation::describe_client_properties::DescribeClientProperties,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_client_properties::DescribeClientPropertiesError,
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
        crate::operation::describe_client_properties::DescribeClientPropertiesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_client_properties::DescribeClientPropertiesError,
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
        crate::operation::describe_client_properties::DescribeClientPropertiesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_client_properties::DescribeClientPropertiesError,
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
            crate::operation::describe_client_properties::DescribeClientProperties,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_client_properties::DescribeClientPropertiesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `ResourceIds`.
    ///
    /// To override the contents of this collection use [`set_resource_ids`](Self::set_resource_ids).
    ///
    /// <p>The resource identifier, in the form of directory IDs.</p>
    pub fn resource_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_ids(input.into());
        self
    }
    /// <p>The resource identifier, in the form of directory IDs.</p>
    pub fn set_resource_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_resource_ids(input);
        self
    }
}
