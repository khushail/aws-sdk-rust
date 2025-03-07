// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>RetrieveTapeArchiveOutput</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RetrieveTapeArchiveOutput {
    /// <p>The Amazon Resource Name (ARN) of the retrieved virtual tape.</p>
    #[doc(hidden)]
    pub tape_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RetrieveTapeArchiveOutput {
    /// <p>The Amazon Resource Name (ARN) of the retrieved virtual tape.</p>
    pub fn tape_arn(&self) -> ::std::option::Option<&str> {
        self.tape_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for RetrieveTapeArchiveOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RetrieveTapeArchiveOutput {
    /// Creates a new builder-style object to manufacture [`RetrieveTapeArchiveOutput`](crate::operation::retrieve_tape_archive::RetrieveTapeArchiveOutput).
    pub fn builder(
    ) -> crate::operation::retrieve_tape_archive::builders::RetrieveTapeArchiveOutputBuilder {
        crate::operation::retrieve_tape_archive::builders::RetrieveTapeArchiveOutputBuilder::default(
        )
    }
}

/// A builder for [`RetrieveTapeArchiveOutput`](crate::operation::retrieve_tape_archive::RetrieveTapeArchiveOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RetrieveTapeArchiveOutputBuilder {
    pub(crate) tape_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RetrieveTapeArchiveOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the retrieved virtual tape.</p>
    pub fn tape_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.tape_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the retrieved virtual tape.</p>
    pub fn set_tape_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.tape_arn = input;
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
    /// Consumes the builder and constructs a [`RetrieveTapeArchiveOutput`](crate::operation::retrieve_tape_archive::RetrieveTapeArchiveOutput).
    pub fn build(self) -> crate::operation::retrieve_tape_archive::RetrieveTapeArchiveOutput {
        crate::operation::retrieve_tape_archive::RetrieveTapeArchiveOutput {
            tape_arn: self.tape_arn,
            _request_id: self._request_id,
        }
    }
}
