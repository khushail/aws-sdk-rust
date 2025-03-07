// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutVoiceConnectorTermination`](crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl ::std::convert::Into<String>)`](crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationFluentBuilder::set_voice_connector_id): <p>The Amazon Chime Voice Connector ID.</p>
    ///   - [`termination(Termination)`](crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationFluentBuilder::termination) / [`set_termination(Option<Termination>)`](crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationFluentBuilder::set_termination): <p>The termination setting details to add.</p>
    /// - On success, responds with [`PutVoiceConnectorTerminationOutput`](crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationOutput) with field(s):
    ///   - [`termination(Option<Termination>)`](crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationOutput::termination): <p>The updated termination setting details.</p>
    /// - On failure, responds with [`SdkError<PutVoiceConnectorTerminationError>`](crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationError)
    pub fn put_voice_connector_termination(&self) -> crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationFluentBuilder{
        crate::operation::put_voice_connector_termination::builders::PutVoiceConnectorTerminationFluentBuilder::new(self.handle.clone())
    }
}
