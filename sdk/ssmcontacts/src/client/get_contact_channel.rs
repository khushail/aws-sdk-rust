// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetContactChannel`](crate::operation::get_contact_channel::builders::GetContactChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`contact_channel_id(impl ::std::convert::Into<String>)`](crate::operation::get_contact_channel::builders::GetContactChannelFluentBuilder::contact_channel_id) / [`set_contact_channel_id(Option<String>)`](crate::operation::get_contact_channel::builders::GetContactChannelFluentBuilder::set_contact_channel_id): <p>The Amazon Resource Name (ARN) of the contact channel you want information about.</p>
    /// - On success, responds with [`GetContactChannelOutput`](crate::operation::get_contact_channel::GetContactChannelOutput) with field(s):
    ///   - [`contact_arn(Option<String>)`](crate::operation::get_contact_channel::GetContactChannelOutput::contact_arn): <p>The ARN of the contact that the channel belongs to.</p>
    ///   - [`contact_channel_arn(Option<String>)`](crate::operation::get_contact_channel::GetContactChannelOutput::contact_channel_arn): <p>The ARN of the contact channel.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_contact_channel::GetContactChannelOutput::name): <p>The name of the contact channel</p>
    ///   - [`r#type(Option<ChannelType>)`](crate::operation::get_contact_channel::GetContactChannelOutput::type): <p>The type of contact channel. The type is <code>SMS</code>, <code>VOICE</code>, or <code>EMAIL</code>.</p>
    ///   - [`delivery_address(Option<ContactChannelAddress>)`](crate::operation::get_contact_channel::GetContactChannelOutput::delivery_address): <p>The details that Incident Manager uses when trying to engage the contact channel.</p>
    ///   - [`activation_status(Option<ActivationStatus>)`](crate::operation::get_contact_channel::GetContactChannelOutput::activation_status): <p>A Boolean value indicating if the contact channel has been activated or not.</p>
    /// - On failure, responds with [`SdkError<GetContactChannelError>`](crate::operation::get_contact_channel::GetContactChannelError)
    pub fn get_contact_channel(
        &self,
    ) -> crate::operation::get_contact_channel::builders::GetContactChannelFluentBuilder {
        crate::operation::get_contact_channel::builders::GetContactChannelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
