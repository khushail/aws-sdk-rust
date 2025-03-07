// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_voice_connector_group::_get_voice_connector_group_output::GetVoiceConnectorGroupOutputBuilder;

pub use crate::operation::get_voice_connector_group::_get_voice_connector_group_input::GetVoiceConnectorGroupInputBuilder;

/// Fluent builder constructing a request to `GetVoiceConnectorGroup`.
///
/// <p>Retrieves details for the specified Amazon Chime SDK Voice Connector group, such as timestamps,name, and associated <code>VoiceConnectorItems</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetVoiceConnectorGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupInputBuilder,
}
impl GetVoiceConnectorGroupFluentBuilder {
    /// Creates a new `GetVoiceConnectorGroup`.
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
            crate::operation::get_voice_connector_group::GetVoiceConnectorGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_voice_connector_group::GetVoiceConnectorGroupError,
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
        crate::operation::get_voice_connector_group::GetVoiceConnectorGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_voice_connector_group::GetVoiceConnectorGroupError,
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
        crate::operation::get_voice_connector_group::GetVoiceConnectorGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_voice_connector_group::GetVoiceConnectorGroupError,
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
            crate::operation::get_voice_connector_group::GetVoiceConnectorGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_voice_connector_group::GetVoiceConnectorGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Voice Connector group ID.</p>
    pub fn voice_connector_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.voice_connector_group_id(input.into());
        self
    }
    /// <p>The Voice Connector group ID.</p>
    pub fn set_voice_connector_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_voice_connector_group_id(input);
        self
    }
}
