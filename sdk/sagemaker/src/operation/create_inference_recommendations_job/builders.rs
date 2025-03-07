// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_inference_recommendations_job::_create_inference_recommendations_job_output::CreateInferenceRecommendationsJobOutputBuilder;

pub use crate::operation::create_inference_recommendations_job::_create_inference_recommendations_job_input::CreateInferenceRecommendationsJobInputBuilder;

/// Fluent builder constructing a request to `CreateInferenceRecommendationsJob`.
///
/// <p>Starts a recommendation job. You can create either an instance recommendation or load test job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateInferenceRecommendationsJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_inference_recommendations_job::builders::CreateInferenceRecommendationsJobInputBuilder,
}
impl CreateInferenceRecommendationsJobFluentBuilder {
    /// Creates a new `CreateInferenceRecommendationsJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJob, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJobError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJobOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJobError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJobOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJobError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJob, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::create_inference_recommendations_job::CreateInferenceRecommendationsJobError>
    >{
        self.customize_middleware().await
    }
    /// <p>A name for the recommendation job. The name must be unique within the Amazon Web Services Region and within your Amazon Web Services account. The job name is passed down to the resources created by the recommendation job. The names of resources (such as the model, endpoint configuration, endpoint, and compilation) that are prefixed with the job name are truncated at 40 characters.</p>
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_name(input.into());
        self
    }
    /// <p>A name for the recommendation job. The name must be unique within the Amazon Web Services Region and within your Amazon Web Services account. The job name is passed down to the resources created by the recommendation job. The names of resources (such as the model, endpoint configuration, endpoint, and compilation) that are prefixed with the job name are truncated at 40 characters.</p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_name(input);
        self
    }
    /// <p>Defines the type of recommendation job. Specify <code>Default</code> to initiate an instance recommendation and <code>Advanced</code> to initiate a load test. If left unspecified, Amazon SageMaker Inference Recommender will run an instance recommendation (<code>DEFAULT</code>) job.</p>
    pub fn job_type(mut self, input: crate::types::RecommendationJobType) -> Self {
        self.inner = self.inner.job_type(input);
        self
    }
    /// <p>Defines the type of recommendation job. Specify <code>Default</code> to initiate an instance recommendation and <code>Advanced</code> to initiate a load test. If left unspecified, Amazon SageMaker Inference Recommender will run an instance recommendation (<code>DEFAULT</code>) job.</p>
    pub fn set_job_type(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationJobType>,
    ) -> Self {
        self.inner = self.inner.set_job_type(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on your behalf.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on your behalf.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>Provides information about the versioned model package Amazon Resource Name (ARN), the traffic pattern, and endpoint configurations.</p>
    pub fn input_config(mut self, input: crate::types::RecommendationJobInputConfig) -> Self {
        self.inner = self.inner.input_config(input);
        self
    }
    /// <p>Provides information about the versioned model package Amazon Resource Name (ARN), the traffic pattern, and endpoint configurations.</p>
    pub fn set_input_config(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationJobInputConfig>,
    ) -> Self {
        self.inner = self.inner.set_input_config(input);
        self
    }
    /// <p>Description of the recommendation job.</p>
    pub fn job_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.job_description(input.into());
        self
    }
    /// <p>Description of the recommendation job.</p>
    pub fn set_job_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_job_description(input);
        self
    }
    /// <p>A set of conditions for stopping a recommendation job. If any of the conditions are met, the job is automatically stopped.</p>
    pub fn stopping_conditions(
        mut self,
        input: crate::types::RecommendationJobStoppingConditions,
    ) -> Self {
        self.inner = self.inner.stopping_conditions(input);
        self
    }
    /// <p>A set of conditions for stopping a recommendation job. If any of the conditions are met, the job is automatically stopped.</p>
    pub fn set_stopping_conditions(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationJobStoppingConditions>,
    ) -> Self {
        self.inner = self.inner.set_stopping_conditions(input);
        self
    }
    /// <p>Provides information about the output artifacts and the KMS key to use for Amazon S3 server-side encryption.</p>
    pub fn output_config(mut self, input: crate::types::RecommendationJobOutputConfig) -> Self {
        self.inner = self.inner.output_config(input);
        self
    }
    /// <p>Provides information about the output artifacts and the KMS key to use for Amazon S3 server-side encryption.</p>
    pub fn set_output_config(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationJobOutputConfig>,
    ) -> Self {
        self.inner = self.inner.set_output_config(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The metadata that you apply to Amazon Web Services resources to help you categorize and organize them. Each tag consists of a key and a value, both of which you define. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> in the Amazon Web Services General Reference.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The metadata that you apply to Amazon Web Services resources to help you categorize and organize them. Each tag consists of a key and a value, both of which you define. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> in the Amazon Web Services General Reference.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
