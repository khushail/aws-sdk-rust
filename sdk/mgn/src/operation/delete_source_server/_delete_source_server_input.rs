// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteSourceServerInput {
    /// <p>Request to delete Source Server from service by Server ID.</p>
    #[doc(hidden)]
    pub source_server_id: ::std::option::Option<::std::string::String>,
}
impl DeleteSourceServerInput {
    /// <p>Request to delete Source Server from service by Server ID.</p>
    pub fn source_server_id(&self) -> ::std::option::Option<&str> {
        self.source_server_id.as_deref()
    }
}
impl DeleteSourceServerInput {
    /// Creates a new builder-style object to manufacture [`DeleteSourceServerInput`](crate::operation::delete_source_server::DeleteSourceServerInput).
    pub fn builder(
    ) -> crate::operation::delete_source_server::builders::DeleteSourceServerInputBuilder {
        crate::operation::delete_source_server::builders::DeleteSourceServerInputBuilder::default()
    }
}

/// A builder for [`DeleteSourceServerInput`](crate::operation::delete_source_server::DeleteSourceServerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteSourceServerInputBuilder {
    pub(crate) source_server_id: ::std::option::Option<::std::string::String>,
}
impl DeleteSourceServerInputBuilder {
    /// <p>Request to delete Source Server from service by Server ID.</p>
    pub fn source_server_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_server_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Request to delete Source Server from service by Server ID.</p>
    pub fn set_source_server_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_server_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteSourceServerInput`](crate::operation::delete_source_server::DeleteSourceServerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_source_server::DeleteSourceServerInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_source_server::DeleteSourceServerInput {
                source_server_id: self.source_server_id,
            },
        )
    }
}
