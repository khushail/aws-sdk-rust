// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTokenInput {
    /// <p>Token ID.</p>
    #[doc(hidden)]
    pub token_id: ::std::option::Option<::std::string::String>,
}
impl DeleteTokenInput {
    /// <p>Token ID.</p>
    pub fn token_id(&self) -> ::std::option::Option<&str> {
        self.token_id.as_deref()
    }
}
impl DeleteTokenInput {
    /// Creates a new builder-style object to manufacture [`DeleteTokenInput`](crate::operation::delete_token::DeleteTokenInput).
    pub fn builder() -> crate::operation::delete_token::builders::DeleteTokenInputBuilder {
        crate::operation::delete_token::builders::DeleteTokenInputBuilder::default()
    }
}

/// A builder for [`DeleteTokenInput`](crate::operation::delete_token::DeleteTokenInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteTokenInputBuilder {
    pub(crate) token_id: ::std::option::Option<::std::string::String>,
}
impl DeleteTokenInputBuilder {
    /// <p>Token ID.</p>
    pub fn token_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.token_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Token ID.</p>
    pub fn set_token_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.token_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTokenInput`](crate::operation::delete_token::DeleteTokenInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_token::DeleteTokenInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_token::DeleteTokenInput {
            token_id: self.token_id,
        })
    }
}
