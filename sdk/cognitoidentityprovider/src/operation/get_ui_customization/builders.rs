// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_ui_customization::_get_ui_customization_output::GetUiCustomizationOutputBuilder;

pub use crate::operation::get_ui_customization::_get_ui_customization_input::GetUiCustomizationInputBuilder;

/// Fluent builder constructing a request to `GetUICustomization`.
///
/// <p>Gets the user interface (UI) Customization information for a particular app client's app UI, if any such information exists for the client. If nothing is set for the particular client, but there is an existing pool level customization (the app <code>clientId</code> is <code>ALL</code>), then that information is returned. If nothing is present, then an empty shape is returned.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetUICustomizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_ui_customization::builders::GetUiCustomizationInputBuilder,
}
impl GetUICustomizationFluentBuilder {
    /// Creates a new `GetUICustomization`.
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
            crate::operation::get_ui_customization::GetUICustomization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ui_customization::GetUICustomizationError,
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
        crate::operation::get_ui_customization::GetUiCustomizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ui_customization::GetUICustomizationError,
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
        crate::operation::get_ui_customization::GetUiCustomizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ui_customization::GetUICustomizationError,
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
            crate::operation::get_ui_customization::GetUICustomization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ui_customization::GetUICustomizationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The user pool ID for the user pool.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID for the user pool.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The client ID for the client app.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_id(input.into());
        self
    }
    /// <p>The client ID for the client app.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_id(input);
        self
    }
}
