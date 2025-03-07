// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::detect_syntax::_detect_syntax_output::DetectSyntaxOutputBuilder;

pub use crate::operation::detect_syntax::_detect_syntax_input::DetectSyntaxInputBuilder;

/// Fluent builder constructing a request to `DetectSyntax`.
///
/// <p>Inspects text for syntax and the part of speech of words in the document. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/how-syntax.html">Syntax</a> in the Comprehend Developer Guide. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DetectSyntaxFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::detect_syntax::builders::DetectSyntaxInputBuilder,
}
impl DetectSyntaxFluentBuilder {
    /// Creates a new `DetectSyntax`.
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
            crate::operation::detect_syntax::DetectSyntax,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::detect_syntax::DetectSyntaxError>,
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
        crate::operation::detect_syntax::DetectSyntaxOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::detect_syntax::DetectSyntaxError>,
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
        crate::operation::detect_syntax::DetectSyntaxOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::detect_syntax::DetectSyntaxError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::detect_syntax::DetectSyntax,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::detect_syntax::DetectSyntaxError>,
    > {
        self.customize_middleware().await
    }
    /// <p>A UTF-8 string. The maximum string size is 5 KB.</p>
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.text(input.into());
        self
    }
    /// <p>A UTF-8 string. The maximum string size is 5 KB.</p>
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_text(input);
        self
    }
    /// <p>The language code of the input documents. You can specify any of the following languages supported by Amazon Comprehend: German ("de"), English ("en"), Spanish ("es"), French ("fr"), Italian ("it"), or Portuguese ("pt").</p>
    pub fn language_code(mut self, input: crate::types::SyntaxLanguageCode) -> Self {
        self.inner = self.inner.language_code(input);
        self
    }
    /// <p>The language code of the input documents. You can specify any of the following languages supported by Amazon Comprehend: German ("de"), English ("en"), Spanish ("es"), French ("fr"), Italian ("it"), or Portuguese ("pt").</p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<crate::types::SyntaxLanguageCode>,
    ) -> Self {
        self.inner = self.inner.set_language_code(input);
        self
    }
}
