// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeOrganizationHealthInput {
    /// <p>The ID of the Amazon Web Services account.</p>
    #[doc(hidden)]
    pub account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The ID of the organizational unit.</p>
    #[doc(hidden)]
    pub organizational_unit_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeOrganizationHealthInput {
    /// <p>The ID of the Amazon Web Services account.</p>
    pub fn account_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.account_ids.as_deref()
    }
    /// <p>The ID of the organizational unit.</p>
    pub fn organizational_unit_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.organizational_unit_ids.as_deref()
    }
}
impl DescribeOrganizationHealthInput {
    /// Creates a new builder-style object to manufacture [`DescribeOrganizationHealthInput`](crate::operation::describe_organization_health::DescribeOrganizationHealthInput).
    pub fn builder() -> crate::operation::describe_organization_health::builders::DescribeOrganizationHealthInputBuilder{
        crate::operation::describe_organization_health::builders::DescribeOrganizationHealthInputBuilder::default()
    }
}

/// A builder for [`DescribeOrganizationHealthInput`](crate::operation::describe_organization_health::DescribeOrganizationHealthInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeOrganizationHealthInputBuilder {
    pub(crate) account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) organizational_unit_ids:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeOrganizationHealthInputBuilder {
    /// Appends an item to `account_ids`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>The ID of the Amazon Web Services account.</p>
    pub fn account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.account_ids.unwrap_or_default();
        v.push(input.into());
        self.account_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ID of the Amazon Web Services account.</p>
    pub fn set_account_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.account_ids = input;
        self
    }
    /// Appends an item to `organizational_unit_ids`.
    ///
    /// To override the contents of this collection use [`set_organizational_unit_ids`](Self::set_organizational_unit_ids).
    ///
    /// <p>The ID of the organizational unit.</p>
    pub fn organizational_unit_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.organizational_unit_ids.unwrap_or_default();
        v.push(input.into());
        self.organizational_unit_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ID of the organizational unit.</p>
    pub fn set_organizational_unit_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.organizational_unit_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeOrganizationHealthInput`](crate::operation::describe_organization_health::DescribeOrganizationHealthInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_organization_health::DescribeOrganizationHealthInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_organization_health::DescribeOrganizationHealthInput {
                account_ids: self.account_ids,
                organizational_unit_ids: self.organizational_unit_ids,
            },
        )
    }
}
