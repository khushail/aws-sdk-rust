// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVoiceConnectorGroup`](crate::operation::delete_voice_connector_group::builders::DeleteVoiceConnectorGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_group_id(impl ::std::convert::Into<String>)`](crate::operation::delete_voice_connector_group::builders::DeleteVoiceConnectorGroupFluentBuilder::voice_connector_group_id) / [`set_voice_connector_group_id(Option<String>)`](crate::operation::delete_voice_connector_group::builders::DeleteVoiceConnectorGroupFluentBuilder::set_voice_connector_group_id): <p>The Amazon Chime Voice Connector group ID.</p>
    /// - On success, responds with [`DeleteVoiceConnectorGroupOutput`](crate::operation::delete_voice_connector_group::DeleteVoiceConnectorGroupOutput)
    /// - On failure, responds with [`SdkError<DeleteVoiceConnectorGroupError>`](crate::operation::delete_voice_connector_group::DeleteVoiceConnectorGroupError)
    pub fn delete_voice_connector_group(&self) -> crate::operation::delete_voice_connector_group::builders::DeleteVoiceConnectorGroupFluentBuilder{
        crate::operation::delete_voice_connector_group::builders::DeleteVoiceConnectorGroupFluentBuilder::new(self.handle.clone())
    }
}
