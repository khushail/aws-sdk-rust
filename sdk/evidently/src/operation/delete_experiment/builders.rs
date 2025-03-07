// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_experiment::_delete_experiment_output::DeleteExperimentOutputBuilder;

pub use crate::operation::delete_experiment::_delete_experiment_input::DeleteExperimentInputBuilder;

/// Fluent builder constructing a request to `DeleteExperiment`.
///
/// <p>Deletes an Evidently experiment. The feature used for the experiment is not deleted.</p>
/// <p>To stop an experiment without deleting it, use <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_StopExperiment.html">StopExperiment</a>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteExperimentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_experiment::builders::DeleteExperimentInputBuilder,
}
impl DeleteExperimentFluentBuilder {
    /// Creates a new `DeleteExperiment`.
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
            crate::operation::delete_experiment::DeleteExperiment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_experiment::DeleteExperimentError,
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
        crate::operation::delete_experiment::DeleteExperimentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_experiment::DeleteExperimentError,
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
        crate::operation::delete_experiment::DeleteExperimentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_experiment::DeleteExperimentError,
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
            crate::operation::delete_experiment::DeleteExperiment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_experiment::DeleteExperimentError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name or ARN of the project that contains the experiment to delete.</p>
    pub fn project(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project(input.into());
        self
    }
    /// <p>The name or ARN of the project that contains the experiment to delete.</p>
    pub fn set_project(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project(input);
        self
    }
    /// <p>The name of the experiment to delete.</p>
    pub fn experiment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.experiment(input.into());
        self
    }
    /// <p>The name of the experiment to delete.</p>
    pub fn set_experiment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_experiment(input);
        self
    }
}
