// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopVoiceToneAnalysisTaskInput {
    /// <p>The Voice Connector ID.</p>
    #[doc(hidden)]
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the voice tone analysis task.</p>
    #[doc(hidden)]
    pub voice_tone_analysis_task_id: ::std::option::Option<::std::string::String>,
}
impl StopVoiceToneAnalysisTaskInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
    /// <p>The ID of the voice tone analysis task.</p>
    pub fn voice_tone_analysis_task_id(&self) -> ::std::option::Option<&str> {
        self.voice_tone_analysis_task_id.as_deref()
    }
}
impl StopVoiceToneAnalysisTaskInput {
    /// Creates a new builder-style object to manufacture [`StopVoiceToneAnalysisTaskInput`](crate::operation::stop_voice_tone_analysis_task::StopVoiceToneAnalysisTaskInput).
    pub fn builder() -> crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskInputBuilder{
        crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskInputBuilder::default()
    }
}

/// A builder for [`StopVoiceToneAnalysisTaskInput`](crate::operation::stop_voice_tone_analysis_task::StopVoiceToneAnalysisTaskInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopVoiceToneAnalysisTaskInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) voice_tone_analysis_task_id: ::std::option::Option<::std::string::String>,
}
impl StopVoiceToneAnalysisTaskInputBuilder {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn set_voice_connector_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = input;
        self
    }
    /// <p>The ID of the voice tone analysis task.</p>
    pub fn voice_tone_analysis_task_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_tone_analysis_task_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the voice tone analysis task.</p>
    pub fn set_voice_tone_analysis_task_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_tone_analysis_task_id = input;
        self
    }
    /// Consumes the builder and constructs a [`StopVoiceToneAnalysisTaskInput`](crate::operation::stop_voice_tone_analysis_task::StopVoiceToneAnalysisTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::stop_voice_tone_analysis_task::StopVoiceToneAnalysisTaskInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::stop_voice_tone_analysis_task::StopVoiceToneAnalysisTaskInput {
                voice_connector_id: self.voice_connector_id,
                voice_tone_analysis_task_id: self.voice_tone_analysis_task_id,
            },
        )
    }
}
