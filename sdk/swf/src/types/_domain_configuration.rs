// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the configuration settings of a domain.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DomainConfiguration {
    /// <p>The retention period for workflow executions in this domain.</p>
    #[doc(hidden)]
    pub workflow_execution_retention_period_in_days: ::std::option::Option<::std::string::String>,
}
impl DomainConfiguration {
    /// <p>The retention period for workflow executions in this domain.</p>
    pub fn workflow_execution_retention_period_in_days(&self) -> ::std::option::Option<&str> {
        self.workflow_execution_retention_period_in_days.as_deref()
    }
}
impl DomainConfiguration {
    /// Creates a new builder-style object to manufacture [`DomainConfiguration`](crate::types::DomainConfiguration).
    pub fn builder() -> crate::types::builders::DomainConfigurationBuilder {
        crate::types::builders::DomainConfigurationBuilder::default()
    }
}

/// A builder for [`DomainConfiguration`](crate::types::DomainConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DomainConfigurationBuilder {
    pub(crate) workflow_execution_retention_period_in_days:
        ::std::option::Option<::std::string::String>,
}
impl DomainConfigurationBuilder {
    /// <p>The retention period for workflow executions in this domain.</p>
    pub fn workflow_execution_retention_period_in_days(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.workflow_execution_retention_period_in_days =
            ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The retention period for workflow executions in this domain.</p>
    pub fn set_workflow_execution_retention_period_in_days(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.workflow_execution_retention_period_in_days = input;
        self
    }
    /// Consumes the builder and constructs a [`DomainConfiguration`](crate::types::DomainConfiguration).
    pub fn build(self) -> crate::types::DomainConfiguration {
        crate::types::DomainConfiguration {
            workflow_execution_retention_period_in_days: self
                .workflow_execution_retention_period_in_days,
        }
    }
}
