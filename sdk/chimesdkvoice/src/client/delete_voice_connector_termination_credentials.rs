// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVoiceConnectorTerminationCredentials`](crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl ::std::convert::Into<String>)`](crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsFluentBuilder::set_voice_connector_id): <p>The Voice Connector ID.</p>
    ///   - [`usernames(Vec<String>)`](crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsFluentBuilder::usernames) / [`set_usernames(Option<Vec<String>>)`](crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsFluentBuilder::set_usernames): <p>The RFC2617 compliant username associated with the SIP credentials, in US-ASCII format.</p>
    /// - On success, responds with [`DeleteVoiceConnectorTerminationCredentialsOutput`](crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsOutput)
    /// - On failure, responds with [`SdkError<DeleteVoiceConnectorTerminationCredentialsError>`](crate::operation::delete_voice_connector_termination_credentials::DeleteVoiceConnectorTerminationCredentialsError)
    pub fn delete_voice_connector_termination_credentials(&self) -> crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsFluentBuilder{
        crate::operation::delete_voice_connector_termination_credentials::builders::DeleteVoiceConnectorTerminationCredentialsFluentBuilder::new(self.handle.clone())
    }
}
