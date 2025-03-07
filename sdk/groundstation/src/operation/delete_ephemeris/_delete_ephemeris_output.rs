// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteEphemerisOutput {
    /// <p>The AWS Ground Station ephemeris ID.</p>
    #[doc(hidden)]
    pub ephemeris_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteEphemerisOutput {
    /// <p>The AWS Ground Station ephemeris ID.</p>
    pub fn ephemeris_id(&self) -> ::std::option::Option<&str> {
        self.ephemeris_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteEphemerisOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteEphemerisOutput {
    /// Creates a new builder-style object to manufacture [`DeleteEphemerisOutput`](crate::operation::delete_ephemeris::DeleteEphemerisOutput).
    pub fn builder() -> crate::operation::delete_ephemeris::builders::DeleteEphemerisOutputBuilder {
        crate::operation::delete_ephemeris::builders::DeleteEphemerisOutputBuilder::default()
    }
}

/// A builder for [`DeleteEphemerisOutput`](crate::operation::delete_ephemeris::DeleteEphemerisOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteEphemerisOutputBuilder {
    pub(crate) ephemeris_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteEphemerisOutputBuilder {
    /// <p>The AWS Ground Station ephemeris ID.</p>
    pub fn ephemeris_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ephemeris_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The AWS Ground Station ephemeris ID.</p>
    pub fn set_ephemeris_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ephemeris_id = input;
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
    /// Consumes the builder and constructs a [`DeleteEphemerisOutput`](crate::operation::delete_ephemeris::DeleteEphemerisOutput).
    pub fn build(self) -> crate::operation::delete_ephemeris::DeleteEphemerisOutput {
        crate::operation::delete_ephemeris::DeleteEphemerisOutput {
            ephemeris_id: self.ephemeris_id,
            _request_id: self._request_id,
        }
    }
}
