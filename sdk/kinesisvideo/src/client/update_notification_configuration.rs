// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateNotificationConfiguration`](crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_name(impl ::std::convert::Into<String>)`](crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder::stream_name) / [`set_stream_name(Option<String>)`](crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder::set_stream_name): <p>The name of the stream from which to update the notification configuration. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p>
    ///   - [`stream_arn(impl ::std::convert::Into<String>)`](crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder::set_stream_arn): <p>The Amazon Resource Name (ARN) of the Kinesis video stream from where you want to update the notification configuration. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p>
    ///   - [`notification_configuration(NotificationConfiguration)`](crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder::notification_configuration) / [`set_notification_configuration(Option<NotificationConfiguration>)`](crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder::set_notification_configuration): <p>The structure containing the information required for notifications. If the structure is null, the configuration will be deleted from the stream.</p>
    /// - On success, responds with [`UpdateNotificationConfigurationOutput`](crate::operation::update_notification_configuration::UpdateNotificationConfigurationOutput)
    /// - On failure, responds with [`SdkError<UpdateNotificationConfigurationError>`](crate::operation::update_notification_configuration::UpdateNotificationConfigurationError)
    pub fn update_notification_configuration(&self) -> crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder{
        crate::operation::update_notification_configuration::builders::UpdateNotificationConfigurationFluentBuilder::new(self.handle.clone())
    }
}
