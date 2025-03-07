// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_api_keys::_import_api_keys_output::ImportApiKeysOutputBuilder;

pub use crate::operation::import_api_keys::_import_api_keys_input::ImportApiKeysInputBuilder;

/// Fluent builder constructing a request to `ImportApiKeys`.
///
/// <p>Import API keys from an external source, such as a CSV-formatted file.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportApiKeysFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_api_keys::builders::ImportApiKeysInputBuilder,
}
impl ImportApiKeysFluentBuilder {
    /// Creates a new `ImportApiKeys`.
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
            crate::operation::import_api_keys::ImportApiKeys,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::import_api_keys::ImportApiKeysError>,
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
        crate::operation::import_api_keys::ImportApiKeysOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::import_api_keys::ImportApiKeysError>,
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
        crate::operation::import_api_keys::ImportApiKeysOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::import_api_keys::ImportApiKeysError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::import_api_keys::ImportApiKeys,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::import_api_keys::ImportApiKeysError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The payload of the POST request to import API keys. For the payload format, see API Key File Format.</p>
    pub fn body(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.body(input);
        self
    }
    /// <p>The payload of the POST request to import API keys. For the payload format, see API Key File Format.</p>
    pub fn set_body(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_body(input);
        self
    }
    /// <p>A query parameter to specify the input format to imported API keys. Currently, only the <code>csv</code> format is supported.</p>
    pub fn format(mut self, input: crate::types::ApiKeysFormat) -> Self {
        self.inner = self.inner.format(input);
        self
    }
    /// <p>A query parameter to specify the input format to imported API keys. Currently, only the <code>csv</code> format is supported.</p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::ApiKeysFormat>) -> Self {
        self.inner = self.inner.set_format(input);
        self
    }
    /// <p>A query parameter to indicate whether to rollback ApiKey importation (<code>true</code>) or not (<code>false</code>) when error is encountered.</p>
    pub fn fail_on_warnings(mut self, input: bool) -> Self {
        self.inner = self.inner.fail_on_warnings(input);
        self
    }
    /// <p>A query parameter to indicate whether to rollback ApiKey importation (<code>true</code>) or not (<code>false</code>) when error is encountered.</p>
    pub fn set_fail_on_warnings(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_fail_on_warnings(input);
        self
    }
}
