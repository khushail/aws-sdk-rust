// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The compromised credentials actions type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CompromisedCredentialsActionsType {
    /// <p>The event action.</p>
    #[doc(hidden)]
    pub event_action: ::std::option::Option<crate::types::CompromisedCredentialsEventActionType>,
}
impl CompromisedCredentialsActionsType {
    /// <p>The event action.</p>
    pub fn event_action(
        &self,
    ) -> ::std::option::Option<&crate::types::CompromisedCredentialsEventActionType> {
        self.event_action.as_ref()
    }
}
impl CompromisedCredentialsActionsType {
    /// Creates a new builder-style object to manufacture [`CompromisedCredentialsActionsType`](crate::types::CompromisedCredentialsActionsType).
    pub fn builder() -> crate::types::builders::CompromisedCredentialsActionsTypeBuilder {
        crate::types::builders::CompromisedCredentialsActionsTypeBuilder::default()
    }
}

/// A builder for [`CompromisedCredentialsActionsType`](crate::types::CompromisedCredentialsActionsType).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CompromisedCredentialsActionsTypeBuilder {
    pub(crate) event_action:
        ::std::option::Option<crate::types::CompromisedCredentialsEventActionType>,
}
impl CompromisedCredentialsActionsTypeBuilder {
    /// <p>The event action.</p>
    pub fn event_action(
        mut self,
        input: crate::types::CompromisedCredentialsEventActionType,
    ) -> Self {
        self.event_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>The event action.</p>
    pub fn set_event_action(
        mut self,
        input: ::std::option::Option<crate::types::CompromisedCredentialsEventActionType>,
    ) -> Self {
        self.event_action = input;
        self
    }
    /// Consumes the builder and constructs a [`CompromisedCredentialsActionsType`](crate::types::CompromisedCredentialsActionsType).
    pub fn build(self) -> crate::types::CompromisedCredentialsActionsType {
        crate::types::CompromisedCredentialsActionsType {
            event_action: self.event_action,
        }
    }
}
