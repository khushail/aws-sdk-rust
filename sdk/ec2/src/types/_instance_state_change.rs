// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an instance state change.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceStateChange {
    /// <p>The current state of the instance.</p>
    #[doc(hidden)]
    pub current_state: ::std::option::Option<crate::types::InstanceState>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The previous state of the instance.</p>
    #[doc(hidden)]
    pub previous_state: ::std::option::Option<crate::types::InstanceState>,
}
impl InstanceStateChange {
    /// <p>The current state of the instance.</p>
    pub fn current_state(&self) -> ::std::option::Option<&crate::types::InstanceState> {
        self.current_state.as_ref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The previous state of the instance.</p>
    pub fn previous_state(&self) -> ::std::option::Option<&crate::types::InstanceState> {
        self.previous_state.as_ref()
    }
}
impl InstanceStateChange {
    /// Creates a new builder-style object to manufacture [`InstanceStateChange`](crate::types::InstanceStateChange).
    pub fn builder() -> crate::types::builders::InstanceStateChangeBuilder {
        crate::types::builders::InstanceStateChangeBuilder::default()
    }
}

/// A builder for [`InstanceStateChange`](crate::types::InstanceStateChange).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InstanceStateChangeBuilder {
    pub(crate) current_state: ::std::option::Option<crate::types::InstanceState>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) previous_state: ::std::option::Option<crate::types::InstanceState>,
}
impl InstanceStateChangeBuilder {
    /// <p>The current state of the instance.</p>
    pub fn current_state(mut self, input: crate::types::InstanceState) -> Self {
        self.current_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the instance.</p>
    pub fn set_current_state(
        mut self,
        input: ::std::option::Option<crate::types::InstanceState>,
    ) -> Self {
        self.current_state = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The previous state of the instance.</p>
    pub fn previous_state(mut self, input: crate::types::InstanceState) -> Self {
        self.previous_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The previous state of the instance.</p>
    pub fn set_previous_state(
        mut self,
        input: ::std::option::Option<crate::types::InstanceState>,
    ) -> Self {
        self.previous_state = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceStateChange`](crate::types::InstanceStateChange).
    pub fn build(self) -> crate::types::InstanceStateChange {
        crate::types::InstanceStateChange {
            current_state: self.current_state,
            instance_id: self.instance_id,
            previous_state: self.previous_state,
        }
    }
}
