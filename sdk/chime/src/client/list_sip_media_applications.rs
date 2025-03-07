// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSipMediaApplications`](crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder::set_max_results): <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder::set_next_token): <p>The token to use to retrieve the next page of results.</p>
    /// - On success, responds with [`ListSipMediaApplicationsOutput`](crate::operation::list_sip_media_applications::ListSipMediaApplicationsOutput) with field(s):
    ///   - [`sip_media_applications(Option<Vec<SipMediaApplication>>)`](crate::operation::list_sip_media_applications::ListSipMediaApplicationsOutput::sip_media_applications): <p>List of SIP media applications and application details.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_sip_media_applications::ListSipMediaApplicationsOutput::next_token): <p>The token to use to retrieve the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListSipMediaApplicationsError>`](crate::operation::list_sip_media_applications::ListSipMediaApplicationsError)
    pub fn list_sip_media_applications(&self) -> crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder{
        crate::operation::list_sip_media_applications::builders::ListSipMediaApplicationsFluentBuilder::new(self.handle.clone())
    }
}
