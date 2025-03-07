// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the response from the server for the registration confirmation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConfirmSignUpOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for ConfirmSignUpOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ConfirmSignUpOutput {
    /// Creates a new builder-style object to manufacture [`ConfirmSignUpOutput`](crate::operation::confirm_sign_up::ConfirmSignUpOutput).
    pub fn builder() -> crate::operation::confirm_sign_up::builders::ConfirmSignUpOutputBuilder {
        crate::operation::confirm_sign_up::builders::ConfirmSignUpOutputBuilder::default()
    }
}

/// A builder for [`ConfirmSignUpOutput`](crate::operation::confirm_sign_up::ConfirmSignUpOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConfirmSignUpOutputBuilder {
    _request_id: Option<String>,
}
impl ConfirmSignUpOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ConfirmSignUpOutput`](crate::operation::confirm_sign_up::ConfirmSignUpOutput).
    pub fn build(self) -> crate::operation::confirm_sign_up::ConfirmSignUpOutput {
        crate::operation::confirm_sign_up::ConfirmSignUpOutput {
            _request_id: self._request_id,
        }
    }
}
