// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about an error found in a behavior specification.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidationError {
    /// <p>The description of an error found in the behaviors.</p>
    #[doc(hidden)]
    pub error_message: ::std::option::Option<::std::string::String>,
}
impl ValidationError {
    /// <p>The description of an error found in the behaviors.</p>
    pub fn error_message(&self) -> ::std::option::Option<&str> {
        self.error_message.as_deref()
    }
}
impl ValidationError {
    /// Creates a new builder-style object to manufacture [`ValidationError`](crate::types::ValidationError).
    pub fn builder() -> crate::types::builders::ValidationErrorBuilder {
        crate::types::builders::ValidationErrorBuilder::default()
    }
}

/// A builder for [`ValidationError`](crate::types::ValidationError).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidationErrorBuilder {
    pub(crate) error_message: ::std::option::Option<::std::string::String>,
}
impl ValidationErrorBuilder {
    /// <p>The description of an error found in the behaviors.</p>
    pub fn error_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.error_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of an error found in the behaviors.</p>
    pub fn set_error_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.error_message = input;
        self
    }
    /// Consumes the builder and constructs a [`ValidationError`](crate::types::ValidationError).
    pub fn build(self) -> crate::types::ValidationError {
        crate::types::ValidationError {
            error_message: self.error_message,
        }
    }
}
