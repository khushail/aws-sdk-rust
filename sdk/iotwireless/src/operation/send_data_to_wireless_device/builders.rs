// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_data_to_wireless_device::_send_data_to_wireless_device_output::SendDataToWirelessDeviceOutputBuilder;

pub use crate::operation::send_data_to_wireless_device::_send_data_to_wireless_device_input::SendDataToWirelessDeviceInputBuilder;

/// Fluent builder constructing a request to `SendDataToWirelessDevice`.
///
/// <p>Sends a decrypted application data frame to a device.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendDataToWirelessDeviceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::send_data_to_wireless_device::builders::SendDataToWirelessDeviceInputBuilder,
}
impl SendDataToWirelessDeviceFluentBuilder {
    /// Creates a new `SendDataToWirelessDevice`.
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
            crate::operation::send_data_to_wireless_device::SendDataToWirelessDevice,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_data_to_wireless_device::SendDataToWirelessDeviceError,
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
        crate::operation::send_data_to_wireless_device::SendDataToWirelessDeviceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_data_to_wireless_device::SendDataToWirelessDeviceError,
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
        crate::operation::send_data_to_wireless_device::SendDataToWirelessDeviceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_data_to_wireless_device::SendDataToWirelessDeviceError,
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
            crate::operation::send_data_to_wireless_device::SendDataToWirelessDevice,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::send_data_to_wireless_device::SendDataToWirelessDeviceError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the wireless device to receive the data.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the wireless device to receive the data.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The transmit mode to use to send data to the wireless device. Can be: <code>0</code> for UM (unacknowledge mode) or <code>1</code> for AM (acknowledge mode).</p>
    pub fn transmit_mode(mut self, input: i32) -> Self {
        self.inner = self.inner.transmit_mode(input);
        self
    }
    /// <p>The transmit mode to use to send data to the wireless device. Can be: <code>0</code> for UM (unacknowledge mode) or <code>1</code> for AM (acknowledge mode).</p>
    pub fn set_transmit_mode(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_transmit_mode(input);
        self
    }
    /// <p>The binary to be sent to the end device, encoded in base64.</p>
    pub fn payload_data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.payload_data(input.into());
        self
    }
    /// <p>The binary to be sent to the end device, encoded in base64.</p>
    pub fn set_payload_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_payload_data(input);
        self
    }
    /// <p>Metadata about the message request.</p>
    pub fn wireless_metadata(mut self, input: crate::types::WirelessMetadata) -> Self {
        self.inner = self.inner.wireless_metadata(input);
        self
    }
    /// <p>Metadata about the message request.</p>
    pub fn set_wireless_metadata(
        mut self,
        input: ::std::option::Option<crate::types::WirelessMetadata>,
    ) -> Self {
        self.inner = self.inner.set_wireless_metadata(input);
        self
    }
}
