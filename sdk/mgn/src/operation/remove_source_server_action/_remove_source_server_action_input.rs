// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RemoveSourceServerActionInput {
    /// <p>Source server ID of the post migration custom action to remove.</p>
    #[doc(hidden)]
    pub source_server_id: ::std::option::Option<::std::string::String>,
    /// <p>Source server post migration custom action ID to remove.</p>
    #[doc(hidden)]
    pub action_id: ::std::option::Option<::std::string::String>,
}
impl RemoveSourceServerActionInput {
    /// <p>Source server ID of the post migration custom action to remove.</p>
    pub fn source_server_id(&self) -> ::std::option::Option<&str> {
        self.source_server_id.as_deref()
    }
    /// <p>Source server post migration custom action ID to remove.</p>
    pub fn action_id(&self) -> ::std::option::Option<&str> {
        self.action_id.as_deref()
    }
}
impl RemoveSourceServerActionInput {
    /// Creates a new builder-style object to manufacture [`RemoveSourceServerActionInput`](crate::operation::remove_source_server_action::RemoveSourceServerActionInput).
    pub fn builder(
    ) -> crate::operation::remove_source_server_action::builders::RemoveSourceServerActionInputBuilder
    {
        crate::operation::remove_source_server_action::builders::RemoveSourceServerActionInputBuilder::default()
    }
}

/// A builder for [`RemoveSourceServerActionInput`](crate::operation::remove_source_server_action::RemoveSourceServerActionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RemoveSourceServerActionInputBuilder {
    pub(crate) source_server_id: ::std::option::Option<::std::string::String>,
    pub(crate) action_id: ::std::option::Option<::std::string::String>,
}
impl RemoveSourceServerActionInputBuilder {
    /// <p>Source server ID of the post migration custom action to remove.</p>
    pub fn source_server_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_server_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Source server ID of the post migration custom action to remove.</p>
    pub fn set_source_server_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_server_id = input;
        self
    }
    /// <p>Source server post migration custom action ID to remove.</p>
    pub fn action_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Source server post migration custom action ID to remove.</p>
    pub fn set_action_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_id = input;
        self
    }
    /// Consumes the builder and constructs a [`RemoveSourceServerActionInput`](crate::operation::remove_source_server_action::RemoveSourceServerActionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::remove_source_server_action::RemoveSourceServerActionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::remove_source_server_action::RemoveSourceServerActionInput {
                source_server_id: self.source_server_id,
                action_id: self.action_id,
            },
        )
    }
}
