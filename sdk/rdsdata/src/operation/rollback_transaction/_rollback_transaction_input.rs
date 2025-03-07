// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request parameters represent the input of a request to perform a rollback of a transaction.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RollbackTransactionInput {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    #[doc(hidden)]
    pub secret_arn: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the transaction to roll back.</p>
    #[doc(hidden)]
    pub transaction_id: ::std::option::Option<::std::string::String>,
}
impl RollbackTransactionInput {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub fn secret_arn(&self) -> ::std::option::Option<&str> {
        self.secret_arn.as_deref()
    }
    /// <p>The identifier of the transaction to roll back.</p>
    pub fn transaction_id(&self) -> ::std::option::Option<&str> {
        self.transaction_id.as_deref()
    }
}
impl RollbackTransactionInput {
    /// Creates a new builder-style object to manufacture [`RollbackTransactionInput`](crate::operation::rollback_transaction::RollbackTransactionInput).
    pub fn builder(
    ) -> crate::operation::rollback_transaction::builders::RollbackTransactionInputBuilder {
        crate::operation::rollback_transaction::builders::RollbackTransactionInputBuilder::default()
    }
}

/// A builder for [`RollbackTransactionInput`](crate::operation::rollback_transaction::RollbackTransactionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RollbackTransactionInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) secret_arn: ::std::option::Option<::std::string::String>,
    pub(crate) transaction_id: ::std::option::Option<::std::string::String>,
}
impl RollbackTransactionInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub fn secret_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    pub fn set_secret_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_arn = input;
        self
    }
    /// <p>The identifier of the transaction to roll back.</p>
    pub fn transaction_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transaction_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the transaction to roll back.</p>
    pub fn set_transaction_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transaction_id = input;
        self
    }
    /// Consumes the builder and constructs a [`RollbackTransactionInput`](crate::operation::rollback_transaction::RollbackTransactionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::rollback_transaction::RollbackTransactionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::rollback_transaction::RollbackTransactionInput {
                resource_arn: self.resource_arn,
                secret_arn: self.secret_arn,
                transaction_id: self.transaction_id,
            },
        )
    }
}
