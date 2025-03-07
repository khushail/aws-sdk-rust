// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents information about the state of an action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ActionState {
    /// <p>The name of the action.</p>
    #[doc(hidden)]
    pub action_name: ::std::option::Option<::std::string::String>,
    /// <p>Represents information about the version (or revision) of an action.</p>
    #[doc(hidden)]
    pub current_revision: ::std::option::Option<crate::types::ActionRevision>,
    /// <p>Represents information about the run of an action.</p>
    #[doc(hidden)]
    pub latest_execution: ::std::option::Option<crate::types::ActionExecution>,
    /// <p>A URL link for more information about the state of the action, such as a deployment group details page.</p>
    #[doc(hidden)]
    pub entity_url: ::std::option::Option<::std::string::String>,
    /// <p>A URL link for more information about the revision, such as a commit details page.</p>
    #[doc(hidden)]
    pub revision_url: ::std::option::Option<::std::string::String>,
}
impl ActionState {
    /// <p>The name of the action.</p>
    pub fn action_name(&self) -> ::std::option::Option<&str> {
        self.action_name.as_deref()
    }
    /// <p>Represents information about the version (or revision) of an action.</p>
    pub fn current_revision(&self) -> ::std::option::Option<&crate::types::ActionRevision> {
        self.current_revision.as_ref()
    }
    /// <p>Represents information about the run of an action.</p>
    pub fn latest_execution(&self) -> ::std::option::Option<&crate::types::ActionExecution> {
        self.latest_execution.as_ref()
    }
    /// <p>A URL link for more information about the state of the action, such as a deployment group details page.</p>
    pub fn entity_url(&self) -> ::std::option::Option<&str> {
        self.entity_url.as_deref()
    }
    /// <p>A URL link for more information about the revision, such as a commit details page.</p>
    pub fn revision_url(&self) -> ::std::option::Option<&str> {
        self.revision_url.as_deref()
    }
}
impl ActionState {
    /// Creates a new builder-style object to manufacture [`ActionState`](crate::types::ActionState).
    pub fn builder() -> crate::types::builders::ActionStateBuilder {
        crate::types::builders::ActionStateBuilder::default()
    }
}

/// A builder for [`ActionState`](crate::types::ActionState).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ActionStateBuilder {
    pub(crate) action_name: ::std::option::Option<::std::string::String>,
    pub(crate) current_revision: ::std::option::Option<crate::types::ActionRevision>,
    pub(crate) latest_execution: ::std::option::Option<crate::types::ActionExecution>,
    pub(crate) entity_url: ::std::option::Option<::std::string::String>,
    pub(crate) revision_url: ::std::option::Option<::std::string::String>,
}
impl ActionStateBuilder {
    /// <p>The name of the action.</p>
    pub fn action_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the action.</p>
    pub fn set_action_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_name = input;
        self
    }
    /// <p>Represents information about the version (or revision) of an action.</p>
    pub fn current_revision(mut self, input: crate::types::ActionRevision) -> Self {
        self.current_revision = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents information about the version (or revision) of an action.</p>
    pub fn set_current_revision(
        mut self,
        input: ::std::option::Option<crate::types::ActionRevision>,
    ) -> Self {
        self.current_revision = input;
        self
    }
    /// <p>Represents information about the run of an action.</p>
    pub fn latest_execution(mut self, input: crate::types::ActionExecution) -> Self {
        self.latest_execution = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents information about the run of an action.</p>
    pub fn set_latest_execution(
        mut self,
        input: ::std::option::Option<crate::types::ActionExecution>,
    ) -> Self {
        self.latest_execution = input;
        self
    }
    /// <p>A URL link for more information about the state of the action, such as a deployment group details page.</p>
    pub fn entity_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A URL link for more information about the state of the action, such as a deployment group details page.</p>
    pub fn set_entity_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_url = input;
        self
    }
    /// <p>A URL link for more information about the revision, such as a commit details page.</p>
    pub fn revision_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.revision_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A URL link for more information about the revision, such as a commit details page.</p>
    pub fn set_revision_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.revision_url = input;
        self
    }
    /// Consumes the builder and constructs a [`ActionState`](crate::types::ActionState).
    pub fn build(self) -> crate::types::ActionState {
        crate::types::ActionState {
            action_name: self.action_name,
            current_revision: self.current_revision,
            latest_execution: self.latest_execution,
            entity_url: self.entity_url,
            revision_url: self.revision_url,
        }
    }
}
