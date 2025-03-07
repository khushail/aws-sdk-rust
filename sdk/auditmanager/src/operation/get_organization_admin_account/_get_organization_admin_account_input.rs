// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetOrganizationAdminAccountInput {}
impl GetOrganizationAdminAccountInput {
    /// Creates a new builder-style object to manufacture [`GetOrganizationAdminAccountInput`](crate::operation::get_organization_admin_account::GetOrganizationAdminAccountInput).
    pub fn builder() -> crate::operation::get_organization_admin_account::builders::GetOrganizationAdminAccountInputBuilder{
        crate::operation::get_organization_admin_account::builders::GetOrganizationAdminAccountInputBuilder::default()
    }
}

/// A builder for [`GetOrganizationAdminAccountInput`](crate::operation::get_organization_admin_account::GetOrganizationAdminAccountInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetOrganizationAdminAccountInputBuilder {}
impl GetOrganizationAdminAccountInputBuilder {
    /// Consumes the builder and constructs a [`GetOrganizationAdminAccountInput`](crate::operation::get_organization_admin_account::GetOrganizationAdminAccountInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_organization_admin_account::GetOrganizationAdminAccountInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_organization_admin_account::GetOrganizationAdminAccountInput {},
        )
    }
}
