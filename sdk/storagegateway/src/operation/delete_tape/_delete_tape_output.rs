// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>DeleteTapeOutput</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTapeOutput {
    /// <p>The Amazon Resource Name (ARN) of the deleted virtual tape.</p>
    #[doc(hidden)]
    pub tape_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteTapeOutput {
    /// <p>The Amazon Resource Name (ARN) of the deleted virtual tape.</p>
    pub fn tape_arn(&self) -> ::std::option::Option<&str> {
        self.tape_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteTapeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTapeOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTapeOutput`](crate::operation::delete_tape::DeleteTapeOutput).
    pub fn builder() -> crate::operation::delete_tape::builders::DeleteTapeOutputBuilder {
        crate::operation::delete_tape::builders::DeleteTapeOutputBuilder::default()
    }
}

/// A builder for [`DeleteTapeOutput`](crate::operation::delete_tape::DeleteTapeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteTapeOutputBuilder {
    pub(crate) tape_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteTapeOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the deleted virtual tape.</p>
    pub fn tape_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.tape_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the deleted virtual tape.</p>
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
    /// Consumes the builder and constructs a [`DeleteTapeOutput`](crate::operation::delete_tape::DeleteTapeOutput).
    pub fn build(self) -> crate::operation::delete_tape::DeleteTapeOutput {
        crate::operation::delete_tape::DeleteTapeOutput {
            tape_arn: self.tape_arn,
            _request_id: self._request_id,
        }
    }
}
