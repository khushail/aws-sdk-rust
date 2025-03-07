// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeactivateContactChannel`](crate::operation::deactivate_contact_channel::builders::DeactivateContactChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`contact_channel_id(impl ::std::convert::Into<String>)`](crate::operation::deactivate_contact_channel::builders::DeactivateContactChannelFluentBuilder::contact_channel_id) / [`set_contact_channel_id(Option<String>)`](crate::operation::deactivate_contact_channel::builders::DeactivateContactChannelFluentBuilder::set_contact_channel_id): <p>The Amazon Resource Name (ARN) of the contact channel you're deactivating.</p>
    /// - On success, responds with [`DeactivateContactChannelOutput`](crate::operation::deactivate_contact_channel::DeactivateContactChannelOutput)
    /// - On failure, responds with [`SdkError<DeactivateContactChannelError>`](crate::operation::deactivate_contact_channel::DeactivateContactChannelError)
    pub fn deactivate_contact_channel(
        &self,
    ) -> crate::operation::deactivate_contact_channel::builders::DeactivateContactChannelFluentBuilder
    {
        crate::operation::deactivate_contact_channel::builders::DeactivateContactChannelFluentBuilder::new(self.handle.clone())
    }
}
