// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_input_device_transfer::_cancel_input_device_transfer_output::CancelInputDeviceTransferOutputBuilder;

pub use crate::operation::cancel_input_device_transfer::_cancel_input_device_transfer_input::CancelInputDeviceTransferInputBuilder;

/// Fluent builder constructing a request to `CancelInputDeviceTransfer`.
///
/// Cancel an input device transfer that you have requested.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelInputDeviceTransferFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::cancel_input_device_transfer::builders::CancelInputDeviceTransferInputBuilder,
}
impl CancelInputDeviceTransferFluentBuilder {
    /// Creates a new `CancelInputDeviceTransfer`.
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
            crate::operation::cancel_input_device_transfer::CancelInputDeviceTransfer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_input_device_transfer::CancelInputDeviceTransferError,
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
        crate::operation::cancel_input_device_transfer::CancelInputDeviceTransferOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_input_device_transfer::CancelInputDeviceTransferError,
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
        crate::operation::cancel_input_device_transfer::CancelInputDeviceTransferOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_input_device_transfer::CancelInputDeviceTransferError,
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
            crate::operation::cancel_input_device_transfer::CancelInputDeviceTransfer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::cancel_input_device_transfer::CancelInputDeviceTransferError,
        >,
    > {
        self.customize_middleware().await
    }
    /// The unique ID of the input device to cancel. For example, hd-123456789abcdef.
    pub fn input_device_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.input_device_id(input.into());
        self
    }
    /// The unique ID of the input device to cancel. For example, hd-123456789abcdef.
    pub fn set_input_device_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_input_device_id(input);
        self
    }
}
