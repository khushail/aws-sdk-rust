// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RollbackStackOutput {
    /// <p>Unique identifier of the stack.</p>
    #[doc(hidden)]
    pub stack_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RollbackStackOutput {
    /// <p>Unique identifier of the stack.</p>
    pub fn stack_id(&self) -> ::std::option::Option<&str> {
        self.stack_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for RollbackStackOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RollbackStackOutput {
    /// Creates a new builder-style object to manufacture [`RollbackStackOutput`](crate::operation::rollback_stack::RollbackStackOutput).
    pub fn builder() -> crate::operation::rollback_stack::builders::RollbackStackOutputBuilder {
        crate::operation::rollback_stack::builders::RollbackStackOutputBuilder::default()
    }
}

/// A builder for [`RollbackStackOutput`](crate::operation::rollback_stack::RollbackStackOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RollbackStackOutputBuilder {
    pub(crate) stack_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RollbackStackOutputBuilder {
    /// <p>Unique identifier of the stack.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stack_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique identifier of the stack.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stack_id = input;
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
    /// Consumes the builder and constructs a [`RollbackStackOutput`](crate::operation::rollback_stack::RollbackStackOutput).
    pub fn build(self) -> crate::operation::rollback_stack::RollbackStackOutput {
        crate::operation::rollback_stack::RollbackStackOutput {
            stack_id: self.stack_id,
            _request_id: self._request_id,
        }
    }
}
