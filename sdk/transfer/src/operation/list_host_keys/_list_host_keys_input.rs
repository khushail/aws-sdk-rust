// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListHostKeysInput {
    /// <p>The maximum number of host keys to return.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>When there are additional results that were not returned, a <code>NextToken</code> parameter is returned. You can use that value for a subsequent call to <code>ListHostKeys</code> to continue listing results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the server that contains the host keys that you want to view.</p>
    #[doc(hidden)]
    pub server_id: ::std::option::Option<::std::string::String>,
}
impl ListHostKeysInput {
    /// <p>The maximum number of host keys to return.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>When there are additional results that were not returned, a <code>NextToken</code> parameter is returned. You can use that value for a subsequent call to <code>ListHostKeys</code> to continue listing results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The identifier of the server that contains the host keys that you want to view.</p>
    pub fn server_id(&self) -> ::std::option::Option<&str> {
        self.server_id.as_deref()
    }
}
impl ListHostKeysInput {
    /// Creates a new builder-style object to manufacture [`ListHostKeysInput`](crate::operation::list_host_keys::ListHostKeysInput).
    pub fn builder() -> crate::operation::list_host_keys::builders::ListHostKeysInputBuilder {
        crate::operation::list_host_keys::builders::ListHostKeysInputBuilder::default()
    }
}

/// A builder for [`ListHostKeysInput`](crate::operation::list_host_keys::ListHostKeysInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListHostKeysInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) server_id: ::std::option::Option<::std::string::String>,
}
impl ListHostKeysInputBuilder {
    /// <p>The maximum number of host keys to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of host keys to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>When there are additional results that were not returned, a <code>NextToken</code> parameter is returned. You can use that value for a subsequent call to <code>ListHostKeys</code> to continue listing results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When there are additional results that were not returned, a <code>NextToken</code> parameter is returned. You can use that value for a subsequent call to <code>ListHostKeys</code> to continue listing results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The identifier of the server that contains the host keys that you want to view.</p>
    pub fn server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.server_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the server that contains the host keys that you want to view.</p>
    pub fn set_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.server_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ListHostKeysInput`](crate::operation::list_host_keys::ListHostKeysInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_host_keys::ListHostKeysInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_host_keys::ListHostKeysInput {
            max_results: self.max_results,
            next_token: self.next_token,
            server_id: self.server_id,
        })
    }
}
