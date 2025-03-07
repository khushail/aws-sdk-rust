// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAccessPolicyInput {
    /// <p>The ID of the access policy.</p>
    #[doc(hidden)]
    pub access_policy_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAccessPolicyInput {
    /// <p>The ID of the access policy.</p>
    pub fn access_policy_id(&self) -> ::std::option::Option<&str> {
        self.access_policy_id.as_deref()
    }
}
impl DescribeAccessPolicyInput {
    /// Creates a new builder-style object to manufacture [`DescribeAccessPolicyInput`](crate::operation::describe_access_policy::DescribeAccessPolicyInput).
    pub fn builder(
    ) -> crate::operation::describe_access_policy::builders::DescribeAccessPolicyInputBuilder {
        crate::operation::describe_access_policy::builders::DescribeAccessPolicyInputBuilder::default()
    }
}

/// A builder for [`DescribeAccessPolicyInput`](crate::operation::describe_access_policy::DescribeAccessPolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAccessPolicyInputBuilder {
    pub(crate) access_policy_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAccessPolicyInputBuilder {
    /// <p>The ID of the access policy.</p>
    pub fn access_policy_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.access_policy_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the access policy.</p>
    pub fn set_access_policy_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.access_policy_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAccessPolicyInput`](crate::operation::describe_access_policy::DescribeAccessPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_access_policy::DescribeAccessPolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_access_policy::DescribeAccessPolicyInput {
                access_policy_id: self.access_policy_id,
            },
        )
    }
}
