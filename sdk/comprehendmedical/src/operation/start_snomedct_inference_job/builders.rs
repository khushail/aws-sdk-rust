// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_snomedct_inference_job::_start_snomedct_inference_job_output::StartSnomedctInferenceJobOutputBuilder;

pub use crate::operation::start_snomedct_inference_job::_start_snomedct_inference_job_input::StartSnomedctInferenceJobInputBuilder;

/// Fluent builder constructing a request to `StartSNOMEDCTInferenceJob`.
///
/// <p> Starts an asynchronous job to detect medical concepts and link them to the SNOMED-CT ontology. Use the DescribeSNOMEDCTInferenceJob operation to track the status of a job. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartSNOMEDCTInferenceJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::start_snomedct_inference_job::builders::StartSnomedctInferenceJobInputBuilder,
}
impl StartSNOMEDCTInferenceJobFluentBuilder {
    /// Creates a new `StartSNOMEDCTInferenceJob`.
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
            crate::operation::start_snomedct_inference_job::StartSNOMEDCTInferenceJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_snomedct_inference_job::StartSNOMEDCTInferenceJobError,
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
        crate::operation::start_snomedct_inference_job::StartSnomedctInferenceJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_snomedct_inference_job::StartSNOMEDCTInferenceJobError,
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
        crate::operation::start_snomedct_inference_job::StartSnomedctInferenceJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_snomedct_inference_job::StartSNOMEDCTInferenceJobError,
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
            crate::operation::start_snomedct_inference_job::StartSNOMEDCTInferenceJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_snomedct_inference_job::StartSNOMEDCTInferenceJobError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The input properties for an entities detection job. This includes the name of the S3 bucket and the path to the files to be analyzed. </p>
    pub fn input_data_config(mut self, input: crate::types::InputDataConfig) -> Self {
        self.inner = self.inner.input_data_config(input);
        self
    }
    /// <p>The input properties for an entities detection job. This includes the name of the S3 bucket and the path to the files to be analyzed. </p>
    pub fn set_input_data_config(
        mut self,
        input: ::std::option::Option<crate::types::InputDataConfig>,
    ) -> Self {
        self.inner = self.inner.set_input_data_config(input);
        self
    }
    /// <p>The output properties for a detection job.</p>
    pub fn output_data_config(mut self, input: crate::types::OutputDataConfig) -> Self {
        self.inner = self.inner.output_data_config(input);
        self
    }
    /// <p>The output properties for a detection job.</p>
    pub fn set_output_data_config(
        mut self,
        input: ::std::option::Option<crate::types::OutputDataConfig>,
    ) -> Self {
        self.inner = self.inner.set_output_data_config(input);
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. </p>
    pub fn data_access_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.data_access_role_arn(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. </p>
    pub fn set_data_access_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_data_access_role_arn(input);
        self
    }
    /// <p> The user generated name the asynchronous InferSNOMEDCT job. </p>
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_name(input.into());
        self
    }
    /// <p> The user generated name the asynchronous InferSNOMEDCT job. </p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_name(input);
        self
    }
    /// <p> A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one. </p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p> A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one. </p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p> An AWS Key Management Service key used to encrypt your output files. If you do not specify a key, the files are written in plain text. </p>
    pub fn kms_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key(input.into());
        self
    }
    /// <p> An AWS Key Management Service key used to encrypt your output files. If you do not specify a key, the files are written in plain text. </p>
    pub fn set_kms_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key(input);
        self
    }
    /// <p> The language of the input documents. All documents must be in the same language. </p>
    pub fn language_code(mut self, input: crate::types::LanguageCode) -> Self {
        self.inner = self.inner.language_code(input);
        self
    }
    /// <p> The language of the input documents. All documents must be in the same language. </p>
    pub fn set_language_code(
        mut self,
        input: ::std::option::Option<crate::types::LanguageCode>,
    ) -> Self {
        self.inner = self.inner.set_language_code(input);
        self
    }
}
