// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetConfiguredTableAssociationInput {
    /// <p>The unique ID for the configured table association to retrieve. Currently accepts the configured table ID.</p>
    #[doc(hidden)]
    pub configured_table_association_identifier: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the membership that the configured table association belongs to. Currently accepts the membership ID.</p>
    #[doc(hidden)]
    pub membership_identifier: ::std::option::Option<::std::string::String>,
}
impl GetConfiguredTableAssociationInput {
    /// <p>The unique ID for the configured table association to retrieve. Currently accepts the configured table ID.</p>
    pub fn configured_table_association_identifier(&self) -> ::std::option::Option<&str> {
        self.configured_table_association_identifier.as_deref()
    }
    /// <p>A unique identifier for the membership that the configured table association belongs to. Currently accepts the membership ID.</p>
    pub fn membership_identifier(&self) -> ::std::option::Option<&str> {
        self.membership_identifier.as_deref()
    }
}
impl GetConfiguredTableAssociationInput {
    /// Creates a new builder-style object to manufacture [`GetConfiguredTableAssociationInput`](crate::operation::get_configured_table_association::GetConfiguredTableAssociationInput).
    pub fn builder() -> crate::operation::get_configured_table_association::builders::GetConfiguredTableAssociationInputBuilder{
        crate::operation::get_configured_table_association::builders::GetConfiguredTableAssociationInputBuilder::default()
    }
}

/// A builder for [`GetConfiguredTableAssociationInput`](crate::operation::get_configured_table_association::GetConfiguredTableAssociationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetConfiguredTableAssociationInputBuilder {
    pub(crate) configured_table_association_identifier:
        ::std::option::Option<::std::string::String>,
    pub(crate) membership_identifier: ::std::option::Option<::std::string::String>,
}
impl GetConfiguredTableAssociationInputBuilder {
    /// <p>The unique ID for the configured table association to retrieve. Currently accepts the configured table ID.</p>
    pub fn configured_table_association_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configured_table_association_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID for the configured table association to retrieve. Currently accepts the configured table ID.</p>
    pub fn set_configured_table_association_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configured_table_association_identifier = input;
        self
    }
    /// <p>A unique identifier for the membership that the configured table association belongs to. Currently accepts the membership ID.</p>
    pub fn membership_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.membership_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the membership that the configured table association belongs to. Currently accepts the membership ID.</p>
    pub fn set_membership_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.membership_identifier = input;
        self
    }
    /// Consumes the builder and constructs a [`GetConfiguredTableAssociationInput`](crate::operation::get_configured_table_association::GetConfiguredTableAssociationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_configured_table_association::GetConfiguredTableAssociationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_configured_table_association::GetConfiguredTableAssociationInput {
                configured_table_association_identifier: self.configured_table_association_identifier
                ,
                membership_identifier: self.membership_identifier
                ,
            }
        )
    }
}
