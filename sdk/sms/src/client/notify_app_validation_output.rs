// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`NotifyAppValidationOutput`](crate::operation::notify_app_validation_output::builders::NotifyAppValidationOutputFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::notify_app_validation_output::builders::NotifyAppValidationOutputFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::notify_app_validation_output::builders::NotifyAppValidationOutputFluentBuilder::set_app_id): <p>The ID of the application.</p>
    ///   - [`notification_context(NotificationContext)`](crate::operation::notify_app_validation_output::builders::NotifyAppValidationOutputFluentBuilder::notification_context) / [`set_notification_context(Option<NotificationContext>)`](crate::operation::notify_app_validation_output::builders::NotifyAppValidationOutputFluentBuilder::set_notification_context): <p>The notification information.</p>
    /// - On success, responds with [`NotifyAppValidationOutputOutput`](crate::operation::notify_app_validation_output::NotifyAppValidationOutputOutput)
    /// - On failure, responds with [`SdkError<NotifyAppValidationOutputError>`](crate::operation::notify_app_validation_output::NotifyAppValidationOutputError)
    pub fn notify_app_validation_output(&self) -> crate::operation::notify_app_validation_output::builders::NotifyAppValidationOutputFluentBuilder{
        crate::operation::notify_app_validation_output::builders::NotifyAppValidationOutputFluentBuilder::new(self.handle.clone())
    }
}
