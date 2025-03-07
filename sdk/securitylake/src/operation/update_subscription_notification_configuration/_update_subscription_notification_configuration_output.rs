// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSubscriptionNotificationConfigurationOutput {
    /// <p>Returns the ARN of the queue.</p>
    #[doc(hidden)]
    pub queue_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateSubscriptionNotificationConfigurationOutput {
    /// <p>Returns the ARN of the queue.</p>
    pub fn queue_arn(&self) -> ::std::option::Option<&str> {
        self.queue_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateSubscriptionNotificationConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateSubscriptionNotificationConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateSubscriptionNotificationConfigurationOutput`](crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationOutput).
    pub fn builder() -> crate::operation::update_subscription_notification_configuration::builders::UpdateSubscriptionNotificationConfigurationOutputBuilder{
        crate::operation::update_subscription_notification_configuration::builders::UpdateSubscriptionNotificationConfigurationOutputBuilder::default()
    }
}

/// A builder for [`UpdateSubscriptionNotificationConfigurationOutput`](crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateSubscriptionNotificationConfigurationOutputBuilder {
    pub(crate) queue_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateSubscriptionNotificationConfigurationOutputBuilder {
    /// <p>Returns the ARN of the queue.</p>
    pub fn queue_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns the ARN of the queue.</p>
    pub fn set_queue_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_arn = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateSubscriptionNotificationConfigurationOutput`](crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationOutput).
    pub fn build(self) -> crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationOutput{
        crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationOutput {
            queue_arn: self.queue_arn
            ,
            _request_id: self._request_id,
        }
    }
}
