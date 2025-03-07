// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_pipeline::_update_pipeline_output::UpdatePipelineOutputBuilder;

pub use crate::operation::update_pipeline::_update_pipeline_input::UpdatePipelineInputBuilder;

/// Fluent builder constructing a request to `UpdatePipeline`.
///
/// <p>Updates a pipeline.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePipelineFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_pipeline::builders::UpdatePipelineInputBuilder,
}
impl UpdatePipelineFluentBuilder {
    /// Creates a new `UpdatePipeline`.
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
            crate::operation::update_pipeline::UpdatePipeline,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_pipeline::UpdatePipelineError>,
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
        crate::operation::update_pipeline::UpdatePipelineOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_pipeline::UpdatePipelineError>,
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
        crate::operation::update_pipeline::UpdatePipelineOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_pipeline::UpdatePipelineError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_pipeline::UpdatePipeline,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_pipeline::UpdatePipelineError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the pipeline to update.</p>
    pub fn pipeline_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pipeline_name(input.into());
        self
    }
    /// <p>The name of the pipeline to update.</p>
    pub fn set_pipeline_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_name(input);
        self
    }
    /// <p>The display name of the pipeline.</p>
    pub fn pipeline_display_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pipeline_display_name(input.into());
        self
    }
    /// <p>The display name of the pipeline.</p>
    pub fn set_pipeline_display_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_display_name(input);
        self
    }
    /// <p>The JSON pipeline definition.</p>
    pub fn pipeline_definition(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pipeline_definition(input.into());
        self
    }
    /// <p>The JSON pipeline definition.</p>
    pub fn set_pipeline_definition(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_definition(input);
        self
    }
    /// <p>The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location.</p>
    pub fn pipeline_definition_s3_location(
        mut self,
        input: crate::types::PipelineDefinitionS3Location,
    ) -> Self {
        self.inner = self.inner.pipeline_definition_s3_location(input);
        self
    }
    /// <p>The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location.</p>
    pub fn set_pipeline_definition_s3_location(
        mut self,
        input: ::std::option::Option<crate::types::PipelineDefinitionS3Location>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_definition_s3_location(input);
        self
    }
    /// <p>The description of the pipeline.</p>
    pub fn pipeline_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pipeline_description(input.into());
        self
    }
    /// <p>The description of the pipeline.</p>
    pub fn set_pipeline_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_description(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) that the pipeline uses to execute.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that the pipeline uses to execute.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>If specified, it applies to all executions of this pipeline by default.</p>
    pub fn parallelism_configuration(
        mut self,
        input: crate::types::ParallelismConfiguration,
    ) -> Self {
        self.inner = self.inner.parallelism_configuration(input);
        self
    }
    /// <p>If specified, it applies to all executions of this pipeline by default.</p>
    pub fn set_parallelism_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ParallelismConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_parallelism_configuration(input);
        self
    }
}
