// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutVoiceConnectorLoggingConfiguration`](crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl ::std::convert::Into<String>)`](crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationFluentBuilder::set_voice_connector_id): <p>The Voice Connector ID.</p>
    ///   - [`logging_configuration(LoggingConfiguration)`](crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationFluentBuilder::logging_configuration) / [`set_logging_configuration(Option<LoggingConfiguration>)`](crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationFluentBuilder::set_logging_configuration): <p>The logging configuration being updated.</p>
    /// - On success, responds with [`PutVoiceConnectorLoggingConfigurationOutput`](crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationOutput) with field(s):
    ///   - [`logging_configuration(Option<LoggingConfiguration>)`](crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationOutput::logging_configuration): <p>The updated logging configuration.</p>
    /// - On failure, responds with [`SdkError<PutVoiceConnectorLoggingConfigurationError>`](crate::operation::put_voice_connector_logging_configuration::PutVoiceConnectorLoggingConfigurationError)
    pub fn put_voice_connector_logging_configuration(&self) -> crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationFluentBuilder{
        crate::operation::put_voice_connector_logging_configuration::builders::PutVoiceConnectorLoggingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
