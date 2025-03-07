// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_http_namespace::_update_http_namespace_output::UpdateHttpNamespaceOutputBuilder;

pub use crate::operation::update_http_namespace::_update_http_namespace_input::UpdateHttpNamespaceInputBuilder;

/// Fluent builder constructing a request to `UpdateHttpNamespace`.
///
/// <p>Updates an HTTP namespace.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateHttpNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_http_namespace::builders::UpdateHttpNamespaceInputBuilder,
}
impl UpdateHttpNamespaceFluentBuilder {
    /// Creates a new `UpdateHttpNamespace`.
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
            crate::operation::update_http_namespace::UpdateHttpNamespace,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_http_namespace::UpdateHttpNamespaceError,
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
        crate::operation::update_http_namespace::UpdateHttpNamespaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_http_namespace::UpdateHttpNamespaceError,
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
        crate::operation::update_http_namespace::UpdateHttpNamespaceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_http_namespace::UpdateHttpNamespaceError,
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
            crate::operation::update_http_namespace::UpdateHttpNamespace,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_http_namespace::UpdateHttpNamespaceError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the namespace that you want to update.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the namespace that you want to update.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>A unique string that identifies the request and that allows failed <code>UpdateHttpNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn updater_request_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.updater_request_id(input.into());
        self
    }
    /// <p>A unique string that identifies the request and that allows failed <code>UpdateHttpNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    pub fn set_updater_request_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_updater_request_id(input);
        self
    }
    /// <p>Updated properties for the the HTTP namespace.</p>
    pub fn namespace(mut self, input: crate::types::HttpNamespaceChange) -> Self {
        self.inner = self.inner.namespace(input);
        self
    }
    /// <p>Updated properties for the the HTTP namespace.</p>
    pub fn set_namespace(
        mut self,
        input: ::std::option::Option<crate::types::HttpNamespaceChange>,
    ) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
}
