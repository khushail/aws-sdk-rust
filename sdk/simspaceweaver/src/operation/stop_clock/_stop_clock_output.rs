// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopClockOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for StopClockOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StopClockOutput {
    /// Creates a new builder-style object to manufacture [`StopClockOutput`](crate::operation::stop_clock::StopClockOutput).
    pub fn builder() -> crate::operation::stop_clock::builders::StopClockOutputBuilder {
        crate::operation::stop_clock::builders::StopClockOutputBuilder::default()
    }
}

/// A builder for [`StopClockOutput`](crate::operation::stop_clock::StopClockOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopClockOutputBuilder {
    _request_id: Option<String>,
}
impl StopClockOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StopClockOutput`](crate::operation::stop_clock::StopClockOutput).
    pub fn build(self) -> crate::operation::stop_clock::StopClockOutput {
        crate::operation::stop_clock::StopClockOutput {
            _request_id: self._request_id,
        }
    }
}
