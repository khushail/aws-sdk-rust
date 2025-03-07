// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVoiceConnectorLoggingConfiguration`](crate::operation::get_voice_connector_logging_configuration::builders::GetVoiceConnectorLoggingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl ::std::convert::Into<String>)`](crate::operation::get_voice_connector_logging_configuration::builders::GetVoiceConnectorLoggingConfigurationFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::get_voice_connector_logging_configuration::builders::GetVoiceConnectorLoggingConfigurationFluentBuilder::set_voice_connector_id): <p>The Amazon Chime Voice Connector ID.</p>
    /// - On success, responds with [`GetVoiceConnectorLoggingConfigurationOutput`](crate::operation::get_voice_connector_logging_configuration::GetVoiceConnectorLoggingConfigurationOutput) with field(s):
    ///   - [`logging_configuration(Option<LoggingConfiguration>)`](crate::operation::get_voice_connector_logging_configuration::GetVoiceConnectorLoggingConfigurationOutput::logging_configuration): <p>The logging configuration details.</p>
    /// - On failure, responds with [`SdkError<GetVoiceConnectorLoggingConfigurationError>`](crate::operation::get_voice_connector_logging_configuration::GetVoiceConnectorLoggingConfigurationError)
    pub fn get_voice_connector_logging_configuration(&self) -> crate::operation::get_voice_connector_logging_configuration::builders::GetVoiceConnectorLoggingConfigurationFluentBuilder{
        crate::operation::get_voice_connector_logging_configuration::builders::GetVoiceConnectorLoggingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
