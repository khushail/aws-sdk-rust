// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_mobile_sdk_release::_get_mobile_sdk_release_output::GetMobileSdkReleaseOutputBuilder;

pub use crate::operation::get_mobile_sdk_release::_get_mobile_sdk_release_input::GetMobileSdkReleaseInputBuilder;

/// Fluent builder constructing a request to `GetMobileSdkRelease`.
///
/// <p>Retrieves information for the specified mobile SDK release, including release notes and tags.</p>
/// <p>The mobile SDK is not generally available. Customers who have access to the mobile SDK can use it to establish and manage WAF tokens for use in HTTP(S) requests from a mobile device to WAF. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-application-integration.html">WAF client application integration</a> in the <i>WAF Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMobileSdkReleaseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_mobile_sdk_release::builders::GetMobileSdkReleaseInputBuilder,
}
impl GetMobileSdkReleaseFluentBuilder {
    /// Creates a new `GetMobileSdkRelease`.
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
            crate::operation::get_mobile_sdk_release::GetMobileSdkRelease,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseError,
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
        crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseError,
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
        crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseError,
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
            crate::operation::get_mobile_sdk_release::GetMobileSdkRelease,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_mobile_sdk_release::GetMobileSdkReleaseError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The device platform.</p>
    pub fn platform(mut self, input: crate::types::Platform) -> Self {
        self.inner = self.inner.platform(input);
        self
    }
    /// <p>The device platform.</p>
    pub fn set_platform(mut self, input: ::std::option::Option<crate::types::Platform>) -> Self {
        self.inner = self.inner.set_platform(input);
        self
    }
    /// <p>The release version. For the latest available version, specify <code>LATEST</code>.</p>
    pub fn release_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.release_version(input.into());
        self
    }
    /// <p>The release version. For the latest available version, specify <code>LATEST</code>.</p>
    pub fn set_release_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_release_version(input);
        self
    }
}
