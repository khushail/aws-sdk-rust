// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_inbound_dmarc_settings::_put_inbound_dmarc_settings_output::PutInboundDmarcSettingsOutputBuilder;

pub use crate::operation::put_inbound_dmarc_settings::_put_inbound_dmarc_settings_input::PutInboundDmarcSettingsInputBuilder;

/// Fluent builder constructing a request to `PutInboundDmarcSettings`.
///
/// <p>Enables or disables a DMARC policy for a given organization.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutInboundDmarcSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::put_inbound_dmarc_settings::builders::PutInboundDmarcSettingsInputBuilder,
}
impl PutInboundDmarcSettingsFluentBuilder {
    /// Creates a new `PutInboundDmarcSettings`.
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
            crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsError,
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
        crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsError,
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
        crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsError,
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
            crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_inbound_dmarc_settings::PutInboundDmarcSettingsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the organization that you are applying the DMARC policy to.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The ID of the organization that you are applying the DMARC policy to.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>Enforces or suspends a policy after it's applied.</p>
    pub fn enforced(mut self, input: bool) -> Self {
        self.inner = self.inner.enforced(input);
        self
    }
    /// <p>Enforces or suspends a policy after it's applied.</p>
    pub fn set_enforced(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enforced(input);
        self
    }
}
