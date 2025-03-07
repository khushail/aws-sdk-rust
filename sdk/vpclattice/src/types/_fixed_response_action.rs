// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about an action that returns a custom HTTP response. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FixedResponseAction {
    /// <p>The HTTP response code.</p>
    #[doc(hidden)]
    pub status_code: ::std::option::Option<i32>,
}
impl FixedResponseAction {
    /// <p>The HTTP response code.</p>
    pub fn status_code(&self) -> ::std::option::Option<i32> {
        self.status_code
    }
}
impl FixedResponseAction {
    /// Creates a new builder-style object to manufacture [`FixedResponseAction`](crate::types::FixedResponseAction).
    pub fn builder() -> crate::types::builders::FixedResponseActionBuilder {
        crate::types::builders::FixedResponseActionBuilder::default()
    }
}

/// A builder for [`FixedResponseAction`](crate::types::FixedResponseAction).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FixedResponseActionBuilder {
    pub(crate) status_code: ::std::option::Option<i32>,
}
impl FixedResponseActionBuilder {
    /// <p>The HTTP response code.</p>
    pub fn status_code(mut self, input: i32) -> Self {
        self.status_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The HTTP response code.</p>
    pub fn set_status_code(mut self, input: ::std::option::Option<i32>) -> Self {
        self.status_code = input;
        self
    }
    /// Consumes the builder and constructs a [`FixedResponseAction`](crate::types::FixedResponseAction).
    pub fn build(self) -> crate::types::FixedResponseAction {
        crate::types::FixedResponseAction {
            status_code: self.status_code,
        }
    }
}
