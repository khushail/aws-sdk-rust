// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_logging_options::_put_logging_options_output::PutLoggingOptionsOutputBuilder;

pub use crate::operation::put_logging_options::_put_logging_options_input::PutLoggingOptionsInputBuilder;

/// Fluent builder constructing a request to `PutLoggingOptions`.
///
/// <p>Sets or updates the IoT Analytics logging options.</p>
/// <p>If you update the value of any <code>loggingOptions</code> field, it takes up to one minute for the change to take effect. Also, if you change the policy attached to the role you specified in the <code>roleArn</code> field (for example, to correct an invalid policy), it takes up to five minutes for that change to take effect. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutLoggingOptionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_logging_options::builders::PutLoggingOptionsInputBuilder,
}
impl PutLoggingOptionsFluentBuilder {
    /// Creates a new `PutLoggingOptions`.
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
            crate::operation::put_logging_options::PutLoggingOptions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_logging_options::PutLoggingOptionsError,
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
        crate::operation::put_logging_options::PutLoggingOptionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_logging_options::PutLoggingOptionsError,
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
        crate::operation::put_logging_options::PutLoggingOptionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_logging_options::PutLoggingOptionsError,
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
            crate::operation::put_logging_options::PutLoggingOptions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_logging_options::PutLoggingOptionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The new values of the IoT Analytics logging options.</p>
    pub fn logging_options(mut self, input: crate::types::LoggingOptions) -> Self {
        self.inner = self.inner.logging_options(input);
        self
    }
    /// <p>The new values of the IoT Analytics logging options.</p>
    pub fn set_logging_options(
        mut self,
        input: ::std::option::Option<crate::types::LoggingOptions>,
    ) -> Self {
        self.inner = self.inner.set_logging_options(input);
        self
    }
}
