// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateConnectionFromLagInput {
    /// <p>The ID of the connection.</p>
    #[doc(hidden)]
    pub connection_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the LAG.</p>
    #[doc(hidden)]
    pub lag_id: ::std::option::Option<::std::string::String>,
}
impl DisassociateConnectionFromLagInput {
    /// <p>The ID of the connection.</p>
    pub fn connection_id(&self) -> ::std::option::Option<&str> {
        self.connection_id.as_deref()
    }
    /// <p>The ID of the LAG.</p>
    pub fn lag_id(&self) -> ::std::option::Option<&str> {
        self.lag_id.as_deref()
    }
}
impl DisassociateConnectionFromLagInput {
    /// Creates a new builder-style object to manufacture [`DisassociateConnectionFromLagInput`](crate::operation::disassociate_connection_from_lag::DisassociateConnectionFromLagInput).
    pub fn builder() -> crate::operation::disassociate_connection_from_lag::builders::DisassociateConnectionFromLagInputBuilder{
        crate::operation::disassociate_connection_from_lag::builders::DisassociateConnectionFromLagInputBuilder::default()
    }
}

/// A builder for [`DisassociateConnectionFromLagInput`](crate::operation::disassociate_connection_from_lag::DisassociateConnectionFromLagInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociateConnectionFromLagInputBuilder {
    pub(crate) connection_id: ::std::option::Option<::std::string::String>,
    pub(crate) lag_id: ::std::option::Option<::std::string::String>,
}
impl DisassociateConnectionFromLagInputBuilder {
    /// <p>The ID of the connection.</p>
    pub fn connection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the connection.</p>
    pub fn set_connection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.connection_id = input;
        self
    }
    /// <p>The ID of the LAG.</p>
    pub fn lag_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.lag_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the LAG.</p>
    pub fn set_lag_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.lag_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateConnectionFromLagInput`](crate::operation::disassociate_connection_from_lag::DisassociateConnectionFromLagInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_connection_from_lag::DisassociateConnectionFromLagInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::disassociate_connection_from_lag::DisassociateConnectionFromLagInput {
                connection_id: self.connection_id
                ,
                lag_id: self.lag_id
                ,
            }
        )
    }
}
