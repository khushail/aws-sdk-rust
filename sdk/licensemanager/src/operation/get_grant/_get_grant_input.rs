// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetGrantInput {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[doc(hidden)]
    pub grant_arn: ::std::option::Option<::std::string::String>,
    /// <p>Grant version.</p>
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
}
impl GetGrantInput {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    pub fn grant_arn(&self) -> ::std::option::Option<&str> {
        self.grant_arn.as_deref()
    }
    /// <p>Grant version.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl GetGrantInput {
    /// Creates a new builder-style object to manufacture [`GetGrantInput`](crate::operation::get_grant::GetGrantInput).
    pub fn builder() -> crate::operation::get_grant::builders::GetGrantInputBuilder {
        crate::operation::get_grant::builders::GetGrantInputBuilder::default()
    }
}

/// A builder for [`GetGrantInput`](crate::operation::get_grant::GetGrantInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetGrantInputBuilder {
    pub(crate) grant_arn: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl GetGrantInputBuilder {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    pub fn grant_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.grant_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    pub fn set_grant_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.grant_arn = input;
        self
    }
    /// <p>Grant version.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Grant version.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// Consumes the builder and constructs a [`GetGrantInput`](crate::operation::get_grant::GetGrantInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_grant::GetGrantInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_grant::GetGrantInput {
            grant_arn: self.grant_arn,
            version: self.version,
        })
    }
}
