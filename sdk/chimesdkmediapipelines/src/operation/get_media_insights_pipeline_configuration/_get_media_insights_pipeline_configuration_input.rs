// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetMediaInsightsPipelineConfigurationInput {
    /// <p>The unique identifier of the requested resource. Valid values include the name and ARN of the media insights pipeline configuration.</p>
    #[doc(hidden)]
    pub identifier: ::std::option::Option<::std::string::String>,
}
impl GetMediaInsightsPipelineConfigurationInput {
    /// <p>The unique identifier of the requested resource. Valid values include the name and ARN of the media insights pipeline configuration.</p>
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }
}
impl GetMediaInsightsPipelineConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetMediaInsightsPipelineConfigurationInput`](crate::operation::get_media_insights_pipeline_configuration::GetMediaInsightsPipelineConfigurationInput).
    pub fn builder() -> crate::operation::get_media_insights_pipeline_configuration::builders::GetMediaInsightsPipelineConfigurationInputBuilder{
        crate::operation::get_media_insights_pipeline_configuration::builders::GetMediaInsightsPipelineConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetMediaInsightsPipelineConfigurationInput`](crate::operation::get_media_insights_pipeline_configuration::GetMediaInsightsPipelineConfigurationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetMediaInsightsPipelineConfigurationInputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
}
impl GetMediaInsightsPipelineConfigurationInputBuilder {
    /// <p>The unique identifier of the requested resource. Valid values include the name and ARN of the media insights pipeline configuration.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the requested resource. Valid values include the name and ARN of the media insights pipeline configuration.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }
    /// Consumes the builder and constructs a [`GetMediaInsightsPipelineConfigurationInput`](crate::operation::get_media_insights_pipeline_configuration::GetMediaInsightsPipelineConfigurationInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_media_insights_pipeline_configuration::GetMediaInsightsPipelineConfigurationInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_media_insights_pipeline_configuration::GetMediaInsightsPipelineConfigurationInput {
                identifier: self.identifier
                ,
            }
        )
    }
}
