// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListReplicationSetsInput {
    /// <p>The maximum number of results per page. </p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The pagination token to continue to the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListReplicationSetsInput {
    /// <p>The maximum number of results per page. </p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The pagination token to continue to the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListReplicationSetsInput {
    /// Creates a new builder-style object to manufacture [`ListReplicationSetsInput`](crate::operation::list_replication_sets::ListReplicationSetsInput).
    pub fn builder(
    ) -> crate::operation::list_replication_sets::builders::ListReplicationSetsInputBuilder {
        crate::operation::list_replication_sets::builders::ListReplicationSetsInputBuilder::default(
        )
    }
}

/// A builder for [`ListReplicationSetsInput`](crate::operation::list_replication_sets::ListReplicationSetsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListReplicationSetsInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListReplicationSetsInputBuilder {
    /// <p>The maximum number of results per page. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results per page. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The pagination token to continue to the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token to continue to the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListReplicationSetsInput`](crate::operation::list_replication_sets::ListReplicationSetsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_replication_sets::ListReplicationSetsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_replication_sets::ListReplicationSetsInput {
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
