// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelHandshakeOutput {
    /// <p>A structure that contains details about the handshake that you canceled.</p>
    #[doc(hidden)]
    pub handshake: ::std::option::Option<crate::types::Handshake>,
    _request_id: Option<String>,
}
impl CancelHandshakeOutput {
    /// <p>A structure that contains details about the handshake that you canceled.</p>
    pub fn handshake(&self) -> ::std::option::Option<&crate::types::Handshake> {
        self.handshake.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for CancelHandshakeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CancelHandshakeOutput {
    /// Creates a new builder-style object to manufacture [`CancelHandshakeOutput`](crate::operation::cancel_handshake::CancelHandshakeOutput).
    pub fn builder() -> crate::operation::cancel_handshake::builders::CancelHandshakeOutputBuilder {
        crate::operation::cancel_handshake::builders::CancelHandshakeOutputBuilder::default()
    }
}

/// A builder for [`CancelHandshakeOutput`](crate::operation::cancel_handshake::CancelHandshakeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelHandshakeOutputBuilder {
    pub(crate) handshake: ::std::option::Option<crate::types::Handshake>,
    _request_id: Option<String>,
}
impl CancelHandshakeOutputBuilder {
    /// <p>A structure that contains details about the handshake that you canceled.</p>
    pub fn handshake(mut self, input: crate::types::Handshake) -> Self {
        self.handshake = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure that contains details about the handshake that you canceled.</p>
    pub fn set_handshake(mut self, input: ::std::option::Option<crate::types::Handshake>) -> Self {
        self.handshake = input;
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
    /// Consumes the builder and constructs a [`CancelHandshakeOutput`](crate::operation::cancel_handshake::CancelHandshakeOutput).
    pub fn build(self) -> crate::operation::cancel_handshake::CancelHandshakeOutput {
        crate::operation::cancel_handshake::CancelHandshakeOutput {
            handshake: self.handshake,
            _request_id: self._request_id,
        }
    }
}
