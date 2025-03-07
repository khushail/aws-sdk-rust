// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreFromRecoveryPointInput {
    /// <p>The unique identifier of the recovery point to restore from.</p>
    #[doc(hidden)]
    pub recovery_point_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the namespace to restore data into.</p>
    #[doc(hidden)]
    pub namespace_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the workgroup used to restore data.</p>
    #[doc(hidden)]
    pub workgroup_name: ::std::option::Option<::std::string::String>,
}
impl RestoreFromRecoveryPointInput {
    /// <p>The unique identifier of the recovery point to restore from.</p>
    pub fn recovery_point_id(&self) -> ::std::option::Option<&str> {
        self.recovery_point_id.as_deref()
    }
    /// <p>The name of the namespace to restore data into.</p>
    pub fn namespace_name(&self) -> ::std::option::Option<&str> {
        self.namespace_name.as_deref()
    }
    /// <p>The name of the workgroup used to restore data.</p>
    pub fn workgroup_name(&self) -> ::std::option::Option<&str> {
        self.workgroup_name.as_deref()
    }
}
impl RestoreFromRecoveryPointInput {
    /// Creates a new builder-style object to manufacture [`RestoreFromRecoveryPointInput`](crate::operation::restore_from_recovery_point::RestoreFromRecoveryPointInput).
    pub fn builder(
    ) -> crate::operation::restore_from_recovery_point::builders::RestoreFromRecoveryPointInputBuilder
    {
        crate::operation::restore_from_recovery_point::builders::RestoreFromRecoveryPointInputBuilder::default()
    }
}

/// A builder for [`RestoreFromRecoveryPointInput`](crate::operation::restore_from_recovery_point::RestoreFromRecoveryPointInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RestoreFromRecoveryPointInputBuilder {
    pub(crate) recovery_point_id: ::std::option::Option<::std::string::String>,
    pub(crate) namespace_name: ::std::option::Option<::std::string::String>,
    pub(crate) workgroup_name: ::std::option::Option<::std::string::String>,
}
impl RestoreFromRecoveryPointInputBuilder {
    /// <p>The unique identifier of the recovery point to restore from.</p>
    pub fn recovery_point_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recovery_point_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the recovery point to restore from.</p>
    pub fn set_recovery_point_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recovery_point_id = input;
        self
    }
    /// <p>The name of the namespace to restore data into.</p>
    pub fn namespace_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.namespace_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the namespace to restore data into.</p>
    pub fn set_namespace_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.namespace_name = input;
        self
    }
    /// <p>The name of the workgroup used to restore data.</p>
    pub fn workgroup_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.workgroup_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the workgroup used to restore data.</p>
    pub fn set_workgroup_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.workgroup_name = input;
        self
    }
    /// Consumes the builder and constructs a [`RestoreFromRecoveryPointInput`](crate::operation::restore_from_recovery_point::RestoreFromRecoveryPointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_from_recovery_point::RestoreFromRecoveryPointInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::restore_from_recovery_point::RestoreFromRecoveryPointInput {
                recovery_point_id: self.recovery_point_id,
                namespace_name: self.namespace_name,
                workgroup_name: self.workgroup_name,
            },
        )
    }
}
