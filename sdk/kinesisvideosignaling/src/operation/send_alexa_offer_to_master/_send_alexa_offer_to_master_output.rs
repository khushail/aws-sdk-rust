// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SendAlexaOfferToMasterOutput {
    /// <p>The base64-encoded SDP answer content.</p>
    #[doc(hidden)]
    pub answer: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SendAlexaOfferToMasterOutput {
    /// <p>The base64-encoded SDP answer content.</p>
    pub fn answer(&self) -> ::std::option::Option<&str> {
        self.answer.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for SendAlexaOfferToMasterOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SendAlexaOfferToMasterOutput {
    /// Creates a new builder-style object to manufacture [`SendAlexaOfferToMasterOutput`](crate::operation::send_alexa_offer_to_master::SendAlexaOfferToMasterOutput).
    pub fn builder(
    ) -> crate::operation::send_alexa_offer_to_master::builders::SendAlexaOfferToMasterOutputBuilder
    {
        crate::operation::send_alexa_offer_to_master::builders::SendAlexaOfferToMasterOutputBuilder::default()
    }
}

/// A builder for [`SendAlexaOfferToMasterOutput`](crate::operation::send_alexa_offer_to_master::SendAlexaOfferToMasterOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SendAlexaOfferToMasterOutputBuilder {
    pub(crate) answer: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SendAlexaOfferToMasterOutputBuilder {
    /// <p>The base64-encoded SDP answer content.</p>
    pub fn answer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.answer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded SDP answer content.</p>
    pub fn set_answer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.answer = input;
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
    /// Consumes the builder and constructs a [`SendAlexaOfferToMasterOutput`](crate::operation::send_alexa_offer_to_master::SendAlexaOfferToMasterOutput).
    pub fn build(
        self,
    ) -> crate::operation::send_alexa_offer_to_master::SendAlexaOfferToMasterOutput {
        crate::operation::send_alexa_offer_to_master::SendAlexaOfferToMasterOutput {
            answer: self.answer,
            _request_id: self._request_id,
        }
    }
}
