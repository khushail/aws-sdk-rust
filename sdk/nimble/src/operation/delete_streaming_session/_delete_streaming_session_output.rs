// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteStreamingSessionOutput {
    /// <p>The session.</p>
    #[doc(hidden)]
    pub session: ::std::option::Option<crate::types::StreamingSession>,
    _request_id: Option<String>,
}
impl DeleteStreamingSessionOutput {
    /// <p>The session.</p>
    pub fn session(&self) -> ::std::option::Option<&crate::types::StreamingSession> {
        self.session.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteStreamingSessionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteStreamingSessionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteStreamingSessionOutput`](crate::operation::delete_streaming_session::DeleteStreamingSessionOutput).
    pub fn builder(
    ) -> crate::operation::delete_streaming_session::builders::DeleteStreamingSessionOutputBuilder
    {
        crate::operation::delete_streaming_session::builders::DeleteStreamingSessionOutputBuilder::default()
    }
}

/// A builder for [`DeleteStreamingSessionOutput`](crate::operation::delete_streaming_session::DeleteStreamingSessionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteStreamingSessionOutputBuilder {
    pub(crate) session: ::std::option::Option<crate::types::StreamingSession>,
    _request_id: Option<String>,
}
impl DeleteStreamingSessionOutputBuilder {
    /// <p>The session.</p>
    pub fn session(mut self, input: crate::types::StreamingSession) -> Self {
        self.session = ::std::option::Option::Some(input);
        self
    }
    /// <p>The session.</p>
    pub fn set_session(
        mut self,
        input: ::std::option::Option<crate::types::StreamingSession>,
    ) -> Self {
        self.session = input;
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
    /// Consumes the builder and constructs a [`DeleteStreamingSessionOutput`](crate::operation::delete_streaming_session::DeleteStreamingSessionOutput).
    pub fn build(self) -> crate::operation::delete_streaming_session::DeleteStreamingSessionOutput {
        crate::operation::delete_streaming_session::DeleteStreamingSessionOutput {
            session: self.session,
            _request_id: self._request_id,
        }
    }
}
