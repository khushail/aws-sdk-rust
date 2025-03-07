// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteEventsConfiguration`](crate::operation::delete_events_configuration::builders::DeleteEventsConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_events_configuration::builders::DeleteEventsConfigurationFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_events_configuration::builders::DeleteEventsConfigurationFluentBuilder::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`bot_id(impl ::std::convert::Into<String>)`](crate::operation::delete_events_configuration::builders::DeleteEventsConfigurationFluentBuilder::bot_id) / [`set_bot_id(Option<String>)`](crate::operation::delete_events_configuration::builders::DeleteEventsConfigurationFluentBuilder::set_bot_id): <p>The bot ID.</p>
    /// - On success, responds with [`DeleteEventsConfigurationOutput`](crate::operation::delete_events_configuration::DeleteEventsConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteEventsConfigurationError>`](crate::operation::delete_events_configuration::DeleteEventsConfigurationError)
    pub fn delete_events_configuration(&self) -> crate::operation::delete_events_configuration::builders::DeleteEventsConfigurationFluentBuilder{
        crate::operation::delete_events_configuration::builders::DeleteEventsConfigurationFluentBuilder::new(self.handle.clone())
    }
}
