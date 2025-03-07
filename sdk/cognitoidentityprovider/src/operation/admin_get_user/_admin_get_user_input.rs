// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the request to get the specified user as an administrator.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AdminGetUserInput {
    /// <p>The user pool ID for the user pool where you want to get information about the user.</p>
    #[doc(hidden)]
    pub user_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The user name of the user you want to retrieve.</p>
    #[doc(hidden)]
    pub username: ::std::option::Option<::std::string::String>,
}
impl AdminGetUserInput {
    /// <p>The user pool ID for the user pool where you want to get information about the user.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
    /// <p>The user name of the user you want to retrieve.</p>
    pub fn username(&self) -> ::std::option::Option<&str> {
        self.username.as_deref()
    }
}
impl ::std::fmt::Debug for AdminGetUserInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminGetUserInput");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl AdminGetUserInput {
    /// Creates a new builder-style object to manufacture [`AdminGetUserInput`](crate::operation::admin_get_user::AdminGetUserInput).
    pub fn builder() -> crate::operation::admin_get_user::builders::AdminGetUserInputBuilder {
        crate::operation::admin_get_user::builders::AdminGetUserInputBuilder::default()
    }
}

/// A builder for [`AdminGetUserInput`](crate::operation::admin_get_user::AdminGetUserInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AdminGetUserInputBuilder {
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) username: ::std::option::Option<::std::string::String>,
}
impl AdminGetUserInputBuilder {
    /// <p>The user pool ID for the user pool where you want to get information about the user.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user pool ID for the user pool where you want to get information about the user.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// <p>The user name of the user you want to retrieve.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user name of the user you want to retrieve.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.username = input;
        self
    }
    /// Consumes the builder and constructs a [`AdminGetUserInput`](crate::operation::admin_get_user::AdminGetUserInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::admin_get_user::AdminGetUserInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::admin_get_user::AdminGetUserInput {
            user_pool_id: self.user_pool_id,
            username: self.username,
        })
    }
}
impl ::std::fmt::Debug for AdminGetUserInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminGetUserInputBuilder");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
