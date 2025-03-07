// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A workflow parameter.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct WorkflowParameter {
    /// <p>The parameter's description.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Whether the parameter is optional.</p>
    #[doc(hidden)]
    pub optional: ::std::option::Option<bool>,
}
impl WorkflowParameter {
    /// <p>The parameter's description.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Whether the parameter is optional.</p>
    pub fn optional(&self) -> ::std::option::Option<bool> {
        self.optional
    }
}
impl WorkflowParameter {
    /// Creates a new builder-style object to manufacture [`WorkflowParameter`](crate::types::WorkflowParameter).
    pub fn builder() -> crate::types::builders::WorkflowParameterBuilder {
        crate::types::builders::WorkflowParameterBuilder::default()
    }
}

/// A builder for [`WorkflowParameter`](crate::types::WorkflowParameter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WorkflowParameterBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) optional: ::std::option::Option<bool>,
}
impl WorkflowParameterBuilder {
    /// <p>The parameter's description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The parameter's description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Whether the parameter is optional.</p>
    pub fn optional(mut self, input: bool) -> Self {
        self.optional = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether the parameter is optional.</p>
    pub fn set_optional(mut self, input: ::std::option::Option<bool>) -> Self {
        self.optional = input;
        self
    }
    /// Consumes the builder and constructs a [`WorkflowParameter`](crate::types::WorkflowParameter).
    pub fn build(self) -> crate::types::WorkflowParameter {
        crate::types::WorkflowParameter {
            description: self.description,
            optional: self.optional,
        }
    }
}
