// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteConfigurationSetEventDestination`](crate::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl ::std::convert::Into<String>)`](crate::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationFluentBuilder::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationFluentBuilder::set_configuration_set_name): <p>The name of the configuration set that contains the event destination that you want to delete.</p>
    ///   - [`event_destination_name(impl ::std::convert::Into<String>)`](crate::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationFluentBuilder::event_destination_name) / [`set_event_destination_name(Option<String>)`](crate::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationFluentBuilder::set_event_destination_name): <p>The name of the event destination that you want to delete.</p>
    /// - On success, responds with [`DeleteConfigurationSetEventDestinationOutput`](crate::operation::delete_configuration_set_event_destination::DeleteConfigurationSetEventDestinationOutput)
    /// - On failure, responds with [`SdkError<DeleteConfigurationSetEventDestinationError>`](crate::operation::delete_configuration_set_event_destination::DeleteConfigurationSetEventDestinationError)
    pub fn delete_configuration_set_event_destination(&self) -> crate::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationFluentBuilder{
        crate::operation::delete_configuration_set_event_destination::builders::DeleteConfigurationSetEventDestinationFluentBuilder::new(self.handle.clone())
    }
}
