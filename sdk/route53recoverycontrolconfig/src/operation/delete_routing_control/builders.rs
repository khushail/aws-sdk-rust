// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_routing_control::_delete_routing_control_output::DeleteRoutingControlOutputBuilder;

pub use crate::operation::delete_routing_control::_delete_routing_control_input::DeleteRoutingControlInputBuilder;

/// Fluent builder constructing a request to `DeleteRoutingControl`.
///
/// <p>Deletes a routing control.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteRoutingControlFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_routing_control::builders::DeleteRoutingControlInputBuilder,
}
impl DeleteRoutingControlFluentBuilder {
    /// Creates a new `DeleteRoutingControl`.
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
            crate::operation::delete_routing_control::DeleteRoutingControl,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_routing_control::DeleteRoutingControlError,
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
        crate::operation::delete_routing_control::DeleteRoutingControlOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_routing_control::DeleteRoutingControlError,
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
        crate::operation::delete_routing_control::DeleteRoutingControlOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_routing_control::DeleteRoutingControlError,
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
            crate::operation::delete_routing_control::DeleteRoutingControl,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_routing_control::DeleteRoutingControlError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the routing control that you're deleting.</p>
    pub fn routing_control_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.routing_control_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the routing control that you're deleting.</p>
    pub fn set_routing_control_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_routing_control_arn(input);
        self
    }
}
