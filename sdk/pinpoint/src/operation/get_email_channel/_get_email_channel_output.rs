// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetEmailChannelOutput {
    /// <p>Provides information about the status and settings of the email channel for an application.</p>
    #[doc(hidden)]
    pub email_channel_response: ::std::option::Option<crate::types::EmailChannelResponse>,
    _request_id: Option<String>,
}
impl GetEmailChannelOutput {
    /// <p>Provides information about the status and settings of the email channel for an application.</p>
    pub fn email_channel_response(
        &self,
    ) -> ::std::option::Option<&crate::types::EmailChannelResponse> {
        self.email_channel_response.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetEmailChannelOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetEmailChannelOutput {
    /// Creates a new builder-style object to manufacture [`GetEmailChannelOutput`](crate::operation::get_email_channel::GetEmailChannelOutput).
    pub fn builder() -> crate::operation::get_email_channel::builders::GetEmailChannelOutputBuilder
    {
        crate::operation::get_email_channel::builders::GetEmailChannelOutputBuilder::default()
    }
}

/// A builder for [`GetEmailChannelOutput`](crate::operation::get_email_channel::GetEmailChannelOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetEmailChannelOutputBuilder {
    pub(crate) email_channel_response: ::std::option::Option<crate::types::EmailChannelResponse>,
    _request_id: Option<String>,
}
impl GetEmailChannelOutputBuilder {
    /// <p>Provides information about the status and settings of the email channel for an application.</p>
    pub fn email_channel_response(mut self, input: crate::types::EmailChannelResponse) -> Self {
        self.email_channel_response = ::std::option::Option::Some(input);
        self
    }
    /// <p>Provides information about the status and settings of the email channel for an application.</p>
    pub fn set_email_channel_response(
        mut self,
        input: ::std::option::Option<crate::types::EmailChannelResponse>,
    ) -> Self {
        self.email_channel_response = input;
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
    /// Consumes the builder and constructs a [`GetEmailChannelOutput`](crate::operation::get_email_channel::GetEmailChannelOutput).
    pub fn build(self) -> crate::operation::get_email_channel::GetEmailChannelOutput {
        crate::operation::get_email_channel::GetEmailChannelOutput {
            email_channel_response: self.email_channel_response,
            _request_id: self._request_id,
        }
    }
}
