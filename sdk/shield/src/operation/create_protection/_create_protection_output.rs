// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateProtectionOutput {
    /// <p>The unique identifier (ID) for the <code>Protection</code> object that is created.</p>
    #[doc(hidden)]
    pub protection_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateProtectionOutput {
    /// <p>The unique identifier (ID) for the <code>Protection</code> object that is created.</p>
    pub fn protection_id(&self) -> ::std::option::Option<&str> {
        self.protection_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateProtectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateProtectionOutput {
    /// Creates a new builder-style object to manufacture [`CreateProtectionOutput`](crate::operation::create_protection::CreateProtectionOutput).
    pub fn builder() -> crate::operation::create_protection::builders::CreateProtectionOutputBuilder
    {
        crate::operation::create_protection::builders::CreateProtectionOutputBuilder::default()
    }
}

/// A builder for [`CreateProtectionOutput`](crate::operation::create_protection::CreateProtectionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateProtectionOutputBuilder {
    pub(crate) protection_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateProtectionOutputBuilder {
    /// <p>The unique identifier (ID) for the <code>Protection</code> object that is created.</p>
    pub fn protection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.protection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier (ID) for the <code>Protection</code> object that is created.</p>
    pub fn set_protection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.protection_id = input;
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
    /// Consumes the builder and constructs a [`CreateProtectionOutput`](crate::operation::create_protection::CreateProtectionOutput).
    pub fn build(self) -> crate::operation::create_protection::CreateProtectionOutput {
        crate::operation::create_protection::CreateProtectionOutput {
            protection_id: self.protection_id,
            _request_id: self._request_id,
        }
    }
}
