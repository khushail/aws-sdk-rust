// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the resource you're deleting the policy from.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the resource policy you're deleting.</p>
    #[doc(hidden)]
    pub policy_id: ::std::option::Option<::std::string::String>,
}
impl DeleteResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the resource you're deleting the policy from.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>The ID of the resource policy you're deleting.</p>
    pub fn policy_id(&self) -> ::std::option::Option<&str> {
        self.policy_id.as_deref()
    }
}
impl DeleteResourcePolicyInput {
    /// Creates a new builder-style object to manufacture [`DeleteResourcePolicyInput`](crate::operation::delete_resource_policy::DeleteResourcePolicyInput).
    pub fn builder(
    ) -> crate::operation::delete_resource_policy::builders::DeleteResourcePolicyInputBuilder {
        crate::operation::delete_resource_policy::builders::DeleteResourcePolicyInputBuilder::default()
    }
}

/// A builder for [`DeleteResourcePolicyInput`](crate::operation::delete_resource_policy::DeleteResourcePolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteResourcePolicyInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) policy_id: ::std::option::Option<::std::string::String>,
}
impl DeleteResourcePolicyInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the resource you're deleting the policy from.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource you're deleting the policy from.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The ID of the resource policy you're deleting.</p>
    pub fn policy_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource policy you're deleting.</p>
    pub fn set_policy_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteResourcePolicyInput`](crate::operation::delete_resource_policy::DeleteResourcePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_resource_policy::DeleteResourcePolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_resource_policy::DeleteResourcePolicyInput {
                resource_arn: self.resource_arn,
                policy_id: self.policy_id,
            },
        )
    }
}
