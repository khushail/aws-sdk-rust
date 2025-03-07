// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetUserPoolMfaConfigInput {
    /// <p>The user pool ID.</p>
    #[doc(hidden)]
    pub user_pool_id: ::std::option::Option<::std::string::String>,
}
impl GetUserPoolMfaConfigInput {
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
}
impl GetUserPoolMfaConfigInput {
    /// Creates a new builder-style object to manufacture [`GetUserPoolMfaConfigInput`](crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigInput).
    pub fn builder(
    ) -> crate::operation::get_user_pool_mfa_config::builders::GetUserPoolMfaConfigInputBuilder
    {
        crate::operation::get_user_pool_mfa_config::builders::GetUserPoolMfaConfigInputBuilder::default()
    }
}

/// A builder for [`GetUserPoolMfaConfigInput`](crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetUserPoolMfaConfigInputBuilder {
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
}
impl GetUserPoolMfaConfigInputBuilder {
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user pool ID.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetUserPoolMfaConfigInput`](crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigInput {
                user_pool_id: self.user_pool_id,
            },
        )
    }
}
