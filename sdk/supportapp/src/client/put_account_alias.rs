// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAccountAlias`](crate::operation::put_account_alias::builders::PutAccountAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_alias(impl ::std::convert::Into<String>)`](crate::operation::put_account_alias::builders::PutAccountAliasFluentBuilder::account_alias) / [`set_account_alias(Option<String>)`](crate::operation::put_account_alias::builders::PutAccountAliasFluentBuilder::set_account_alias): <p>An alias or short name for an Amazon Web Services account.</p>
    /// - On success, responds with [`PutAccountAliasOutput`](crate::operation::put_account_alias::PutAccountAliasOutput)
    /// - On failure, responds with [`SdkError<PutAccountAliasError>`](crate::operation::put_account_alias::PutAccountAliasError)
    pub fn put_account_alias(
        &self,
    ) -> crate::operation::put_account_alias::builders::PutAccountAliasFluentBuilder {
        crate::operation::put_account_alias::builders::PutAccountAliasFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
