// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_medical_vocabulary::_get_medical_vocabulary_output::GetMedicalVocabularyOutputBuilder;

pub use crate::operation::get_medical_vocabulary::_get_medical_vocabulary_input::GetMedicalVocabularyInputBuilder;

/// Fluent builder constructing a request to `GetMedicalVocabulary`.
///
/// <p>Provides information about the specified custom medical vocabulary.</p>
/// <p>To view the status of the specified custom medical vocabulary, check the <code>VocabularyState</code> field. If the status is <code>READY</code>, your custom vocabulary is available to use. If the status is <code>FAILED</code>, <code>FailureReason</code> provides details on why your vocabulary failed.</p>
/// <p>To get a list of your custom medical vocabularies, use the operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMedicalVocabularyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_medical_vocabulary::builders::GetMedicalVocabularyInputBuilder,
}
impl GetMedicalVocabularyFluentBuilder {
    /// Creates a new `GetMedicalVocabulary`.
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
            crate::operation::get_medical_vocabulary::GetMedicalVocabulary,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_medical_vocabulary::GetMedicalVocabularyError,
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
        crate::operation::get_medical_vocabulary::GetMedicalVocabularyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_medical_vocabulary::GetMedicalVocabularyError,
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
        crate::operation::get_medical_vocabulary::GetMedicalVocabularyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_medical_vocabulary::GetMedicalVocabularyError,
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
            crate::operation::get_medical_vocabulary::GetMedicalVocabulary,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_medical_vocabulary::GetMedicalVocabularyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the custom medical vocabulary you want information about. Custom medical vocabulary names are case sensitive.</p>
    pub fn vocabulary_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vocabulary_name(input.into());
        self
    }
    /// <p>The name of the custom medical vocabulary you want information about. Custom medical vocabulary names are case sensitive.</p>
    pub fn set_vocabulary_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vocabulary_name(input);
        self
    }
}
