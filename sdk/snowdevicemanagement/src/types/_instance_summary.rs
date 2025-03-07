// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details about the instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceSummary {
    /// <p>A structure containing details about the instance.</p>
    #[doc(hidden)]
    pub instance: ::std::option::Option<crate::types::Instance>,
    /// <p>When the instance summary was last updated.</p>
    #[doc(hidden)]
    pub last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl InstanceSummary {
    /// <p>A structure containing details about the instance.</p>
    pub fn instance(&self) -> ::std::option::Option<&crate::types::Instance> {
        self.instance.as_ref()
    }
    /// <p>When the instance summary was last updated.</p>
    pub fn last_updated_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_at.as_ref()
    }
}
impl InstanceSummary {
    /// Creates a new builder-style object to manufacture [`InstanceSummary`](crate::types::InstanceSummary).
    pub fn builder() -> crate::types::builders::InstanceSummaryBuilder {
        crate::types::builders::InstanceSummaryBuilder::default()
    }
}

/// A builder for [`InstanceSummary`](crate::types::InstanceSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InstanceSummaryBuilder {
    pub(crate) instance: ::std::option::Option<crate::types::Instance>,
    pub(crate) last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl InstanceSummaryBuilder {
    /// <p>A structure containing details about the instance.</p>
    pub fn instance(mut self, input: crate::types::Instance) -> Self {
        self.instance = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure containing details about the instance.</p>
    pub fn set_instance(mut self, input: ::std::option::Option<crate::types::Instance>) -> Self {
        self.instance = input;
        self
    }
    /// <p>When the instance summary was last updated.</p>
    pub fn last_updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the instance summary was last updated.</p>
    pub fn set_last_updated_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_at = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceSummary`](crate::types::InstanceSummary).
    pub fn build(self) -> crate::types::InstanceSummary {
        crate::types::InstanceSummary {
            instance: self.instance,
            last_updated_at: self.last_updated_at,
        }
    }
}
