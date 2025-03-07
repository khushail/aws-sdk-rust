// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input value for <code>ListTagsForVaultInput</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsForVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the vault.</p>
    #[doc(hidden)]
    pub vault_name: ::std::option::Option<::std::string::String>,
}
impl ListTagsForVaultInput {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(&self) -> ::std::option::Option<&str> {
        self.vault_name.as_deref()
    }
}
impl ListTagsForVaultInput {
    /// Creates a new builder-style object to manufacture [`ListTagsForVaultInput`](crate::operation::list_tags_for_vault::ListTagsForVaultInput).
    pub fn builder() -> crate::operation::list_tags_for_vault::builders::ListTagsForVaultInputBuilder
    {
        crate::operation::list_tags_for_vault::builders::ListTagsForVaultInputBuilder::default()
    }
}

/// A builder for [`ListTagsForVaultInput`](crate::operation::list_tags_for_vault::ListTagsForVaultInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTagsForVaultInputBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) vault_name: ::std::option::Option<::std::string::String>,
}
impl ListTagsForVaultInputBuilder {
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vault_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vault_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ListTagsForVaultInput`](crate::operation::list_tags_for_vault::ListTagsForVaultInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_tags_for_vault::ListTagsForVaultInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_tags_for_vault::ListTagsForVaultInput {
                account_id: self.account_id,
                vault_name: self.vault_name,
            },
        )
    }
}
