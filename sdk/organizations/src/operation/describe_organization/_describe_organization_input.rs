// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeOrganizationInput {}
impl DescribeOrganizationInput {
    /// Creates a new builder-style object to manufacture [`DescribeOrganizationInput`](crate::operation::describe_organization::DescribeOrganizationInput).
    pub fn builder(
    ) -> crate::operation::describe_organization::builders::DescribeOrganizationInputBuilder {
        crate::operation::describe_organization::builders::DescribeOrganizationInputBuilder::default(
        )
    }
}

/// A builder for [`DescribeOrganizationInput`](crate::operation::describe_organization::DescribeOrganizationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeOrganizationInputBuilder {}
impl DescribeOrganizationInputBuilder {
    /// Consumes the builder and constructs a [`DescribeOrganizationInput`](crate::operation::describe_organization::DescribeOrganizationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_organization::DescribeOrganizationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_organization::DescribeOrganizationInput {},
        )
    }
}
