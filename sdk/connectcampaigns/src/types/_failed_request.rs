// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// A failed request identified by the unique client token.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FailedRequest {
    /// Client provided parameter used for idempotency. Its value must be unique for each request.
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// Identifier representing a Dial request
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// A predefined code indicating the error that caused the failure.
    #[doc(hidden)]
    pub failure_code: ::std::option::Option<crate::types::FailureCode>,
}
impl FailedRequest {
    /// Client provided parameter used for idempotency. Its value must be unique for each request.
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// Identifier representing a Dial request
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// A predefined code indicating the error that caused the failure.
    pub fn failure_code(&self) -> ::std::option::Option<&crate::types::FailureCode> {
        self.failure_code.as_ref()
    }
}
impl FailedRequest {
    /// Creates a new builder-style object to manufacture [`FailedRequest`](crate::types::FailedRequest).
    pub fn builder() -> crate::types::builders::FailedRequestBuilder {
        crate::types::builders::FailedRequestBuilder::default()
    }
}

/// A builder for [`FailedRequest`](crate::types::FailedRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FailedRequestBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) failure_code: ::std::option::Option<crate::types::FailureCode>,
}
impl FailedRequestBuilder {
    /// Client provided parameter used for idempotency. Its value must be unique for each request.
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Client provided parameter used for idempotency. Its value must be unique for each request.
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Identifier representing a Dial request
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier representing a Dial request
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// A predefined code indicating the error that caused the failure.
    pub fn failure_code(mut self, input: crate::types::FailureCode) -> Self {
        self.failure_code = ::std::option::Option::Some(input);
        self
    }
    /// A predefined code indicating the error that caused the failure.
    pub fn set_failure_code(
        mut self,
        input: ::std::option::Option<crate::types::FailureCode>,
    ) -> Self {
        self.failure_code = input;
        self
    }
    /// Consumes the builder and constructs a [`FailedRequest`](crate::types::FailedRequest).
    pub fn build(self) -> crate::types::FailedRequest {
        crate::types::FailedRequest {
            client_token: self.client_token,
            id: self.id,
            failure_code: self.failure_code,
        }
    }
}
