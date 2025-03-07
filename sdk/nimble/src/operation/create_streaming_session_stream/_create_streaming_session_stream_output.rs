// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateStreamingSessionStreamOutput {
    /// <p>The stream.</p>
    #[doc(hidden)]
    pub stream: ::std::option::Option<crate::types::StreamingSessionStream>,
    _request_id: Option<String>,
}
impl CreateStreamingSessionStreamOutput {
    /// <p>The stream.</p>
    pub fn stream(&self) -> ::std::option::Option<&crate::types::StreamingSessionStream> {
        self.stream.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for CreateStreamingSessionStreamOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateStreamingSessionStreamOutput {
    /// Creates a new builder-style object to manufacture [`CreateStreamingSessionStreamOutput`](crate::operation::create_streaming_session_stream::CreateStreamingSessionStreamOutput).
    pub fn builder() -> crate::operation::create_streaming_session_stream::builders::CreateStreamingSessionStreamOutputBuilder{
        crate::operation::create_streaming_session_stream::builders::CreateStreamingSessionStreamOutputBuilder::default()
    }
}

/// A builder for [`CreateStreamingSessionStreamOutput`](crate::operation::create_streaming_session_stream::CreateStreamingSessionStreamOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateStreamingSessionStreamOutputBuilder {
    pub(crate) stream: ::std::option::Option<crate::types::StreamingSessionStream>,
    _request_id: Option<String>,
}
impl CreateStreamingSessionStreamOutputBuilder {
    /// <p>The stream.</p>
    pub fn stream(mut self, input: crate::types::StreamingSessionStream) -> Self {
        self.stream = ::std::option::Option::Some(input);
        self
    }
    /// <p>The stream.</p>
    pub fn set_stream(
        mut self,
        input: ::std::option::Option<crate::types::StreamingSessionStream>,
    ) -> Self {
        self.stream = input;
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
    /// Consumes the builder and constructs a [`CreateStreamingSessionStreamOutput`](crate::operation::create_streaming_session_stream::CreateStreamingSessionStreamOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_streaming_session_stream::CreateStreamingSessionStreamOutput {
        crate::operation::create_streaming_session_stream::CreateStreamingSessionStreamOutput {
            stream: self.stream,
            _request_id: self._request_id,
        }
    }
}
