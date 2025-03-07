// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the context of an action in the stage of a pipeline to a job worker.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ActionContext {
    /// <p>The name of the action in the context of a job.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The system-generated unique ID that corresponds to an action's execution.</p>
    #[doc(hidden)]
    pub action_execution_id: ::std::option::Option<::std::string::String>,
}
impl ActionContext {
    /// <p>The name of the action in the context of a job.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The system-generated unique ID that corresponds to an action's execution.</p>
    pub fn action_execution_id(&self) -> ::std::option::Option<&str> {
        self.action_execution_id.as_deref()
    }
}
impl ActionContext {
    /// Creates a new builder-style object to manufacture [`ActionContext`](crate::types::ActionContext).
    pub fn builder() -> crate::types::builders::ActionContextBuilder {
        crate::types::builders::ActionContextBuilder::default()
    }
}

/// A builder for [`ActionContext`](crate::types::ActionContext).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ActionContextBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) action_execution_id: ::std::option::Option<::std::string::String>,
}
impl ActionContextBuilder {
    /// <p>The name of the action in the context of a job.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the action in the context of a job.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The system-generated unique ID that corresponds to an action's execution.</p>
    pub fn action_execution_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.action_execution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The system-generated unique ID that corresponds to an action's execution.</p>
    pub fn set_action_execution_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.action_execution_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ActionContext`](crate::types::ActionContext).
    pub fn build(self) -> crate::types::ActionContext {
        crate::types::ActionContext {
            name: self.name,
            action_execution_id: self.action_execution_id,
        }
    }
}
