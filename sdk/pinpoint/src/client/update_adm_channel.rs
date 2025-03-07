// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAdmChannel`](crate::operation::update_adm_channel::builders::UpdateAdmChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`adm_channel_request(AdmChannelRequest)`](crate::operation::update_adm_channel::builders::UpdateAdmChannelFluentBuilder::adm_channel_request) / [`set_adm_channel_request(Option<AdmChannelRequest>)`](crate::operation::update_adm_channel::builders::UpdateAdmChannelFluentBuilder::set_adm_channel_request): <p>Specifies the status and settings of the ADM (Amazon Device Messaging) channel for an application.</p>
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::update_adm_channel::builders::UpdateAdmChannelFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::update_adm_channel::builders::UpdateAdmChannelFluentBuilder::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    /// - On success, responds with [`UpdateAdmChannelOutput`](crate::operation::update_adm_channel::UpdateAdmChannelOutput) with field(s):
    ///   - [`adm_channel_response(Option<AdmChannelResponse>)`](crate::operation::update_adm_channel::UpdateAdmChannelOutput::adm_channel_response): <p>Provides information about the status and settings of the ADM (Amazon Device Messaging) channel for an application.</p>
    /// - On failure, responds with [`SdkError<UpdateAdmChannelError>`](crate::operation::update_adm_channel::UpdateAdmChannelError)
    pub fn update_adm_channel(
        &self,
    ) -> crate::operation::update_adm_channel::builders::UpdateAdmChannelFluentBuilder {
        crate::operation::update_adm_channel::builders::UpdateAdmChannelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
