// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifySelfservicePermissionsInput {
    /// <p>The identifier of the directory.</p>
    #[doc(hidden)]
    pub resource_id: ::std::option::Option<::std::string::String>,
    /// <p>The permissions to enable or disable self-service capabilities.</p>
    #[doc(hidden)]
    pub selfservice_permissions: ::std::option::Option<crate::types::SelfservicePermissions>,
}
impl ModifySelfservicePermissionsInput {
    /// <p>The identifier of the directory.</p>
    pub fn resource_id(&self) -> ::std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The permissions to enable or disable self-service capabilities.</p>
    pub fn selfservice_permissions(
        &self,
    ) -> ::std::option::Option<&crate::types::SelfservicePermissions> {
        self.selfservice_permissions.as_ref()
    }
}
impl ModifySelfservicePermissionsInput {
    /// Creates a new builder-style object to manufacture [`ModifySelfservicePermissionsInput`](crate::operation::modify_selfservice_permissions::ModifySelfservicePermissionsInput).
    pub fn builder() -> crate::operation::modify_selfservice_permissions::builders::ModifySelfservicePermissionsInputBuilder{
        crate::operation::modify_selfservice_permissions::builders::ModifySelfservicePermissionsInputBuilder::default()
    }
}

/// A builder for [`ModifySelfservicePermissionsInput`](crate::operation::modify_selfservice_permissions::ModifySelfservicePermissionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ModifySelfservicePermissionsInputBuilder {
    pub(crate) resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) selfservice_permissions: ::std::option::Option<crate::types::SelfservicePermissions>,
}
impl ModifySelfservicePermissionsInputBuilder {
    /// <p>The identifier of the directory.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The permissions to enable or disable self-service capabilities.</p>
    pub fn selfservice_permissions(mut self, input: crate::types::SelfservicePermissions) -> Self {
        self.selfservice_permissions = ::std::option::Option::Some(input);
        self
    }
    /// <p>The permissions to enable or disable self-service capabilities.</p>
    pub fn set_selfservice_permissions(
        mut self,
        input: ::std::option::Option<crate::types::SelfservicePermissions>,
    ) -> Self {
        self.selfservice_permissions = input;
        self
    }
    /// Consumes the builder and constructs a [`ModifySelfservicePermissionsInput`](crate::operation::modify_selfservice_permissions::ModifySelfservicePermissionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_selfservice_permissions::ModifySelfservicePermissionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::modify_selfservice_permissions::ModifySelfservicePermissionsInput {
                resource_id: self.resource_id,
                selfservice_permissions: self.selfservice_permissions,
            },
        )
    }
}
