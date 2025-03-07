// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_contact_recording::_stop_contact_recording_output::StopContactRecordingOutputBuilder;

pub use crate::operation::stop_contact_recording::_stop_contact_recording_input::StopContactRecordingInputBuilder;

/// Fluent builder constructing a request to `StopContactRecording`.
///
/// <p>Stops recording a call when a contact is being recorded. StopContactRecording is a one-time action. If you use StopContactRecording to stop recording an ongoing call, you can't use StartContactRecording to restart it. For scenarios where the recording has started and you want to suspend it for sensitive information (for example, to collect a credit card number), and then restart it, use SuspendContactRecording and ResumeContactRecording.</p>
/// <p>Only voice recordings are supported at this time.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopContactRecordingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_contact_recording::builders::StopContactRecordingInputBuilder,
}
impl StopContactRecordingFluentBuilder {
    /// Creates a new `StopContactRecording`.
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
            crate::operation::stop_contact_recording::StopContactRecording,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_contact_recording::StopContactRecordingError,
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
        crate::operation::stop_contact_recording::StopContactRecordingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_contact_recording::StopContactRecordingError,
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
        crate::operation::stop_contact_recording::StopContactRecordingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_contact_recording::StopContactRecordingError,
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
            crate::operation::stop_contact_recording::StopContactRecording,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::stop_contact_recording::StopContactRecordingError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the contact.</p>
    pub fn contact_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.contact_id(input.into());
        self
    }
    /// <p>The identifier of the contact.</p>
    pub fn set_contact_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_contact_id(input);
        self
    }
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    pub fn initial_contact_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.initial_contact_id(input.into());
        self
    }
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    pub fn set_initial_contact_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_initial_contact_id(input);
        self
    }
}
