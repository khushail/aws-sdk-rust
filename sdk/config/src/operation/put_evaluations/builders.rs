// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_evaluations::_put_evaluations_output::PutEvaluationsOutputBuilder;

pub use crate::operation::put_evaluations::_put_evaluations_input::PutEvaluationsInputBuilder;

/// Fluent builder constructing a request to `PutEvaluations`.
///
/// <p>Used by an Lambda function to deliver evaluation results to Config. This action is required in every Lambda function that is invoked by an Config rule.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutEvaluationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_evaluations::builders::PutEvaluationsInputBuilder,
}
impl PutEvaluationsFluentBuilder {
    /// Creates a new `PutEvaluations`.
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
            crate::operation::put_evaluations::PutEvaluations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::put_evaluations::PutEvaluationsError>,
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
        crate::operation::put_evaluations::PutEvaluationsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::put_evaluations::PutEvaluationsError>,
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
        crate::operation::put_evaluations::PutEvaluationsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::put_evaluations::PutEvaluationsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::put_evaluations::PutEvaluations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::put_evaluations::PutEvaluationsError>,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `Evaluations`.
    ///
    /// To override the contents of this collection use [`set_evaluations`](Self::set_evaluations).
    ///
    /// <p>The assessments that the Lambda function performs. Each evaluation identifies an Amazon Web Services resource and indicates whether it complies with the Config rule that invokes the Lambda function.</p>
    pub fn evaluations(mut self, input: crate::types::Evaluation) -> Self {
        self.inner = self.inner.evaluations(input);
        self
    }
    /// <p>The assessments that the Lambda function performs. Each evaluation identifies an Amazon Web Services resource and indicates whether it complies with the Config rule that invokes the Lambda function.</p>
    pub fn set_evaluations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Evaluation>>,
    ) -> Self {
        self.inner = self.inner.set_evaluations(input);
        self
    }
    /// <p>An encrypted token that associates an evaluation with an Config rule. Identifies the rule and the event that triggered the evaluation.</p>
    pub fn result_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.result_token(input.into());
        self
    }
    /// <p>An encrypted token that associates an evaluation with an Config rule. Identifies the rule and the event that triggered the evaluation.</p>
    pub fn set_result_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_result_token(input);
        self
    }
    /// <p>Use this parameter to specify a test run for <code>PutEvaluations</code>. You can verify whether your Lambda function will deliver evaluation results to Config. No updates occur to your existing evaluations, and evaluation results are not sent to Config.</p> <note>
    /// <p>When <code>TestMode</code> is <code>true</code>, <code>PutEvaluations</code> doesn't require a valid value for the <code>ResultToken</code> parameter, but the value cannot be null.</p>
    /// </note>
    pub fn test_mode(mut self, input: bool) -> Self {
        self.inner = self.inner.test_mode(input);
        self
    }
    /// <p>Use this parameter to specify a test run for <code>PutEvaluations</code>. You can verify whether your Lambda function will deliver evaluation results to Config. No updates occur to your existing evaluations, and evaluation results are not sent to Config.</p> <note>
    /// <p>When <code>TestMode</code> is <code>true</code>, <code>PutEvaluations</code> doesn't require a valid value for the <code>ResultToken</code> parameter, but the value cannot be null.</p>
    /// </note>
    pub fn set_test_mode(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_test_mode(input);
        self
    }
}
