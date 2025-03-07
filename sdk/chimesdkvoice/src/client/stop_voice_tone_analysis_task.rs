// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopVoiceToneAnalysisTask`](crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl ::std::convert::Into<String>)`](crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskFluentBuilder::set_voice_connector_id): <p>The Voice Connector ID.</p>
    ///   - [`voice_tone_analysis_task_id(impl ::std::convert::Into<String>)`](crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskFluentBuilder::voice_tone_analysis_task_id) / [`set_voice_tone_analysis_task_id(Option<String>)`](crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskFluentBuilder::set_voice_tone_analysis_task_id): <p>The ID of the voice tone analysis task.</p>
    /// - On success, responds with [`StopVoiceToneAnalysisTaskOutput`](crate::operation::stop_voice_tone_analysis_task::StopVoiceToneAnalysisTaskOutput)
    /// - On failure, responds with [`SdkError<StopVoiceToneAnalysisTaskError>`](crate::operation::stop_voice_tone_analysis_task::StopVoiceToneAnalysisTaskError)
    pub fn stop_voice_tone_analysis_task(&self) -> crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskFluentBuilder{
        crate::operation::stop_voice_tone_analysis_task::builders::StopVoiceToneAnalysisTaskFluentBuilder::new(self.handle.clone())
    }
}
