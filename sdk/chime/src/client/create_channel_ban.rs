// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateChannelBan`](crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl ::std::convert::Into<String>)`](crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder::set_channel_arn): <p>The ARN of the ban request.</p>
    ///   - [`member_arn(impl ::std::convert::Into<String>)`](crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder::member_arn) / [`set_member_arn(Option<String>)`](crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder::set_member_arn): <p>The ARN of the member being banned.</p>
    ///   - [`chime_bearer(impl ::std::convert::Into<String>)`](crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    /// - On success, responds with [`CreateChannelBanOutput`](crate::operation::create_channel_ban::CreateChannelBanOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::operation::create_channel_ban::CreateChannelBanOutput::channel_arn): <p>The ARN of the response to the ban request.</p>
    ///   - [`member(Option<Identity>)`](crate::operation::create_channel_ban::CreateChannelBanOutput::member): <p>The <code>ChannelArn</code> and <code>BannedIdentity</code> of the member in the ban response.</p>
    /// - On failure, responds with [`SdkError<CreateChannelBanError>`](crate::operation::create_channel_ban::CreateChannelBanError)
    pub fn create_channel_ban(
        &self,
    ) -> crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder {
        crate::operation::create_channel_ban::builders::CreateChannelBanFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
