// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_contact_evaluation::_update_contact_evaluation_output::UpdateContactEvaluationOutputBuilder;

pub use crate::operation::update_contact_evaluation::_update_contact_evaluation_input::UpdateContactEvaluationInputBuilder;

/// Fluent builder constructing a request to `UpdateContactEvaluation`.
///
/// <p>Updates details about a contact evaluation in the specified Amazon Connect instance. A contact evaluation must be in draft state. Answers included in the request are merged with existing answers for the given evaluation. An answer or note can be deleted by passing an empty object (<code>{}</code>) to the question identifier. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateContactEvaluationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::update_contact_evaluation::builders::UpdateContactEvaluationInputBuilder,
}
impl UpdateContactEvaluationFluentBuilder {
    /// Creates a new `UpdateContactEvaluation`.
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
            crate::operation::update_contact_evaluation::UpdateContactEvaluation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_evaluation::UpdateContactEvaluationError,
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
        crate::operation::update_contact_evaluation::UpdateContactEvaluationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_evaluation::UpdateContactEvaluationError,
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
        crate::operation::update_contact_evaluation::UpdateContactEvaluationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_evaluation::UpdateContactEvaluationError,
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
            crate::operation::update_contact_evaluation::UpdateContactEvaluation,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_evaluation::UpdateContactEvaluationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>A unique identifier for the contact evaluation.</p>
    pub fn evaluation_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.evaluation_id(input.into());
        self
    }
    /// <p>A unique identifier for the contact evaluation.</p>
    pub fn set_evaluation_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_evaluation_id(input);
        self
    }
    /// Adds a key-value pair to `Answers`.
    ///
    /// To override the contents of this collection use [`set_answers`](Self::set_answers).
    ///
    /// <p>A map of question identifiers to answer value.</p>
    pub fn answers(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::EvaluationAnswerInput,
    ) -> Self {
        self.inner = self.inner.answers(k.into(), v);
        self
    }
    /// <p>A map of question identifiers to answer value.</p>
    pub fn set_answers(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::EvaluationAnswerInput>,
        >,
    ) -> Self {
        self.inner = self.inner.set_answers(input);
        self
    }
    /// Adds a key-value pair to `Notes`.
    ///
    /// To override the contents of this collection use [`set_notes`](Self::set_notes).
    ///
    /// <p>A map of question identifiers to note value.</p>
    pub fn notes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::EvaluationNote,
    ) -> Self {
        self.inner = self.inner.notes(k.into(), v);
        self
    }
    /// <p>A map of question identifiers to note value.</p>
    pub fn set_notes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::EvaluationNote>,
        >,
    ) -> Self {
        self.inner = self.inner.set_notes(input);
        self
    }
}
