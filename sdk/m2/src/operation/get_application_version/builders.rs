// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_application_version::_get_application_version_output::GetApplicationVersionOutputBuilder;

pub use crate::operation::get_application_version::_get_application_version_input::GetApplicationVersionInputBuilder;

/// Fluent builder constructing a request to `GetApplicationVersion`.
///
/// <p>Returns details about a specific version of a specific application.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetApplicationVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_application_version::builders::GetApplicationVersionInputBuilder,
}
impl GetApplicationVersionFluentBuilder {
    /// Creates a new `GetApplicationVersion`.
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
            crate::operation::get_application_version::GetApplicationVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_application_version::GetApplicationVersionError,
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
        crate::operation::get_application_version::GetApplicationVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_application_version::GetApplicationVersionError,
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
        crate::operation::get_application_version::GetApplicationVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_application_version::GetApplicationVersionError,
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
            crate::operation::get_application_version::GetApplicationVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_application_version::GetApplicationVersionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier of the application.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier of the application.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The specific version of the application.</p>
    pub fn application_version(mut self, input: i32) -> Self {
        self.inner = self.inner.application_version(input);
        self
    }
    /// <p>The specific version of the application.</p>
    pub fn set_application_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_application_version(input);
        self
    }
}
