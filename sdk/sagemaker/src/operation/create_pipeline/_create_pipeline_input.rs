// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreatePipelineInput {
    /// <p>The name of the pipeline.</p>
    #[doc(hidden)]
    pub pipeline_name: ::std::option::Option<::std::string::String>,
    /// <p>The display name of the pipeline.</p>
    #[doc(hidden)]
    pub pipeline_display_name: ::std::option::Option<::std::string::String>,
    /// <p>The JSON pipeline definition of the pipeline.</p>
    #[doc(hidden)]
    pub pipeline_definition: ::std::option::Option<::std::string::String>,
    /// <p>The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location.</p>
    #[doc(hidden)]
    pub pipeline_definition_s3_location:
        ::std::option::Option<crate::types::PipelineDefinitionS3Location>,
    /// <p>A description of the pipeline.</p>
    #[doc(hidden)]
    pub pipeline_description: ::std::option::Option<::std::string::String>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time.</p>
    #[doc(hidden)]
    pub client_request_token: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the role used by the pipeline to access and create resources.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>A list of tags to apply to the created pipeline.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default.</p>
    #[doc(hidden)]
    pub parallelism_configuration: ::std::option::Option<crate::types::ParallelismConfiguration>,
}
impl CreatePipelineInput {
    /// <p>The name of the pipeline.</p>
    pub fn pipeline_name(&self) -> ::std::option::Option<&str> {
        self.pipeline_name.as_deref()
    }
    /// <p>The display name of the pipeline.</p>
    pub fn pipeline_display_name(&self) -> ::std::option::Option<&str> {
        self.pipeline_display_name.as_deref()
    }
    /// <p>The JSON pipeline definition of the pipeline.</p>
    pub fn pipeline_definition(&self) -> ::std::option::Option<&str> {
        self.pipeline_definition.as_deref()
    }
    /// <p>The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location.</p>
    pub fn pipeline_definition_s3_location(
        &self,
    ) -> ::std::option::Option<&crate::types::PipelineDefinitionS3Location> {
        self.pipeline_definition_s3_location.as_ref()
    }
    /// <p>A description of the pipeline.</p>
    pub fn pipeline_description(&self) -> ::std::option::Option<&str> {
        self.pipeline_description.as_deref()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the role used by the pipeline to access and create resources.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>A list of tags to apply to the created pipeline.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default.</p>
    pub fn parallelism_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ParallelismConfiguration> {
        self.parallelism_configuration.as_ref()
    }
}
impl CreatePipelineInput {
    /// Creates a new builder-style object to manufacture [`CreatePipelineInput`](crate::operation::create_pipeline::CreatePipelineInput).
    pub fn builder() -> crate::operation::create_pipeline::builders::CreatePipelineInputBuilder {
        crate::operation::create_pipeline::builders::CreatePipelineInputBuilder::default()
    }
}

/// A builder for [`CreatePipelineInput`](crate::operation::create_pipeline::CreatePipelineInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreatePipelineInputBuilder {
    pub(crate) pipeline_name: ::std::option::Option<::std::string::String>,
    pub(crate) pipeline_display_name: ::std::option::Option<::std::string::String>,
    pub(crate) pipeline_definition: ::std::option::Option<::std::string::String>,
    pub(crate) pipeline_definition_s3_location:
        ::std::option::Option<crate::types::PipelineDefinitionS3Location>,
    pub(crate) pipeline_description: ::std::option::Option<::std::string::String>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) parallelism_configuration:
        ::std::option::Option<crate::types::ParallelismConfiguration>,
}
impl CreatePipelineInputBuilder {
    /// <p>The name of the pipeline.</p>
    pub fn pipeline_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.pipeline_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the pipeline.</p>
    pub fn set_pipeline_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.pipeline_name = input;
        self
    }
    /// <p>The display name of the pipeline.</p>
    pub fn pipeline_display_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.pipeline_display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name of the pipeline.</p>
    pub fn set_pipeline_display_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.pipeline_display_name = input;
        self
    }
    /// <p>The JSON pipeline definition of the pipeline.</p>
    pub fn pipeline_definition(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.pipeline_definition = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The JSON pipeline definition of the pipeline.</p>
    pub fn set_pipeline_definition(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.pipeline_definition = input;
        self
    }
    /// <p>The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location.</p>
    pub fn pipeline_definition_s3_location(
        mut self,
        input: crate::types::PipelineDefinitionS3Location,
    ) -> Self {
        self.pipeline_definition_s3_location = ::std::option::Option::Some(input);
        self
    }
    /// <p>The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location.</p>
    pub fn set_pipeline_definition_s3_location(
        mut self,
        input: ::std::option::Option<crate::types::PipelineDefinitionS3Location>,
    ) -> Self {
        self.pipeline_definition_s3_location = input;
        self
    }
    /// <p>A description of the pipeline.</p>
    pub fn pipeline_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.pipeline_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the pipeline.</p>
    pub fn set_pipeline_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.pipeline_description = input;
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.client_request_token = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the role used by the pipeline to access and create resources.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the role used by the pipeline to access and create resources.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to apply to the created pipeline.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of tags to apply to the created pipeline.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default.</p>
    pub fn parallelism_configuration(
        mut self,
        input: crate::types::ParallelismConfiguration,
    ) -> Self {
        self.parallelism_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default.</p>
    pub fn set_parallelism_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ParallelismConfiguration>,
    ) -> Self {
        self.parallelism_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`CreatePipelineInput`](crate::operation::create_pipeline::CreatePipelineInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_pipeline::CreatePipelineInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_pipeline::CreatePipelineInput {
            pipeline_name: self.pipeline_name,
            pipeline_display_name: self.pipeline_display_name,
            pipeline_definition: self.pipeline_definition,
            pipeline_definition_s3_location: self.pipeline_definition_s3_location,
            pipeline_description: self.pipeline_description,
            client_request_token: self.client_request_token,
            role_arn: self.role_arn,
            tags: self.tags,
            parallelism_configuration: self.parallelism_configuration,
        })
    }
}
