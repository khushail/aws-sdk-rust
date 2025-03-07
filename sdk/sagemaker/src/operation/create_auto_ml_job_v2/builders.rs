// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_auto_ml_job_v2::_create_auto_ml_job_v2_output::CreateAutoMlJobV2OutputBuilder;

pub use crate::operation::create_auto_ml_job_v2::_create_auto_ml_job_v2_input::CreateAutoMlJobV2InputBuilder;

/// Fluent builder constructing a request to `CreateAutoMLJobV2`.
///
/// <p>Creates an Amazon SageMaker AutoML job that uses non-tabular data such as images or text for Computer Vision or Natural Language Processing problems.</p>
/// <p>Find the resulting model after you run an AutoML job V2 by calling <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_DescribeAutoMLJobV2.html">DescribeAutoMLJobV2</a>.</p>
/// <p>To create an <code>AutoMLJob</code> using tabular data, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJob.html">CreateAutoMLJob</a>.</p> <note>
/// <p>This API action is callable through SageMaker Canvas only. Calling it directly from the CLI or an SDK results in an error.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAutoMLJobV2FluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_auto_ml_job_v2::builders::CreateAutoMlJobV2InputBuilder,
}
impl CreateAutoMLJobV2FluentBuilder {
    /// Creates a new `CreateAutoMLJobV2`.
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
            crate::operation::create_auto_ml_job_v2::CreateAutoMLJobV2,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_auto_ml_job_v2::CreateAutoMLJobV2Error,
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
        crate::operation::create_auto_ml_job_v2::CreateAutoMlJobV2Output,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_auto_ml_job_v2::CreateAutoMLJobV2Error,
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
        crate::operation::create_auto_ml_job_v2::CreateAutoMlJobV2Output,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_auto_ml_job_v2::CreateAutoMLJobV2Error,
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
            crate::operation::create_auto_ml_job_v2::CreateAutoMLJobV2,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_auto_ml_job_v2::CreateAutoMLJobV2Error,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p>
    pub fn auto_ml_job_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.auto_ml_job_name(input.into());
        self
    }
    /// <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p>
    pub fn set_auto_ml_job_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_auto_ml_job_name(input);
        self
    }
    /// Appends an item to `AutoMLJobInputDataConfig`.
    ///
    /// To override the contents of this collection use [`set_auto_ml_job_input_data_config`](Self::set_auto_ml_job_input_data_config).
    ///
    /// <p>An array of channel objects describing the input data and their location. Each channel is a named input source. Similar to <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJob.html#sagemaker-CreateAutoMLJob-request-InputDataConfig">InputDataConfig</a> supported by <code>CreateAutoMLJob</code>. The supported formats depend on the problem type:</p>
    /// <ul>
    /// <li> <p>ImageClassification: S3Prefix, <code>ManifestFile</code>, <code>AugmentedManifestFile</code> </p> </li>
    /// <li> <p>TextClassification: S3Prefix</p> </li>
    /// </ul>
    pub fn auto_ml_job_input_data_config(mut self, input: crate::types::AutoMlJobChannel) -> Self {
        self.inner = self.inner.auto_ml_job_input_data_config(input);
        self
    }
    /// <p>An array of channel objects describing the input data and their location. Each channel is a named input source. Similar to <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJob.html#sagemaker-CreateAutoMLJob-request-InputDataConfig">InputDataConfig</a> supported by <code>CreateAutoMLJob</code>. The supported formats depend on the problem type:</p>
    /// <ul>
    /// <li> <p>ImageClassification: S3Prefix, <code>ManifestFile</code>, <code>AugmentedManifestFile</code> </p> </li>
    /// <li> <p>TextClassification: S3Prefix</p> </li>
    /// </ul>
    pub fn set_auto_ml_job_input_data_config(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AutoMlJobChannel>>,
    ) -> Self {
        self.inner = self.inner.set_auto_ml_job_input_data_config(input);
        self
    }
    /// <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job.</p>
    pub fn output_data_config(mut self, input: crate::types::AutoMlOutputDataConfig) -> Self {
        self.inner = self.inner.output_data_config(input);
        self
    }
    /// <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job.</p>
    pub fn set_output_data_config(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlOutputDataConfig>,
    ) -> Self {
        self.inner = self.inner.set_output_data_config(input);
        self
    }
    /// <p>Defines the configuration settings of one of the supported problem types.</p>
    pub fn auto_ml_problem_type_config(
        mut self,
        input: crate::types::AutoMlProblemTypeConfig,
    ) -> Self {
        self.inner = self.inner.auto_ml_problem_type_config(input);
        self
    }
    /// <p>Defines the configuration settings of one of the supported problem types.</p>
    pub fn set_auto_ml_problem_type_config(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlProblemTypeConfig>,
    ) -> Self {
        self.inner = self.inner.set_auto_ml_problem_type_config(input);
        self
    }
    /// <p>The ARN of the role that is used to access the data.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the role that is used to access the data.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, such as by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web ServicesResources</a>. Tag keys must be unique per resource.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, such as by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web ServicesResources</a>. Tag keys must be unique per resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The security configuration for traffic encryption or Amazon VPC settings.</p>
    pub fn security_config(mut self, input: crate::types::AutoMlSecurityConfig) -> Self {
        self.inner = self.inner.security_config(input);
        self
    }
    /// <p>The security configuration for traffic encryption or Amazon VPC settings.</p>
    pub fn set_security_config(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlSecurityConfig>,
    ) -> Self {
        self.inner = self.inner.set_security_config(input);
        self
    }
    /// <p>Specifies a metric to minimize or maximize as the objective of a job. For <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJobV2.html">CreateAutoMLJobV2</a>, only <code>Accuracy</code> is supported.</p>
    pub fn auto_ml_job_objective(mut self, input: crate::types::AutoMlJobObjective) -> Self {
        self.inner = self.inner.auto_ml_job_objective(input);
        self
    }
    /// <p>Specifies a metric to minimize or maximize as the objective of a job. For <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJobV2.html">CreateAutoMLJobV2</a>, only <code>Accuracy</code> is supported.</p>
    pub fn set_auto_ml_job_objective(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlJobObjective>,
    ) -> Self {
        self.inner = self.inner.set_auto_ml_job_objective(input);
        self
    }
    /// <p>Specifies how to generate the endpoint name for an automatic one-click Autopilot model deployment.</p>
    pub fn model_deploy_config(mut self, input: crate::types::ModelDeployConfig) -> Self {
        self.inner = self.inner.model_deploy_config(input);
        self
    }
    /// <p>Specifies how to generate the endpoint name for an automatic one-click Autopilot model deployment.</p>
    pub fn set_model_deploy_config(
        mut self,
        input: ::std::option::Option<crate::types::ModelDeployConfig>,
    ) -> Self {
        self.inner = self.inner.set_model_deploy_config(input);
        self
    }
    /// <p>This structure specifies how to split the data into train and validation datasets.</p>
    /// <p>If you are using the V1 API (for example <code>CreateAutoMLJob</code>) or the V2 API for Natural Language Processing problems (for example <code>CreateAutoMLJobV2</code> with a <code>TextClassificationJobConfig</code> problem type), the validation and training datasets must contain the same headers. Also, for V1 API jobs, the validation dataset must be less than 2 GB in size.</p>
    pub fn data_split_config(mut self, input: crate::types::AutoMlDataSplitConfig) -> Self {
        self.inner = self.inner.data_split_config(input);
        self
    }
    /// <p>This structure specifies how to split the data into train and validation datasets.</p>
    /// <p>If you are using the V1 API (for example <code>CreateAutoMLJob</code>) or the V2 API for Natural Language Processing problems (for example <code>CreateAutoMLJobV2</code> with a <code>TextClassificationJobConfig</code> problem type), the validation and training datasets must contain the same headers. Also, for V1 API jobs, the validation dataset must be less than 2 GB in size.</p>
    pub fn set_data_split_config(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlDataSplitConfig>,
    ) -> Self {
        self.inner = self.inner.set_data_split_config(input);
        self
    }
}
