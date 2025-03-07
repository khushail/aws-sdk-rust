// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The failover configuration for an endpoint. This includes what triggers failover and what happens when it's triggered.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FailoverConfig {
    /// <p>The main Region of the endpoint.</p>
    #[doc(hidden)]
    pub primary: ::std::option::Option<crate::types::Primary>,
    /// <p>The Region that events are routed to when failover is triggered or event replication is enabled.</p>
    #[doc(hidden)]
    pub secondary: ::std::option::Option<crate::types::Secondary>,
}
impl FailoverConfig {
    /// <p>The main Region of the endpoint.</p>
    pub fn primary(&self) -> ::std::option::Option<&crate::types::Primary> {
        self.primary.as_ref()
    }
    /// <p>The Region that events are routed to when failover is triggered or event replication is enabled.</p>
    pub fn secondary(&self) -> ::std::option::Option<&crate::types::Secondary> {
        self.secondary.as_ref()
    }
}
impl FailoverConfig {
    /// Creates a new builder-style object to manufacture [`FailoverConfig`](crate::types::FailoverConfig).
    pub fn builder() -> crate::types::builders::FailoverConfigBuilder {
        crate::types::builders::FailoverConfigBuilder::default()
    }
}

/// A builder for [`FailoverConfig`](crate::types::FailoverConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FailoverConfigBuilder {
    pub(crate) primary: ::std::option::Option<crate::types::Primary>,
    pub(crate) secondary: ::std::option::Option<crate::types::Secondary>,
}
impl FailoverConfigBuilder {
    /// <p>The main Region of the endpoint.</p>
    pub fn primary(mut self, input: crate::types::Primary) -> Self {
        self.primary = ::std::option::Option::Some(input);
        self
    }
    /// <p>The main Region of the endpoint.</p>
    pub fn set_primary(mut self, input: ::std::option::Option<crate::types::Primary>) -> Self {
        self.primary = input;
        self
    }
    /// <p>The Region that events are routed to when failover is triggered or event replication is enabled.</p>
    pub fn secondary(mut self, input: crate::types::Secondary) -> Self {
        self.secondary = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Region that events are routed to when failover is triggered or event replication is enabled.</p>
    pub fn set_secondary(mut self, input: ::std::option::Option<crate::types::Secondary>) -> Self {
        self.secondary = input;
        self
    }
    /// Consumes the builder and constructs a [`FailoverConfig`](crate::types::FailoverConfig).
    pub fn build(self) -> crate::types::FailoverConfig {
        crate::types::FailoverConfig {
            primary: self.primary,
            secondary: self.secondary,
        }
    }
}
